use super::prelude::*;

use std::fs::read_to_string;

use glob::glob;

use marky_mark::Benchmark;

#[derive(Debug, Deserialize, Serialize)]
pub struct Plans {
    /// When the plan was generated in UTC according to the local clock.
    generated_at: NaiveDateTime,

    /// All of the benchmarks which should be run, with which options.
    plans: BTreeSet<RunPlan>,
}

impl Plans {
    pub(crate) fn new(benches_dir: &Path, bench_opts: BenchOpts) -> Result<Self> {
        info!("Searching {} for benchmarks...", benches_dir.display());

        let mut benchmarks: Vec<(PathBuf, Benchmark)> = Vec::new();
        for file in glob(&format!("{}/**/*.rs", benches_dir.display()))? {
            let file = file?;
            let contents = read_to_string(&file)?;
            if let Ok((bench, _)) = Benchmark::parse(&contents) {
                benchmarks.push((file, bench));
            }
        }

        info!("Found and parsed {} benchmarks.", benchmarks.len());

        let benchmarks = benchmarks
            .into_iter()
            .filter(|b| bench_opts.filter.matches(&b.1))
            .collect::<Vec<_>>();

        info!(
            "Applied filter {:?}, {} benchmarks remain.",
            bench_opts.filter,
            benchmarks.len()
        );

        let toolchains = bench_opts.toolchains.all_of_em();

        info!("Will run with these toolchains: {:?}", toolchains);

        let plans = toolchains
            .into_iter()
            .flat_map(move |toolchain: String| {
                let shield = bench_opts.shield_spec.as_ref().map(Clone::clone);

                benchmarks
                    .clone()
                    .into_iter()
                    .map(move |(path, benchmark)| RunPlan {
                        benchmark,
                        shield: shield.clone(),
                        toolchain: toolchain.clone(),
                        source_path: path.to_owned(),
                    })
            })
            .collect();

        Ok(Self {
            generated_at: Utc::now().naive_utc(),
            plans,
        })
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct RunPlan {
    shield: Option<ShieldSpec>,
    toolchain: String,
    source_path: PathBuf,
    benchmark: Benchmark,
}

#[derive(Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct BenchOpts {
    shield_spec: Option<ShieldSpec>,
    filter: BenchFilter,
    toolchains: ToolchainSpec,
}

impl BenchOpts {
    pub fn unshielded(filter: BenchFilter, toolchains: ToolchainSpec) -> Self {
        Self {
            filter,
            toolchains,
            shield_spec: None,
        }
    }

    pub fn shielded(filter: BenchFilter, toolchains: ToolchainSpec, shield: ShieldSpec) -> Self {
        Self {
            filter,
            toolchains,
            shield_spec: Some(shield),
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub enum ToolchainSpec {
    Single(String),
    Range(NaiveDate, NaiveDate),
}

impl ToolchainSpec {
    fn all_of_em(&self) -> Vec<String> {
        use ToolchainSpec::*;
        match self {
            Single(s) => vec![s.to_owned()],
            Range(start, end) => {
                let mut current = *start;
                let mut nightlies = Vec::new();

                while current <= *end {
                    nightlies.push(format!("nightly-{}", current));
                    current = current.succ();
                }

                nightlies
            }
        }
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub enum BenchFilter {
    All,
    Runner(String),
}

impl BenchFilter {
    fn matches(&self, bench: &Benchmark) -> bool {
        match (self, &bench.runner) {
            (BenchFilter::All, _) => true,
            (BenchFilter::Runner(current), Some(ref required)) => current == required,
            _ => false,
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
                    String::from(concat!("nightly-2015-", $datefrag)),
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
