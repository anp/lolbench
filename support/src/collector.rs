use super::Result;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use marky_mark::Benchmark;
use noisy_float::prelude::*;
// use serde::{de::DeserializeOwned, Serialize};
use serde_json;

use run_plan::RunPlan;
use storage::{index, measurement, Entry, Estimates, Statistic, StorageKey};
use toolchain::Toolchain;
use {BenchOpts, CriterionConfig};

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
    pub fn new(data_dir: &Path) -> Result<Self> {
        ::std::fs::create_dir_all(data_dir)?;
        Ok(Collector(data_dir.to_path_buf()))
    }

    pub fn run_all(&mut self, candidates: BTreeMap<Toolchain, BTreeSet<RunPlan>>) -> Result<()> {
        // for (toolchain, benchmarks) in self.enumerate_builds(opts)? {
        //     // TODO pass over all of these and see which ones actually need to be built
        //     toolchain.install()?;

        //     for plan in benchmarks {
        //         self.run(plan)?;
        //     }
        // }

        Ok(())
    }

    pub fn enumerate_builds_for_opts(
        &self,
        opts: BenchOpts,
    ) -> Result<BTreeMap<Toolchain, BTreeSet<RunPlan>>> {
        let benchmarks = ::registry::get_benches(opts.runner.as_ref().map(String::as_str))?;
        let toolchains = opts.toolchains.all_of_em();

        let mut plans = BTreeMap::new();

        for toolchain in toolchains {
            let shield = opts.shield_spec.as_ref().map(Clone::clone);
            let create_runplan = |benchmark: &Benchmark| {
                let path = benchmark.entrypoint_path.clone();
                RunPlan::new(
                    benchmark.clone(),
                    // TODO(anp): serialize criterion config if we have it
                    None,
                    shield.clone(),
                    toolchain.clone(),
                    path,
                )
            };

            for benchmark in &benchmarks {
                let rp = create_runplan(benchmark)?;
                rp.validate()?;

                // TODO check if we can skip this

                plans
                    .entry(toolchain.clone())
                    .or_insert(BTreeSet::new())
                    .insert(rp);
            }
        }

        Ok(plans)
    }

    fn compute_binary_hash(&self, rp: &RunPlan) -> Result<Entry<index::Key, Vec<u8>>> {
        let (key, maybe_existing) = self.existing_binary_hash(rp)?;

        Ok(match maybe_existing {
            Some(e) => Entry::Existing(e),
            None => Entry::New(key, rp.build()?, self.0.clone()),
        })
    }

    fn existing_binary_hash(&self, rp: &RunPlan) -> Result<(index::Key, Option<Vec<u8>>)> {
        let ikey = index::Key::new(&rp);
        let found = ikey.get(&self.0)?.map(|a| a.1);
        Ok((ikey, found))
    }

    fn compute_estimates(
        &self,
        rp: &RunPlan,
        binary_hash: &[u8],
    ) -> Result<Entry<measurement::Key, <measurement::Key as StorageKey>::Contents>> {
        let (mkey, maybe_existing) = self.existing_estimates(rp, binary_hash)?;

        Ok(match maybe_existing {
            Some(e) => Entry::Existing(e),
            None => {
                let res = rp
                    .exec()
                    .map_err(|why| Error {
                        kind: ErrorKind::Run(why.to_string()),
                        max_retries: DEFAULT_MAX_RETRIES,
                        num_retries: 0,
                        retryable: false,
                    })
                    .and_then(|()| {
                        self.process(&rp).map_err(|why| Error {
                            kind: ErrorKind::Run(why.to_string()),
                            max_retries: DEFAULT_MAX_RETRIES,
                            num_retries: 0,
                            retryable: false,
                        })
                    });

                Entry::New(mkey, res, self.0.clone())
            }
        })
    }

    fn existing_estimates(
        &self,
        rp: &RunPlan,
        binary_hash: &[u8],
    ) -> Result<(
        measurement::Key,
        Option<<measurement::Key as StorageKey>::Contents>,
    )> {
        let mkey = measurement::Key::new(
            binary_hash.to_vec(),
            // TODO make sure the benchmark's runner isn't optional by here?
            rp.benchmark.runner.clone().unwrap(),
            rp.shield.clone(),
        );

        let found = mkey.get(&self.0)?.map(|a| a.1);
        Ok((mkey, found))
    }

    /// Run a planned benchmark from before it has been built through to storing its results in
    /// the data directory.
    ///
    /// As optimizations, this may not actually build the binary or run the benchmarks if the data
    /// directory already has their respsective outputs for the provided RunPlan.
    ///
    /// Assumes that the `RunPlan`'s toolchain has already been installed.
    pub fn run(&mut self, rp: RunPlan) -> Result<()> {
        // TODO git cleanliness and update operations go here

        let binary_hash = self.compute_binary_hash(&rp)?;
        let estimates = self.compute_estimates(&rp, &*binary_hash)?;

        binary_hash.ensure_persisted()?;
        estimates.ensure_persisted()?;

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
    let mut collector = Collector::new(Path::new("/tmp/lolbenchtest")).unwrap();
    collector.run(plan).unwrap();
}
