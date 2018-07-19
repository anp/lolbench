use super::Result;

use std::path::Path;
use std::process::{Command, ExitStatus};

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
