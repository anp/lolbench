use super::Result;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::{Command, ExitStatus};

use serde_json;

use cpu_shield::RenameThisCommandWrapper;

pub fn run_benchmark(
    bench_dir: impl AsRef<Path>,
    bench_name: &str,
    toolchain: &str,
    target_dir: impl AsRef<Path>,
    _cpu_pattern: Option<&str>,
    _move_kthreads: bool,
) -> Result<ExitStatus> {
    let cargo_action = |action: &str| -> Result<()> {
        info!("{:?} {} with {}...", bench_dir.as_ref(), action, toolchain);
        let build_output = Command::new("rustup")
            .arg("run")
            .arg(toolchain)
            .arg("cargo")
            .arg(action)
            .arg("--release")
            .arg("--bin")
            .arg(bench_name)
            .current_dir(bench_dir.as_ref())
            .env("CARGO_TARGET_DIR", target_dir.as_ref())
            .output()?;

        if !build_output.status.success() {
            let stderr = String::from_utf8(build_output.stderr).unwrap();
            bail!(
                "failed to {} {:?} with {}{}",
                action,
                bench_dir.as_ref(),
                toolchain,
                stderr
            );
        }

        Ok(())
    };

    cargo_action("build")?;

    println!("Running benchmarks on {}...", toolchain);

    let mut binary_path = Path::new(target_dir.as_ref()).join("release");
    binary_path.push("run_benches");

    let mut shielded_runner = RenameThisCommandWrapper::new(&binary_path);
    shielded_runner.env("CARGO_TARGET_DIR", target_dir.as_ref());

    #[cfg(target_os = "linux")]
    {
        if let Some(mask) = _cpu_pattern {
            shielded_runner.cpu_mask(mask);
            shielded_runner.move_kthreads(_move_kthreads);
        }
    }

    Ok(shielded_runner.status()?)
}

pub fn run_with_toolchain(
    toolchain: &str,
    _cpu_pattern: &Option<String>,
    _move_kthreads: bool,
) -> Result<()> {
    // let target_dir = format!("target-{}", toolchain);

    if !install_toolchain(toolchain)? {
        warn!("couldn't install {}", toolchain);
        return Ok(());
    }

    // FIXME(anp): pass the right compiler flags (LTO, etc)

    // FIXME(anp): figure out which benchmarks to run
    // FIXME(anp): run each benchmark in turn, and post process them separately
    // FIXME(anp): accept an output path argument and bundle them all together

    // let exit = benchmark::run_benchmarks(
    // bench_dir: impl AsRef<Path>,
    // bench_name: &str,
    // toolchain: &str,
    // target_dir: impl AsRef<Path>,
    // _cpu_pattern: Option<&str>,
    // _move_kthreads: bool,
    //     toolchain,
    //     target_dir,
    //     _cpu_pattern.as_ref().map(|s| s.as_str()),
    //     _move_kthreads,
    // ).expect("running benchmark");

    // println!("exit status: {:?}", exit);

    post_process(toolchain)?;

    Ok(())
}

pub fn install_toolchain(toolchain: &str) -> Result<bool> {
    info!("Installing {}...", toolchain);
    let install_output = Command::new("rustup")
        .arg("toolchain")
        .arg("install")
        .arg(toolchain)
        .output()
        .expect("unable to run rustup");

    if !install_output.status.success() {
        let stderr = String::from_utf8(install_output.stderr).unwrap();

        if !stderr.find("no release found").is_some() {
            // we failed to install, and rustup isn't telling us that it can't find the release
            // so something is probably wrong (disk space, perms, etc)
            bail!(
                "rustup failed to install {}, but it wasn't because the release was missing: {}",
                toolchain,
                stderr
            );
        }

        warn!("No release found for {}.", toolchain);
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn post_process(toolchain: &str) -> Result<()> {
    let mut measurements = BTreeMap::new();
    let target_dir = format!("target-{}", &toolchain);
    let criterion_dir = Path::new(&target_dir).join("criterion");

    println!("postprocessing");
    for entry in fs::read_dir(&criterion_dir).expect("reading criterion directory") {
        let entry = entry.expect("reading directory entry");
        let path = entry.path();
        let benchmark = path.file_name()
            .expect("finding the filename")
            .to_string_lossy()
            .to_string();

        if !entry.file_type().expect("finding the file type").is_dir() {
            continue;
        }

        let runtime_estimates_path = path.join("new").join("estimates.json");
        let metrics_estimates_path = path.join("new").join("metrics-estimates.json");

        let runtime_estimates_json = fs::read_to_string(runtime_estimates_path)?;
        let metrics_estimates_json = fs::read_to_string(metrics_estimates_path)?;

        let runtime_estimates: Estimates = serde_json::from_str(&runtime_estimates_json)?;
        let mut metrics_estimates: BTreeMap<String, Estimates> =
            serde_json::from_str(&metrics_estimates_json)?;

        metrics_estimates.insert(String::from("nanoseconds"), runtime_estimates);
        measurements.insert(benchmark, metrics_estimates);
    }

    let compiled = CompiledResults {
        toolchain: toolchain.to_string(),
        measurements,
    };

    fs::write(
        criterion_dir.join("lolbench-output.json"),
        serde_json::to_string_pretty(&compiled)?,
    )?;

    Ok(())
}

#[derive(Serialize)]
struct CompiledResults {
    toolchain: String,
    measurements: BTreeMap<String, BTreeMap<String, Estimates>>,
}

// the below is adapted from criterion

type Estimates = BTreeMap<Statistic, Estimate>;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
pub enum Statistic {
    Mean,
    Median,
    MedianAbsDev,
    Slope,
    StdDev,
}

#[derive(Clone, Copy, PartialEq, Deserialize, Serialize, Debug)]
struct ConfidenceInterval {
    confidence_level: f64,
    lower_bound: f64,
    upper_bound: f64,
}

#[derive(Clone, Copy, PartialEq, Deserialize, Serialize, Debug)]
struct Estimate {
    /// The confidence interval for this estimate
    confidence_interval: ConfidenceInterval,
    ///
    point_estimate: f64,
    /// The standard error of this estimate
    standard_error: f64,
}
