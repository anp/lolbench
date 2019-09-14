use super::Result;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use itertools::Itertools;
use serde_json;

use run_plan::RunPlan;
use signal::exit_if_needed;
use storage::{index, measurement, Entry, Estimates, GitStore, Statistic, StorageKey};
use toolchain::Toolchain;
use website::build_website;

pub type CollectionResult<T> = ::std::result::Result<T, self::Error>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ErrorKind {
    Build(String),
    Run(String),
    PostProcess(String),
}

/// Runs benchmarks, memoizes their results, and allows results to be shared across multiple
/// toolchains if the binaries they produce are identical.
pub struct Collector {
    storage: GitStore,
    data_dir: PathBuf,
    site_dir: PathBuf,
}

impl Collector {
    /// Open a Collector. Creates the passed path and initializes a git repo there if it does not
    /// already exist.
    pub fn new(data_dir: &Path, site_dir: &Path) -> Result<Self> {
        ::std::fs::create_dir_all(data_dir)?;
        let storage = GitStore::ensure_initialized(data_dir)?;
        Ok(Collector {
            storage,
            data_dir: data_dir.to_owned(),
            site_dir: site_dir.to_owned(),
        })
    }

    fn batch_commit(
        &mut self,
        toolchain: &Toolchain,
        results: &mut Vec<(bool, RunPlan, Option<String>)>,
        publish: bool,
    ) -> Result<()> {
        let mut lines = Vec::new();
        let mut num_ok = 0;
        let mut num_err = 0;
        let mut runners = ::std::collections::BTreeSet::new();
        for (ok, rp, hexhash) in results.drain(..) {
            if ok {
                num_ok += 1;
            } else {
                num_err += 1;
            }

            runners.insert(rp.benchmark.clone().runner.unwrap());

            lines.push(format!(
                "\n{} {}, binary {:?}\n{:#?}",
                if ok { "ok" } else { "err" },
                rp,
                hexhash,
                rp
            ));
        }

        self.storage.commit(&format!(
            "{} {:?}, {} ok, {} err\n{}",
            toolchain,
            runners,
            num_ok,
            num_err,
            lines.iter().join("\n")
        ))?;

        if publish {
            self.storage.sync_down()?;
            self.storage.push()?;
        }

        build_website(&self.data_dir, &self.site_dir, publish)?;

        Ok(())
    }

    /// Run all the passed benchmarks with the given toolchain, installing the toolchain beforehand
    /// and uninstalling it afterwards if it was installed by us.
    pub fn run_benches_with_toolchain(
        &mut self,
        toolchain: Toolchain,
        run_plans: &[RunPlan],
        publish: bool,
    ) -> Result<()> {
        exit_if_needed();

        if publish {
            self.storage.sync_down()?;
        }

        let _guard = toolchain.ensure_installed()?;

        let mut results = Vec::new();

        let batch_size = 10;
        let mut pushed = 0;

        for rp in run_plans {
            exit_if_needed();

            let (status, hexhash) = self.run(rp)?;
            results.push((status, rp.clone(), hexhash));

            pushed += 1;
            if pushed == batch_size {
                self.batch_commit(&toolchain, &mut results, publish)?;
                pushed = 0;
            }
        }

        // pick up any stragglers
        self.batch_commit(&toolchain, &mut results, publish)?;

        Ok(())
    }

    /// Take a list of potential benchmarks to run and filter out any plans for which we have end to
    /// end results stored already.
    pub fn compute_builds_needed(
        &mut self,
        plans: &BTreeMap<Toolchain, BTreeSet<RunPlan>>,
    ) -> Result<BTreeMap<Toolchain, Vec<RunPlan>>> {
        let mut needed = BTreeMap::new();

        for (toolchain, run_plans) in plans {
            for rp in run_plans {
                if !self.plan_can_be_skipped_with_no_work(rp)? {
                    needed
                        .entry(toolchain.clone())
                        .or_insert_with(Vec::new)
                        .push(rp.to_owned());
                }
            }
        }

        Ok(needed)
    }

    /// Check to see if we need to do anything with this RunPlan. Used for conveniently pruning
    /// the list of benchmarks before we start installing toolchains and building binaries.
    fn plan_can_be_skipped_with_no_work(&mut self, rp: &RunPlan) -> Result<bool> {
        Ok(
            if let (_, Some(Ok(hash))) = self.existing_binary_hash(rp)? {
                if let (_, Some(_)) = self.existing_estimates(rp, &hash)? {
                    true
                } else {
                    false
                }
            } else {
                false
            },
        )
    }

    /// Builds a benchmark binary for the current runner if it not been previously built and run.
    fn compute_binary_hash(&mut self, rp: &RunPlan) -> Result<Entry<index::Key>> {
        let (key, maybe_existing) = self.existing_binary_hash(rp)?;

        Ok(match maybe_existing {
            Some(r) => Entry::Existing(r),
            None => Entry::New(
                key,
                rp.build().map_err(|e| Error {
                    kind: ErrorKind::Build(e.to_string()),
                }),
            ),
        })
    }

    /// Checks to see if we've previously built a binary for this exact RunPlan and stored its hash.
    fn existing_binary_hash(
        &mut self,
        rp: &RunPlan,
    ) -> Result<(index::Key, Option<CollectionResult<Vec<u8>>>)> {
        let ikey = index::Key::new(&rp);
        let found = self.storage.get(&ikey)?;
        Ok((ikey, found))
    }

    /// Runs a benchmark for the current runner if the results have not previously been recorded.
    fn compute_estimates(
        &mut self,
        rp: &RunPlan,
        binary_hash: &[u8],
    ) -> Result<Entry<measurement::Key>> {
        let (mkey, maybe_existing) = self.existing_estimates(rp, binary_hash)?;

        let res = match maybe_existing {
            Some(e) => Entry::Existing(e),
            None => {
                let res = rp
                    .exec()
                    .map_err(|why| Error {
                        kind: ErrorKind::Run(why.to_string()),
                    })
                    .and_then(|()| {
                        self.process(&rp).map_err(|why| Error {
                            kind: ErrorKind::Run(why.to_string()),
                        })
                    });

                Entry::New(mkey, res)
            }
        };

        Ok(res)
    }

    /// Check to see if we have already have measurements for this benchmark.
    fn existing_estimates(
        &mut self,
        rp: &RunPlan,
        binary_hash: &[u8],
    ) -> Result<(
        measurement::Key,
        Option<<measurement::Key as StorageKey>::Contents>,
    )> {
        let mkey = measurement::Key::new(
            binary_hash.to_vec(),
            rp.benchmark.runner.clone(),
            rp.shield.clone(),
        );

        let found = self.storage.get(&mkey)?;
        Ok((mkey, found))
    }

    /// Run a planned benchmark from before it has been built through to storing its results in
    /// the data directory.
    ///
    /// As optimizations, this may not actually build the binary or run the benchmarks if the data
    /// directory already has their respsective outputs for the provided RunPlan.
    ///
    /// Assumes that the `RunPlan`'s toolchain has already been installed.
    pub fn run(&mut self, rp: &RunPlan) -> Result<(bool, Option<String>)> {
        let binary_hash_res = self.compute_binary_hash(rp)?;

        binary_hash_res
            .clone()
            .ensure_persisted(&mut self.storage)?;

        let (estimates, binary_hash) = if let Ok(hash) = &*binary_hash_res {
            (Some(self.compute_estimates(rp, &*hash)?), Some(hash))
        } else {
            (None, None)
        };

        let status = if let Some(e) = estimates {
            e.ensure_persisted(&mut self.storage)?;
            true
        } else {
            false
        };

        let hexhash = binary_hash.map(|h| {
            h.into_iter()
                .map(|d| format!("{:x}", d))
                .fold(String::new(), |mut acc, x| {
                    acc.push_str(&x);
                    acc
                })
        });

        Ok((status, hexhash))
    }

    /// Parses the results of a benchmark. This assumes that the benchmark has already been
    /// executed.
    fn process(&self, rp: &RunPlan) -> Result<Estimates> {
        info!("post-processing {}", rp);

        let target_dir = rp
            .toolchain
            .as_ref()
            .map(Toolchain::target_dir)
            .unwrap_or_else(|| PathBuf::from("target"));

        let path = target_dir
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
