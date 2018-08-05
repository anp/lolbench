use super::prelude::*;

use std::fs::read_to_string;

use glob::glob;

use marky_mark::Benchmark;

#[derive(Debug, Deserialize, Serialize)]
pub struct Plans {
    /// When the plan was generated in UTC according to the local clock.
    generated_at: NaiveDateTime,

    /// Configuration of CPU resources.
    shield_spec: ShieldSpec,

    /// All of the benchmarks which should be run, with which options.
    plans: BTreeSet<RunPlan>,
}

impl Plans {
    pub(crate) fn new(benches_dir: &Path, bench_opts: BenchOpts) -> Result<Self> {
        info!("Searching {} for benchmarks...", benches_dir.display());

        let mut benchmarks = Vec::new();
        for file in glob(&format!("{}/**/*.rs", benches_dir.display()))? {
            let contents = read_to_string(file?)?;
            if let Ok((bench, _)) = Benchmark::parse(&contents) {
                benchmarks.push(bench);
            }
        }

        info!("Found and parsed {} benchmarks.", benchmarks.len());

        let benchmarks = benchmarks
            .into_iter()
            .filter(|b| bench_opts.filter.matches(b))
            .collect::<Vec<_>>();

        info!(
            "Applied filter {:?}, {} benchmarks remain.",
            bench_opts.filter,
            benchmarks.len()
        );

        // TODO figure out which toolchains to use

        // TODO create run plans
        unimplemented!()
    }

    pub fn write(&self, p: &Path) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct RunPlan {
    toolchain: String,
    source_path: PathBuf,
    binary_path: PathBuf,
}

#[derive(Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct BenchOpts {
    shield_spec: Option<ShieldSpec>,
    filter: BenchFilter,
}

impl BenchOpts {
    pub fn unshielded(filter: BenchFilter) -> Self {
        Self {
            filter,
            shield_spec: None,
        }
    }

    pub fn shielded(filter: BenchFilter, shield: ShieldSpec) -> Self {
        Self {
            filter,
            shield_spec: Some(shield),
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
