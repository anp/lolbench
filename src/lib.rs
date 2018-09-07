#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

extern crate byteorder;
extern crate chrono;
extern crate criterion;
extern crate git2;
extern crate glob;
extern crate itertools;
extern crate libc;
extern crate lolbench_support;
extern crate marky_mark;
extern crate min_max_heap;
extern crate noisy_float;
extern crate ring;
extern crate serde;
extern crate serde_json;
extern crate signal_hook;
extern crate simple_logger;
extern crate slug;
extern crate walkdir;

#[cfg(test)]
#[macro_use]
extern crate proptest;
#[cfg(test)]
extern crate tempfile;

mod collector;
mod cpu_shield;
mod generator;
mod registry;
mod run_plan;
mod signal;
mod storage;
mod toolchain;

pub use self::{
    collector::*, cpu_shield::*, generator::*, registry::*, run_plan::*, signal::*, storage::*,
    toolchain::*,
};
pub use lolbench_support::*;
pub use marky_mark::*;

use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use chrono::NaiveDate;
use slug::slugify;

pub fn measure(opts: BenchOpts, data_dir: &Path) -> Result<()> {
    info!("ensuring data dir {} exists", data_dir.display());
    let mut collector = Collector::new(data_dir)?;

    info!("cataloging potential builds to run");
    let candidates = opts.enumerate_bench_candidates()?;

    info!(
        "{} possible toolchains to run to satisfy provided options, pruning...",
        candidates.len()
    );

    let to_run = collector.compute_builds_needed(&candidates)?;

    info!("{} plans to run after pruning, running...", to_run.len());

    for (toolchain, benches) in to_run.into_iter().rev() {
        info!("running {} benches with {}", benches.len(), toolchain);
        collector.run_benches_with_toolchain(toolchain, &benches)?;
    }

    info!("all done!");

    Ok(())
}

pub fn end_to_end_test(crate_name: &str, bench_name: &str) {
    let bench_source_name = format!("{}.rs", slugify(bench_name));

    let _ = simple_logger::init();

    let entrypoint_path = Path::new("benches")
        .join(crate_name)
        .join("src")
        .join("bin")
        .join(bench_source_name);

    let source_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(&entrypoint_path);
    let plan = RunPlan::new(
        Benchmark {
            runner: None,
            name: String::from(bench_name),
            crate_name: String::from(crate_name),
            entrypoint_path,
        },
        Some(CriterionConfig {
            confidence_level: r32(0.95),
            measurement_time_ms: 500,
            nresamples: 2,
            noise_threshold: r32(0.0),
            sample_size: 5,
            significance_level: r32(0.05),
            warm_up_time_ms: 1,
        }),
        None,
        None,
        source_path,
    ).unwrap();

    let data_dir = ::std::env::var("LOLBENCH_DATA_DIR").unwrap_or_else(|_| {
        format!(
            "/tmp/lolbenchtest-{}-{}",
            slugify(crate_name),
            slugify(bench_name)
        )
    });

    // FIXME make this a proper temp dir
    let mut collector = Collector::new(Path::new(&data_dir)).unwrap();
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
                    Some(toolchain.clone()),
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
    fn quickcheck_end_to_end_test() {
        end_to_end_test("quickcheck_0_6_1", "shrink_string_1_tuple");
    }

    #[test]
    fn crossbeam_end_to_end_test() {
        end_to_end_test("crossbeam_epoch_0_4_0", "defer::multi_alloc_defer_free");
    }

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
