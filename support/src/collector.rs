use super::Result;

use std::path::{Path, PathBuf};

use marky_mark::Benchmark;
use noisy_float::prelude::*;
use serde_json;

use run_plan::RunPlan;
use storage::{index, measurement, Estimates, Statistic, StorageKey};
use toolchain::Toolchain;
use CriterionConfig;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Error {
    kind: ErrorKind,
    num_retries: u8,
    max_retries: u8,
    retryable: bool,
}

const DEFAULT_MAX_RETRIES: u8 = 2;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) enum ErrorKind {
    Run(String),
    PostProcess(String),
}

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

        let get_estimates = || -> ::std::result::Result<_, Error> {
            rp.exec().map_err(|why| Error {
                kind: ErrorKind::Run(why.to_string()),
                max_retries: DEFAULT_MAX_RETRIES,
                num_retries: 0,
                retryable: false,
            })?;

            self.process(&rp).map_err(|why| Error {
                kind: ErrorKind::Run(why.to_string()),
                max_retries: DEFAULT_MAX_RETRIES,
                num_retries: 0,
                retryable: false,
            })
        };

        let (should_write_measure, estimates) = if let Some((_ts, r)) = mkey.get(&self.0)? {
            let e = match r {
                Ok(e) => Ok(e),
                Err(Error {
                    retryable: _retryable,
                    num_retries: _num_retries,
                    max_retries: _max_retries,
                    kind: _kind,
                }) => {
                    unimplemented!();
                }
            };
            (false, e)
        } else {
            (true, get_estimates())
        };

        info!("finished running and parsing estimates");

        if should_write_idx {
            info!("index storage needs updating");
            ikey.set(&self.0, binary_hash)?;
        }

        if should_write_measure {
            info!("measurement storage needs updating");
            mkey.set(&self.0, estimates)?;
        }

        // TODO git commit/push operations go here

        info!("all done with {}", rp);
        Ok(())
    }

    /// Parses the results of a benchmark. This assumes that the benchmark has already been
    /// executed.
    fn process(&self, rp: &RunPlan) -> Result<Estimates> {
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
    source_path: &Path,
) {
    let _ = ::simple_logger::init();

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
            name: String::from(bench_name),
            crate_name: String::from(crate_name),
            entrypoint_path: source_path.to_owned(),
        },
        binary_name: binary_name.to_owned(),
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
