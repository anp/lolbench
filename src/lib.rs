#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate byteorder;
extern crate chrono;
extern crate criterion;
extern crate glob;
extern crate lolbench_support;
extern crate marky_mark;
extern crate noisy_float;
extern crate ring;
extern crate serde;
extern crate serde_json;
extern crate simple_logger;
extern crate slug;

#[cfg(test)]
#[macro_use]
extern crate proptest;
#[cfg(test)]
extern crate tempfile;

mod collector;
mod cpu_shield;
mod registry;
mod run_plan;
mod storage;
mod toolchain;

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use chrono::NaiveDate;

pub use self::{collector::*, cpu_shield::*, registry::*, run_plan::*, storage::*, toolchain::*};
pub use lolbench_support::*;
pub use marky_mark::*;

pub fn measure(opts: BenchOpts, data_dir: &Path) -> Result<()> {
    info!("ensuring data dir {} exists", data_dir.display());
    let collector = Collector::new(data_dir)?;

    info!("cataloging potential builds to run");
    let candidates = opts.enumerate_bench_candidates()?;

    info!(
        "{} possible benchmark plans to run to satisfy provided options, pruning...",
        candidates.len()
    );

    let to_run = collector.compute_builds_needed(&candidates)?;

    info!("{} plans to run after pruning, running...", to_run.len());

    for (toolchain, benches) in to_run {
        info!("running {} benches with {}", benches.len(), toolchain);
        collector.run_benches_with_toolchain(toolchain, &benches)?;
    }

    info!("all done!");

    Ok(())
}

pub fn end_to_end_test(
    crate_name: &str,
    bench_name: &str,
    bench_source_name: &str,
    binary_name: &str,
) {
    let _ = simple_logger::init();

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
            entrypoint_path: PathBuf::from("/dev/null"),
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
    let collector = Collector::new(Path::new("/tmp/lolbenchtest")).unwrap();
    collector.run(&plan).unwrap();
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct BenchOpts {
    pub shield_spec: Option<ShieldSpec>,
    pub runner: Option<String>,
    pub toolchains: ToolchainSpec,
}

impl BenchOpts {
    pub fn enumerate_bench_candidates(&self) -> Result<BTreeMap<Toolchain, BTreeSet<RunPlan>>> {
        let benchmarks = get_benches(self.runner.as_ref().map(String::as_str))?;
        let toolchains = self.toolchains.all_of_em();

        let mut plans = BTreeMap::new();

        for toolchain in toolchains {
            let shield = self.shield_spec.as_ref().map(Clone::clone);
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
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ToolchainSpec {
    Single(String),
    Range(NaiveDate, NaiveDate),
}

impl ToolchainSpec {
    fn all_of_em(&self) -> Vec<Toolchain> {
        use ToolchainSpec::*;
        match self {
            Single(s) => vec![Toolchain::from(s)],
            Range(start, end) => {
                let mut current = *start;
                let mut nightlies = Vec::new();

                while current <= *end {
                    nightlies.push(Toolchain::from(&format!("nightly-{}", current)));
                    current = current.succ();
                }

                nightlies
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolchain_date_range() {
        let spec = ToolchainSpec::Range(
            NaiveDate::from_ymd(2015, 5, 15),
            NaiveDate::from_ymd(2015, 6, 2),
        );

        macro_rules! lolol {
            ( $( $datefrag:expr, )* ) => {
                vec![
                $(
                    Toolchain::from(concat!("nightly-2015-", $datefrag)),
                )*
                ]
            };
        }

        assert_eq!(
            spec.all_of_em(),
            lolol![
                "05-15", "05-16", "05-17", "05-18", "05-19", "05-20", "05-21", "05-22", "05-23",
                "05-24", "05-25", "05-26", "05-27", "05-28", "05-29", "05-30", "05-31", "06-01",
                "06-02",
            ]
        )
    }
}
