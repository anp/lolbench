extern crate chrono;
extern crate clap;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate simple_logger;
#[macro_use]
extern crate structopt;

use std::process::Command;

use chrono::{Duration, NaiveDate, Utc};
use structopt::StructOpt;

pub mod benchmark;
pub mod cpu_shield;
pub mod measure;

pub(crate) type Result<T> = std::result::Result<T, failure::Error>;

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
            run_with_toolchain(&toolchain, &opt.cpu_pattern, opt.move_kernel_threads)
                .expect(&format!("couldn't run benchmarks for {}", toolchain));
        }
        SubCommand::NightliesSince { date } => {
            let mut current = date;
            let today = Utc::today().naive_utc();

            while current <= today {
                let toolchain = format!("nightly-{}", current);
                info!("running {}", toolchain);

                run_with_toolchain(&toolchain, &opt.cpu_pattern, opt.move_kernel_threads)
                    .expect(&format!("couldn't run benchmarks for {}", toolchain));

                current += Duration::days(1);
            }
        }
    }
}

fn run_with_toolchain(
    toolchain: &str,
    _cpu_pattern: &Option<String>,
    _move_kthreads: bool,
) -> Result<()> {
    let target_dir = format!("target-{}", toolchain);

    if !install_toolchain(toolchain)? {
        warn!("couldn't install {}", toolchain);
        return Ok(());
    }

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

    measure::post_process(toolchain)?;

    Ok(())
}

fn install_toolchain(toolchain: &str) -> Result<bool> {
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
