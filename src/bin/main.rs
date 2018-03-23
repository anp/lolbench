extern crate clap;
extern crate lolbench;
#[macro_use]
extern crate structopt;

use std::process::{Command, Stdio};

use structopt::StructOpt;

use lolbench::cpu_shield::RenameThisCommandWrapper;

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short = "t", long = "toolchain")]
    toolchain: String,
    #[structopt(short = "c", long = "cpus")]
    cpu_pattern: Option<String>,
    #[cfg(target_os = "linux")]
    #[structopt(short = "k", long = "move-kthreads")]
    move_kernel_threads: bool,
}

fn main() {
    let opt = Options::from_args();

    // TODO check if rustup knows about the toolchain
    // TODO if rustup doesn't, install it, set uninstall flag
    println!(
        "Building benchmark runner with {} toolchain...",
        opt.toolchain
    );

    Command::new("cargo")
        // use rustup shortcut for invoking toolchain
        .arg(format!("+{}", opt.toolchain))
        .arg("build")
        .arg("--release")
        .arg("--bin")
        .arg("run_benches")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to spawn benchmark builder");

    println!("Running benchmarks...");
    let exit = RenameThisCommandWrapper::new("target/release/run_benches")
        .status()
        .expect("failed to run benchmarks");

    println!("{:?}", exit);
}
