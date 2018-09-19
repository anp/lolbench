use super::*;

use std::path::{Path, PathBuf};

use askama::Template;
use chrono::{DateTime, Utc};

pub fn build_website(
    data_dir: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    publish: bool,
) -> Result<()> {
    info!("reading all estimates from the data directory...");
    let data_storage = GitStore::ensure_initialized(data_dir.as_ref())?;
    let estimates = data_storage
        .all_stored_estimates()?
        .into_iter()
        .map(|(name, estimates)| {
            (
                name,
                estimates
                    .into_iter()
                    .filter_map(|(maybe_tc, ests)| maybe_tc.map(|tc| (tc, ests)))
                    .collect(),
            )
        }).collect();

    info!("running analysis, building the website...");
    let website = Website::from_estimates(estimates)?;
    let files = website.render_files()?;

    info!("generated {} files.", files.len());

    let mut output_storage = if publish {
        let mut output_storage = GitStore::ensure_initialized(output_dir.as_ref())?;
        output_storage.sync_down()?;
        Some(output_storage)
    } else {
        None
    };

    info!("cleaning the output directory...");
    for entry in ::std::fs::read_dir(output_dir.as_ref())? {
        let entry = entry?;
        let p = entry.path();
        if p.file_name().unwrap() != ".git" {
            debug!("deleting {}", p.display());
            if p.is_dir() {
                ::std::fs::remove_dir_all(p)?;
            } else {
                ::std::fs::remove_file(p)?;
            }
        } else {
            debug!("skipping .git dir in output directory");
        }
    }

    info!("writing files to output directory...");
    for (subpath, contents) in files {
        let abspath = output_dir.as_ref().join(subpath);
        let parent = abspath.parent().unwrap();

        debug!("creating {}", parent.display());
        ::std::fs::create_dir_all(parent)?;

        debug!("writing {}...", abspath.display());
        ::std::fs::write(&abspath, contents)?;
    }

    if let Some(output_storage) = &mut output_storage {
        info!("committing to storage...");
        output_storage.commit(&format!("build @ {}", website.generated_at))?;

        info!("pushing to a remote if it exists...");
        output_storage.push()?;
    }

    info!("all done writing website to disk!");

    Ok(())
}

#[derive(Template)]
#[template(path = "index.html")]
struct Website {
    generated_at: DateTime<Utc>,
    pub benchmarks: Vec<Benchmark>,
}

impl Website {
    pub fn from_estimates(
        estimates: BTreeMap<String, BTreeMap<Toolchain, (Vec<u8>, Estimates)>>,
    ) -> Result<Self> {
        let benchmarks = estimates
            .clone()
            .into_iter()
            .map(|(name, estimates)| {
                let analysis = Analysis::from_estimates(&estimates);
                Benchmark {
                    name,
                    estimates,
                    analysis,
                }
            }).collect();
        Ok(Self {
            generated_at: Utc::now(),
            benchmarks,
        })
    }

    pub fn render_files(&self) -> Result<Vec<(PathBuf, Vec<u8>)>> {
        let mut files = vec![(
            PathBuf::from("index.html"),
            self.render().unwrap().into_bytes(),
        )];

        for benchmark in &self.benchmarks {
            files.push((benchmark.path(), benchmark.render().unwrap().into_bytes()));
        }

        Ok(files)
    }
}

#[derive(Template)]
#[template(path = "benchmark.html")]
pub struct Benchmark {
    name: String,
    estimates: BTreeMap<Toolchain, (Vec<u8>, Estimates)>,
    analysis: Analysis,
}

impl Benchmark {
    fn timings(&self) -> Vec<Timing> {
        let mut estimates = self.estimates.iter();
        let mut chunks = Vec::new();
        let first_estimate = estimates.next().unwrap();

        let mut current_binhash = (first_estimate.1).0.clone();
        let mut current_toolchains = vec![first_estimate.0.clone()];
        let mut current_measure = (first_estimate.1).1.clone();
        let mut previous_timing = None;

        while let Some((tc, &(ref binhash, ref measure))) = estimates.next() {
            if binhash == &current_binhash {
                current_toolchains.push(tc.to_owned());
            } else {
                let timing = Timing::new(
                    &current_binhash,
                    &current_toolchains,
                    &current_measure,
                    previous_timing.as_ref(),
                );

                previous_timing = Some(timing.clone());
                chunks.push(timing);

                current_binhash = binhash.to_owned();
                current_toolchains = vec![tc.to_owned()];
                current_measure = measure.to_owned();
            }
        }

        chunks.reverse();
        chunks
    }

    pub fn path(&self) -> PathBuf {
        Path::new("benchmarks").join(slugify(&self.name) + ".html")
    }

    pub fn link(&self) -> ::askama::MarkupDisplay<String> {
        ::askama::MarkupDisplay::Safe(format!(
            r#"<a href="{}">{}</a>"#,
            self.path().display(),
            self.name
        ))
    }
}

#[derive(Clone)]
struct Timing {
    binary_hash: String,
    toolchains: Vec<Toolchain>,
    delta: Option<TimingDelta>,
    nanoseconds: f64,
    instructions: f64,
}

#[derive(Clone)]
struct TimingDelta {
    nanoseconds_pct: f64,
    instructions_pct: f64,
}

impl Timing {
    fn new(
        current_binhash: &[u8],
        current_toolchains: &[Toolchain],
        current_measure: &Estimates,
        previous: Option<&Self>,
    ) -> Self {
        let nice_hex =
            String::from_utf8(current_binhash.iter().fold(Vec::new(), |mut buf, byte| {
                use std::io::Write;
                buf.write_fmt(format_args!("{:x}", byte)).unwrap();
                buf
            })).unwrap();

        let nanoseconds = current_measure["nanoseconds"].median.point_estimate;
        let instructions = current_measure["instructions"].median.point_estimate;

        let pct = |a, b| ((b - a) / a) * 100.0;
        let delta = previous.map(|prev| TimingDelta {
            nanoseconds_pct: pct(prev.nanoseconds, nanoseconds),
            instructions_pct: pct(prev.instructions, instructions),
        });

        Self {
            binary_hash: nice_hex,
            toolchains: current_toolchains.to_owned(),
            delta,
            nanoseconds,
            instructions,
        }
    }

    // other things we might want to track:
    // "align-faults"
    // "bus-cycles"
    // "context-switches"
    // "cpu-clock"
    // "cpu-migrations"
    // "bpu-read-access"
    // "bpu-read-miss"
    // "branch-instructions"
    // "branch-misses"
    // "cache-misses"
    // "cache-references"
    // "cpu-cycles"
    // "dtlb-read-access"
    // "dtlb-read-miss"
    // "dtlb-write-access"
    // "dtlb-write-miss"
    // "emulation-faults"
    // "itlb-read-access"
    // "itlb-read-miss"
    // "l1d-read-access"
    // "l1d-read-miss"
    // "l1d-write-access"
    // "l1i-read-miss"
    // "ll-read-access"
    // "ll-read-miss"
    // "ll-write-access"
    // "ll-write-miss"
    // "node-read-access"
    // "node-read-miss"
    // "node-write-access"
    // "node-write-miss"
    // "page-fault"
    // "page-fault-minor"
}

mod filters {
    use separator::Separatable;
    pub fn float_fmt(s: &f64) -> Result<String, ::askama::Error> {
        let separated = s.separated_string();
        Ok(separated
            .find('.')
            .map(|dot_idx| {
                separated
                    .split_at(::std::cmp::min(separated.len(), dot_idx + 3))
                    .0
                    .to_owned()
            }).unwrap_or(separated))
    }
}
