extern crate lolbench;

#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;

extern crate chrono;
extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate simple_logger;
extern crate syn;
extern crate toml;
extern crate walkdir;

use std::path::PathBuf;

use chrono::{Duration, NaiveDate, Utc};
use structopt::StructOpt;

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
    #[structopt(name = "extract")]
    ExtractBins {
        directory: String,
        bin_output: String,
    },
    #[structopt(name = "single")]
    Single { toolchain: String },
    #[structopt(name = "nightlies-since")]
    NightliesSince { date: NaiveDate },
}

fn main() {
    let opt = Options::from_args();

    simple_logger::init_with_level(log::Level::Debug).unwrap();

    match opt.cmd {
        SubCommand::ExtractBins {
            directory,
            bin_output,
        } => {
            let dir = PathBuf::from(directory);
            let bin_output = PathBuf::from(bin_output);
            lolbench::extract::extract_and_write_crate(&dir, &bin_output)
                .expect(&format!("couldn't extract benchmark runners for {:?}", dir));
        }
        SubCommand::Single { toolchain } => {
            lolbench::run_with_toolchain(&toolchain, &opt.cpu_pattern, opt.move_kernel_threads)
                .expect(&format!("couldn't run benchmarks for {}", toolchain));
        }
        SubCommand::NightliesSince { date } => {
            let mut current = date;
            let today = Utc::today().naive_utc();

            while current <= today {
                let toolchain = format!("nightly-{}", current);
                info!("running {}", toolchain);

                lolbench::run_with_toolchain(&toolchain, &opt.cpu_pattern, opt.move_kernel_threads)
                    .expect(&format!("couldn't run benchmarks for {}", toolchain));

                current += Duration::days(1);
            }
        }
    }
}
