use super::Result;

use std::path::{Path, PathBuf};

use marky_mark::Benchmark;
use noisy_float::prelude::*;
use serde_json;

use run_plan::RunPlan;
use storage::{index, measurement, Estimates, Statistic, StorageKey};
use toolchain::Toolchain;
use CriterionConfig;

/// Runs benchmarks, memoizes their results, and allows results to be shared across multiple
/// toolchains if the binaries they produce are identical.
pub struct Collector(PathBuf);

impl Collector {
    pub fn rehydrate(data_dir: &Path) -> Result<Self> {
        ::std::fs::create_dir_all(data_dir)?;
        Ok(Collector(data_dir.to_path_buf()))
    }

    /// Run a planned benchmark from before it has been built through to storing its results in
    /// the data directory.
    ///
    /// As optimizations, this may not actually build the binary or run the benchmarks if the data
    /// directory already has their respsective outputs for the provided RunPlan.
    ///
    /// Assumes that the `RunPlan`'s toolchain has already been installed.
    pub fn run(&self, rp: RunPlan) -> Result<()> {
        let ikey = index::Key::new(rp.benchmark.key(), rp.toolchain.clone());

        // TODO git cleanliness operations go here

        let (should_write_idx, binary_hash) = if let Some((_ts, h)) = ikey.get(&self.0)? {
            (false, h)
        } else {
            // we don't already have an index entry, so we need to build the binary to create one
            (true, rp.build()?)
        };

        let mkey = measurement::Key::new(
            binary_hash.clone(),
            // TODO make sure the benchmark's runner isn't optional by here?
            rp.benchmark.runner.clone().unwrap(),
            rp.shield.clone(),
        );

        let (should_write_measure, estimates) = if let Some((_ts, e)) = mkey.get(&self.0)? {
            (false, e)
        } else {
            rp.exec()?;
            (true, self.process(&rp)?)
        };

        if should_write_idx {
            ikey.set(&self.0, binary_hash)?;
        }

        if should_write_measure {
            mkey.set(&self.0, estimates)?;
        }

        // TODO git commit/push operations go here

        Ok(())
    }

    /// Parses the results of a benchmark. This assumes that the benchmark has already been
    /// executed.
    pub fn process(&self, rp: &RunPlan) -> Result<Estimates> {
        info!("post-processing {}", rp);

        let path = rp
            .toolchain
            .target_dir()
            .join("criterion")
            .join(format!(
                "{}::{}",
                &rp.benchmark.crate_name, &rp.benchmark.name
            ))
            .join("new");

        info!("postprocessing");

        let runtime_estimates_path = path.join("estimates.json");

        debug!(
            "reading runtime estimates from disk @ {}",
            runtime_estimates_path.display()
        );
        let runtime_estimates_json = ::std::fs::read_to_string(runtime_estimates_path)?;

        debug!("parsing runtime estimates");
        let runtime_estimates: Statistic = serde_json::from_str(&runtime_estimates_json)?;

        let mut metrics_estimates = Estimates::new();

        metrics_estimates.insert(String::from("nanoseconds"), runtime_estimates);

        let metrics_estimates_path = path.join("metrics-estimates.json");
        debug!("reading metrics estimates from disk");
        if let Ok(metrics_estimates_json) = ::std::fs::read_to_string(metrics_estimates_path) {
            debug!("parsing metrics estimates");
            let estimates: Estimates = serde_json::from_str(&metrics_estimates_json)?;
            metrics_estimates.extend(estimates);
        } else {
            warn!("couldn't read metrics-estimates.json for {}", rp);
        }

        Ok(metrics_estimates)
    }
}

pub fn end_to_end_test(
    crate_name: &str,
    bench_name: &str,
    bench_source_name: &str,
    binary_name: &str,
) {
    let _ = ::simple_logger::init();

    let target_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target");
    let binary_path = target_dir.join("release").join(binary_name);

    let plan = RunPlan {
        shield: None,
        toolchain: Toolchain::from("stable"),
        source_path: Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("benches")
            .join(crate_name)
            .join("src")
            .join("bin")
            .join(bench_source_name),
        manifest_path: Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("benches")
            .join(crate_name)
            .join("Cargo.toml"),
        benchmark: Benchmark {
            runner: None,
            runtime_estimate: None,
            name: String::from(bench_name),
            crate_name: String::from(crate_name),
            entrypoint_path: binary_path.clone(),
        },
        binary_path,
        bench_config: Some(CriterionConfig {
            confidence_level: r32(0.95),
            measurement_time_ms: 500,
            nresamples: 2,
            noise_threshold: r32(0.0),
            sample_size: 5,
            significance_level: r32(0.05),
            warm_up_time_ms: 1,
        }),
    };

    // FIXME make this a proper temp dir
    let collector = Collector::rehydrate(Path::new("/tmp/lolbenchtest")).unwrap();
    collector.run(plan).unwrap();
}
