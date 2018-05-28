extern crate chrono;
extern crate clap;
#[macro_use]
extern crate log;
extern crate lolbench;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate simple_logger;
#[macro_use]
extern crate structopt;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use chrono::{Duration, NaiveDate, Utc};
use structopt::StructOpt;

use lolbench::cpu_shield::RenameThisCommandWrapper;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short = "c", long = "cpus")]
    cpu_pattern: Option<String>,
    #[structopt(short = "k", long = "move-kthreads")]
    move_kernel_threads: bool,
    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    #[structopt(name = "single")]
    Single { toolchain: String },
    #[structopt(name = "nightlies-since")]
    NightliesSince { date: NaiveDate },
}

fn main() {
    let opt = Options::from_args();

    simple_logger::init_with_level(log::Level::Debug).unwrap();

    match opt.cmd {
        SubCommand::Single { toolchain } => {
            run_with_toolchain(&toolchain, &opt.cpu_pattern, opt.move_kernel_threads);
        }
        SubCommand::NightliesSince { date } => {
            let mut current = date;
            let today = Utc::today().naive_utc();

            while current <= today {
                let toolchain = format!("nightly-{}", current);
                info!("running {}", toolchain);

                run_with_toolchain(&toolchain, &opt.cpu_pattern, opt.move_kernel_threads);

                current += Duration::days(1);
            }
        }
    }
}

fn run_with_toolchain(toolchain: &str, _cpu_pattern: &Option<String>, _move_kthreads: bool) {
    let target_dir = format!("target-{}", toolchain);

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
            panic!(
                "rustup failed to install {}, but it wasn't because the release was missing: {}",
                toolchain, stderr
            );
        }

        warn!("No release found for {}, skipping.", toolchain);
        return;
    }

    info!("Building benchmark runner with {}...", toolchain);
    let build_output = Command::new("rustup")
        .arg("run")
        .arg(toolchain)
        .arg("cargo")
        .arg("build")
        .arg("--release")
        .arg("--bin")
        .arg("run_benches")
        .env("CARGO_TARGET_DIR", &target_dir)
        .output()
        .expect("failed to spawn benchmark builder");

    if !build_output.status.success() {
        let stderr = String::from_utf8(build_output.stderr).unwrap();
        warn!(
            "failed to build benchmarks with {}, skipping:\n\n{}",
            toolchain, stderr
        );

        return;
    }

    println!("Running benchmarks on {}...", toolchain);

    let mut binary_path = ::std::path::Path::new(&target_dir).join("release");
    binary_path.push("run_benches");

    let mut shielded_runner = RenameThisCommandWrapper::new(&binary_path);
    shielded_runner.env("CARGO_TARGET_DIR", &target_dir);

    #[cfg(target_os = "linux")]
    {
        if let &Some(ref mask) = _cpu_pattern {
            shielded_runner.cpu_mask(mask);
        }

        shielded_runner.move_kthreads(_move_kthreads);
    }

    let exit = shielded_runner.status().expect("failed to run benchmarks");

    post_process(toolchain);

    println!("{:?}", exit);
}

#[derive(Serialize)]
struct CompiledResults {
    toolchain: String,
    measurements: BTreeMap<String, BTreeMap<String, Estimates>>,
}

fn post_process(toolchain: &str) {
    let mut measurements = BTreeMap::new();
    let target_dir = format!("target-{}", &toolchain);
    let criterion_dir = Path::new(&target_dir).join("criterion");

    println!("postprocessing");
    for entry in fs::read_dir(&criterion_dir).expect("reading criterion directory") {
        let entry = entry.expect("reading directory entry");
        let path = entry.path();
        let benchmark = path
            .file_name()
            .expect("finding the filename")
            .to_string_lossy()
            .to_string();

        if !entry.file_type().expect("finding the file type").is_dir() {
            continue;
        }

        let runtime_estimates_path = path.join("new").join("estimates.json");
        let metrics_estimates_path = path.join("new").join("metrics-estimates.json");

        let runtime_estimates_json =
            fs::read_to_string(runtime_estimates_path).expect("reading runtime estimates");
        let metrics_estimates_json =
            fs::read_to_string(metrics_estimates_path).expect("reading metrics estimates");

        let runtime_estimates: Estimates = serde_json::from_str(&runtime_estimates_json)
            .expect("parsing runtime estimates as json");
        let mut metrics_estimates: BTreeMap<String, Estimates> =
            serde_json::from_str(&metrics_estimates_json)
                .expect("parsing metrics estimates as json");

        metrics_estimates.insert(String::from("nanoseconds"), runtime_estimates);
        measurements.insert(benchmark, metrics_estimates);
    }

    let compiled = CompiledResults {
        toolchain: toolchain.to_string(),
        measurements,
    };

    fs::write(
        criterion_dir.join("lolbench-output.json"),
        serde_json::to_string_pretty(&compiled).expect("converting compiled results to json"),
    ).expect("writing compiled results to disk");
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
