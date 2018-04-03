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
    #[cfg(target_os = "linux")]
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

    let target_dir = ::std::env::var("CARGO_TARGET_DIR").unwrap_or("target".to_string());
    let mut binary_path = ::std::path::PathBuf::from(target_dir);
    binary_path.push("release");
    binary_path.push("run_benches");

    let mut shielded_runner = RenameThisCommandWrapper::new(&binary_path);

    #[cfg(target_os = "linux")]
    {
        if let Some(mask) = opt.cpu_pattern {
            shielded_runner.cpu_mask(mask);
        }

        shielded_runner.move_kthreads(opt.move_kernel_threads);
    }

    let exit = shielded_runner.status().expect("failed to run benchmarks");

    println!("{:?}", exit);
}
