extern crate chrono;
extern crate clap;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate serde_json;
extern crate simple_logger;
extern crate structopt;
extern crate syn;
extern crate toml;
extern crate walkdir;

use std::path::PathBuf;
use std::process::Command;

pub mod benchmark;
pub mod cpu_shield;
pub mod extract;
pub mod measure;

pub type Result<T> = std::result::Result<T, failure::Error>;

#[macro_export]
macro_rules! criterion_from_env {
    ($( $build_method:ident, )*) => {
        {
            let mut crit = ::criterion::Criterion::default().without_plots();

            $(
                if let Ok(v) = ::std::env::var(
                    concat!("lolbench_", stringify!($build_method))
                ) {
                    println!("setting {}", stringify!($build_method));
                    crit = crit.$build_method(v.parse().unwrap());
                }
            )*

            crit
        }
    };
}

#[macro_export]
macro_rules! lolbench {
    ($krate:ident, $benchmark:ident) => {
        extern crate criterion;
        extern crate $krate;

        trait CriterionExt: Sized {
            fn warm_up_time_ms(self, ms: usize) -> ::criterion::Criterion;
            fn measurement_time_ms(self, ms: usize) -> ::criterion::Criterion;
        }

        impl CriterionExt for ::criterion::Criterion {
            #[inline]
            fn warm_up_time_ms(self, ms: usize) -> Self {
                self.warm_up_time(::std::time::Duration::from_millis(ms as u64))
            }

            #[inline]
            fn measurement_time_ms(self, ms: usize) -> Self {
                self.measurement_time(::std::time::Duration::from_millis(ms as u64))
            }
        }

        fn main() {
            use std::default::Default;
            ::criterion::init_logging();

            let mut crit = criterion_from_env!(
                sample_size,
                warm_up_time_ms,
                measurement_time_ms,
                nresamples,
                noise_threshold,
                confidence_level,
                significance_level,
            );

            $krate::$benchmark(&mut crit);
        }
    };
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
