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
    analysis: Analysis,
}

impl Website {
    pub fn from_estimates(
        estimates: BTreeMap<String, BTreeMap<Toolchain, (Vec<u8>, Estimates)>>,
    ) -> Result<Self> {
        let mut benchmarks: Vec<Benchmark> = estimates
            .clone()
            .into_iter()
            .map(|(name, estimates)| Benchmark::new(name, estimates.into_iter()))
            .collect();

        let all_timings = benchmarks
            .iter()
            .flat_map(|b| {
                b.timings
                    .iter()
                    .map(move |t| (b.name.clone(), t.to_owned()))
            }).collect::<Vec<_>>();

        let analysis = Analysis::new(all_timings);

        for (toolchain, timings) in &analysis.anomalous_timings {
            for anomaly in timings {
                for benchmark in &mut benchmarks {
                    if &benchmark.name == &anomaly.bench_fn {
                        benchmark
                            .anomalous_timings
                            .entry(toolchain.clone())
                            .or_default()
                            .push(anomaly.to_owned());
                    }
                }
            }
        }

        Ok(Self {
            generated_at: Utc::now(),
            benchmarks,
            analysis,
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
    timings: Vec<TimingRecord>,
    anomalous_timings: BTreeMap<Toolchain, Vec<AnomalousTiming>>,
}

impl Benchmark {
    pub fn empty(name: String) -> Self {
        Self {
            name,
            timings: vec![],
            anomalous_timings: BTreeMap::new(),
        }
    }

    fn new(
        name: String,
        mut estimates: impl Iterator<Item = (Toolchain, (Vec<u8>, Estimates))>,
    ) -> Self {
        let mut timings = Vec::new();
        let first_estimate = estimates.next().unwrap();

        let mut current_binhash = (first_estimate.1).0;
        let mut current_toolchains = vec![first_estimate.0];
        let mut current_measure = (first_estimate.1).1;

        while let Some((tc, (binhash, measure))) = estimates.next() {
            if binhash == current_binhash {
                current_toolchains.push(tc.to_owned());
            } else {
                let timing = TimingRecord::new(
                    &current_binhash,
                    &current_toolchains,
                    &current_measure,
                    &timings,
                );

                timings.push(timing);

                current_binhash = binhash.to_owned();
                current_toolchains = vec![tc.to_owned()];
                current_measure = measure.to_owned();
            }
        }

        timings.reverse();
        Self {
            name: name.to_owned(),
            timings,
            anomalous_timings: BTreeMap::new(),
        }
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

    fn metrics_with_anomaly_indices(&self) -> Vec<&'static str> {
        vec![
            "nanoseconds",
            "instructions",
            "cpu_cycles",
            "branch_instructions",
            "branch_misses",
            "cache_references",
            "cache_misses",
        ]
    }
}

pub mod filters {
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
