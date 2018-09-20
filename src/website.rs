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
            .map(|(name, estimates)| Benchmark::new(name, estimates.into_iter()))
            .collect();
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
    timings: Vec<Timing>,
}

impl Benchmark {
    fn new(
        name: String,
        mut estimates: impl Iterator<Item = (Toolchain, (Vec<u8>, Estimates))>,
    ) -> Self {
        let mut timings = Vec::new();
        let first_estimate = estimates.next().unwrap();

        let mut current_binhash = (first_estimate.1).0;
        let mut current_toolchains = vec![first_estimate.0];
        let mut current_measure = (first_estimate.1).1;
        let mut previous_timing = None;

        while let Some((tc, (binhash, measure))) = estimates.next() {
            if binhash == current_binhash {
                current_toolchains.push(tc.to_owned());
            } else {
                let timing = Timing::new(
                    &current_binhash,
                    &current_toolchains,
                    &current_measure,
                    previous_timing.as_ref(),
                );

                previous_timing = Some(timing.clone());
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
}

#[derive(Clone, Serialize)]
struct Timing {
    binary_hash: String,
    toolchains: Vec<Toolchain>,
    delta: Option<TimingDelta>,

    nanoseconds: f64,
    instructions: f64,

    align_faults: f64,
    bus_cycles: f64,
    context_switches: f64,
    cpu_clock: f64,
    cpu_migrations: f64,
    bpu_read_access: f64,
    bpu_read_miss: f64,
    branch_instructions: f64,
    branch_misses: f64,
    cache_misses: f64,
    cache_references: f64,
    cpu_cycles: f64,
    dtlb_read_access: f64,
    dtlb_read_miss: f64,
    dtlb_write_access: f64,
    dtlb_write_miss: f64,
    emulation_faults: f64,
    itlb_read_access: f64,
    itlb_read_miss: f64,
    l1d_read_access: f64,
    l1d_read_miss: f64,
    l1d_write_access: f64,
    l1i_read_miss: f64,
    ll_read_access: f64,
    ll_read_miss: f64,
    ll_write_access: f64,
    ll_write_miss: f64,
    node_read_access: f64,
    node_read_miss: f64,
    node_write_access: f64,
    node_write_miss: f64,
    page_fault: f64,
    page_fault_minor: f64,
}

#[derive(Clone, Serialize)]
struct TimingDelta {
    nanoseconds_pct: f64,
    instructions_pct: f64,
    align_faults_pct: f64,
    bus_cycles_pct: f64,
    context_switches_pct: f64,
    cpu_clock_pct: f64,
    cpu_migrations_pct: f64,
    bpu_read_access_pct: f64,
    bpu_read_miss_pct: f64,
    branch_instructions_pct: f64,
    branch_misses_pct: f64,
    cache_misses_pct: f64,
    cache_references_pct: f64,
    cpu_cycles_pct: f64,
    dtlb_read_access_pct: f64,
    dtlb_read_miss_pct: f64,
    dtlb_write_access_pct: f64,
    dtlb_write_miss_pct: f64,
    emulation_faults_pct: f64,
    itlb_read_access_pct: f64,
    itlb_read_miss_pct: f64,
    l1d_read_access_pct: f64,
    l1d_read_miss_pct: f64,
    l1d_write_access_pct: f64,
    l1i_read_miss_pct: f64,
    ll_read_access_pct: f64,
    ll_read_miss_pct: f64,
    ll_write_access_pct: f64,
    ll_write_miss_pct: f64,
    node_read_access_pct: f64,
    node_read_miss_pct: f64,
    node_write_access_pct: f64,
    node_write_miss_pct: f64,
    page_fault_pct: f64,
    page_fault_minor_pct: f64,
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
        let align_faults = current_measure["align-faults"].median.point_estimate;
        let bus_cycles = current_measure["bus-cycles"].median.point_estimate;
        let context_switches = current_measure["context-switches"].median.point_estimate;
        let cpu_clock = current_measure["cpu-clock"].median.point_estimate;
        let cpu_migrations = current_measure["cpu-migrations"].median.point_estimate;
        let bpu_read_access = current_measure["bpu-read-access"].median.point_estimate;
        let bpu_read_miss = current_measure["bpu-read-miss"].median.point_estimate;
        let branch_instructions = current_measure["branch-instructions"].median.point_estimate;
        let branch_misses = current_measure["branch-misses"].median.point_estimate;
        let cache_misses = current_measure["cache-misses"].median.point_estimate;
        let cache_references = current_measure["cache-references"].median.point_estimate;
        let cpu_cycles = current_measure["cpu-cycles"].median.point_estimate;
        let dtlb_read_access = current_measure["dtlb-read-access"].median.point_estimate;
        let dtlb_read_miss = current_measure["dtlb-read-miss"].median.point_estimate;
        let dtlb_write_access = current_measure["dtlb-write-access"].median.point_estimate;
        let dtlb_write_miss = current_measure["dtlb-write-miss"].median.point_estimate;
        let emulation_faults = current_measure["emulation-faults"].median.point_estimate;
        let itlb_read_access = current_measure["itlb-read-access"].median.point_estimate;
        let itlb_read_miss = current_measure["itlb-read-miss"].median.point_estimate;
        let l1d_read_access = current_measure["l1d-read-access"].median.point_estimate;
        let l1d_read_miss = current_measure["l1d-read-miss"].median.point_estimate;
        let l1d_write_access = current_measure["l1d-write-access"].median.point_estimate;
        let l1i_read_miss = current_measure["l1i-read-miss"].median.point_estimate;
        let ll_read_access = current_measure["ll-read-access"].median.point_estimate;
        let ll_read_miss = current_measure["ll-read-miss"].median.point_estimate;
        let ll_write_access = current_measure["ll-write-access"].median.point_estimate;
        let ll_write_miss = current_measure["ll-write-miss"].median.point_estimate;
        let node_read_access = current_measure["node-read-access"].median.point_estimate;
        let node_read_miss = current_measure["node-read-miss"].median.point_estimate;
        let node_write_access = current_measure["node-write-access"].median.point_estimate;
        let node_write_miss = current_measure["node-write-miss"].median.point_estimate;
        let page_fault = current_measure["page-fault"].median.point_estimate;
        let page_fault_minor = current_measure["page-fault-minor"].median.point_estimate;

        let pct = |a, b| ((b - a) / a) * 100.0;
        let delta = previous.map(|prev| TimingDelta {
            nanoseconds_pct: pct(prev.nanoseconds, nanoseconds),
            instructions_pct: pct(prev.instructions, instructions),

            align_faults_pct: pct(prev.align_faults, align_faults),
            bus_cycles_pct: pct(prev.bus_cycles, bus_cycles),
            context_switches_pct: pct(prev.context_switches, context_switches),
            cpu_clock_pct: pct(prev.cpu_clock, cpu_clock),
            cpu_migrations_pct: pct(prev.cpu_migrations, cpu_migrations),
            bpu_read_access_pct: pct(prev.bpu_read_access, bpu_read_access),
            bpu_read_miss_pct: pct(prev.bpu_read_miss, bpu_read_miss),
            branch_instructions_pct: pct(prev.branch_instructions, branch_instructions),
            branch_misses_pct: pct(prev.branch_misses, branch_misses),
            cache_misses_pct: pct(prev.cache_misses, cache_misses),
            cache_references_pct: pct(prev.cache_references, cache_references),
            cpu_cycles_pct: pct(prev.cpu_cycles, cpu_cycles),
            dtlb_read_access_pct: pct(prev.dtlb_read_access, dtlb_read_access),
            dtlb_read_miss_pct: pct(prev.dtlb_read_miss, dtlb_read_miss),
            dtlb_write_access_pct: pct(prev.dtlb_write_access, dtlb_write_access),
            dtlb_write_miss_pct: pct(prev.dtlb_write_miss, dtlb_write_miss),
            emulation_faults_pct: pct(prev.emulation_faults, emulation_faults),
            itlb_read_access_pct: pct(prev.itlb_read_access, itlb_read_access),
            itlb_read_miss_pct: pct(prev.itlb_read_miss, itlb_read_miss),
            l1d_read_access_pct: pct(prev.l1d_read_access, l1d_read_access),
            l1d_read_miss_pct: pct(prev.l1d_read_miss, l1d_read_miss),
            l1d_write_access_pct: pct(prev.l1d_write_access, l1d_write_access),
            l1i_read_miss_pct: pct(prev.l1i_read_miss, l1i_read_miss),
            ll_read_access_pct: pct(prev.ll_read_access, ll_read_access),
            ll_read_miss_pct: pct(prev.ll_read_miss, ll_read_miss),
            ll_write_access_pct: pct(prev.ll_write_access, ll_write_access),
            ll_write_miss_pct: pct(prev.ll_write_miss, ll_write_miss),
            node_read_access_pct: pct(prev.node_read_access, node_read_access),
            node_read_miss_pct: pct(prev.node_read_miss, node_read_miss),
            node_write_access_pct: pct(prev.node_write_access, node_write_access),
            node_write_miss_pct: pct(prev.node_write_miss, node_write_miss),
            page_fault_pct: pct(prev.page_fault, page_fault),
            page_fault_minor_pct: pct(prev.page_fault_minor, page_fault_minor),
        });

        Self {
            binary_hash: nice_hex,
            toolchains: current_toolchains.to_owned(),
            delta,
            nanoseconds,
            instructions,

            align_faults,
            bus_cycles,
            context_switches,
            cpu_clock,
            cpu_migrations,
            bpu_read_access,
            bpu_read_miss,
            branch_instructions,
            branch_misses,
            cache_misses,
            cache_references,
            cpu_cycles,
            dtlb_read_access,
            dtlb_read_miss,
            dtlb_write_access,
            dtlb_write_miss,
            emulation_faults,
            itlb_read_access,
            itlb_read_miss,
            l1d_read_access,
            l1d_read_miss,
            l1d_write_access,
            l1i_read_miss,
            ll_read_access,
            ll_read_miss,
            ll_write_access,
            ll_write_miss,
            node_read_access,
            node_read_miss,
            node_write_access,
            node_write_miss,
            page_fault,
            page_fault_minor,
        }
    }
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
