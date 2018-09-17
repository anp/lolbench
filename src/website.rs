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
        })
        .collect();

    info!("running analysis, building the website...");
    let website = Website::from_estimates(estimates)?;

    info!("generating file list and contents...");
    let files = website.render();
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

struct Website {
    generated_at: DateTime<Utc>,
    analysis: Analysis,
    benchmarks: BTreeMap<String, BTreeMap<Toolchain, Estimates>>,
    index: templates::Index,
}

impl Website {
    pub fn from_estimates(
        estimates: BTreeMap<String, BTreeMap<Toolchain, Estimates>>,
    ) -> Result<Self> {
        let benchmark_names = estimates.keys().cloned().collect();
        Ok(Self {
            generated_at: Utc::now(),
            analysis: Analysis::from_estimates(&estimates),
            benchmarks: estimates,
            index: templates::Index {
                benchmarks: benchmark_names,
            },
        })
    }

    pub fn render(&self) -> Vec<(PathBuf, Vec<u8>)> {
        vec![(
            PathBuf::from("index.html"),
            self.index.render().unwrap().into_bytes(),
        )]
    }
}

mod templates {
    use super::*;

    #[derive(Template)]
    #[template(path = "index.html")]
    pub struct Index {
        pub benchmarks: Vec<String>,
    }
}
