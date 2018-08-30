use super::Result;

use std::collections::BTreeMap;

use chrono::NaiveDate;

use marky_mark::Benchmark;

use cpu_shield::ShieldSpec;
use run_plan::RunPlan;
use toolchain::Toolchain;

pub fn plan_benchmarks(opts: BenchOpts) -> Result<BTreeMap<Toolchain, Vec<RunPlan>>> {
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
            plans
                .entry(toolchain.clone())
                .or_insert(Vec::new())
                .push(rp);
        }
    }

    Ok(plans)
}

#[derive(Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct BenchOpts {
    pub shield_spec: Option<ShieldSpec>,
    pub runner: Option<String>,
    pub toolchains: ToolchainSpec,
}

#[derive(Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
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
