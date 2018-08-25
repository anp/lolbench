use super::Result;

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use noisy_float::prelude::*;
use serde_json;

use marky_mark::Benchmark;

use cpu_shield::{RenameThisCommandWrapper, ShieldSpec};
use CriterionConfig;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct RunPlan {
    pub shield: Option<ShieldSpec>,
    pub toolchain: String,
    pub source_path: PathBuf,
    pub target_dir: PathBuf,
    pub manifest_path: PathBuf,
    pub benchmark: Benchmark,
    pub binary_path: PathBuf,
    pub bench_config: Option<CriterionConfig>,
}

impl RunPlan {
    pub fn new(
        mut benchmark: Benchmark,
        bench_config: Option<CriterionConfig>,
        shield: Option<ShieldSpec>,
        toolchain: String,
        source_path: PathBuf,
        output_dir: PathBuf,
    ) -> Result<Self> {
        let target_dir = PathBuf::from(output_dir.join(format!("target-{}", toolchain)));
        let binary_path = target_dir
            .join("release")
            .join(source_path.file_stem().unwrap());

        let mut manifest_path = None;
        for dir in source_path.ancestors() {
            let candidate = dir.join("Cargo.toml");
            if candidate.is_file() {
                manifest_path = Some(candidate);
                break;
            }
        }

        let manifest_path = manifest_path.unwrap();

        benchmark.strip();

        Ok(Self {
            benchmark,
            shield,
            toolchain,
            source_path,
            target_dir,
            manifest_path,
            binary_path,
            bench_config,
        })
    }

    pub fn run(self) -> Result<Estimates> {
        info!("running {:?}", self);

        if let Err(why) = self.attempt_toolchain_install() {
            warn!(
                "the next series of commands will almost certainly fail. error: {:?}",
                why
            );
        };

        info!("building benchmark binary");

        self.build()?;

        info!("shelling out to {}", self.binary_path.display());

        let mut cmd = RenameThisCommandWrapper::new(&self.binary_path, self.shield.clone());
        cmd.env("CARGO_TARGET_DIR", &self.target_dir);

        if let Some(cfg) = &self.bench_config {
            for (k, v) in cfg.envs() {
                cmd.env(k, v);
            }
        }

        let output = cmd.output()?;

        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;

        if !output.status.success() {
            bail!("benchmark failed! stdout: {}, stderr: {}", stdout, stderr);
        }

        debug!(
            "benchmark run complete.\nstdout: {}\nstderr: {}",
            stdout, stderr
        );

        Ok(self.post_process()?)
    }

    pub fn attempt_toolchain_install(&self) -> Result<()> {
        info!("Installing {}...", self.toolchain);
        let install_output = Command::new("rustup")
            .arg("toolchain")
            .arg("install")
            .arg(&self.toolchain)
            .output()?;

        if !install_output.status.success() {
            let stderr = String::from_utf8(install_output.stderr).unwrap();

            if !stderr.find("no release found").is_some() {
                // we failed to install, and rustup isn't telling us that it can't find the release
                // so something is probably wrong (disk space, perms, etc)
                bail!(
                "rustup failed to install {}, but it wasn't because the release was missing: {}",
                self.toolchain,
                stderr
            );
            }

            bail!("No release found for {}.", self.toolchain);
        }
        Ok(())
    }

    fn build(&self) -> Result<()> {
        let build_output = Command::new("rustup")
            .arg("run")
            .arg(&self.toolchain)
            .arg("cargo")
            .arg("build")
            .arg("--release")
            .arg("--manifest-path")
            .arg(&self.manifest_path)
            .arg("--bin")
            .arg(&self.source_path.file_stem().unwrap())
            .env("CARGO_TARGET_DIR", &self.target_dir)
            .output()?;

        if !build_output.status.success() {
            let stdout = String::from_utf8(build_output.stdout).unwrap();
            let stderr = String::from_utf8(build_output.stderr).unwrap();
            bail!(
                "failed to build {:#?}.\nstdout: {},\nstderr: {}",
                self,
                stdout,
                stderr
            );
        }

        Ok(())
    }

    fn post_process(&self) -> Result<Estimates> {
        let path = self
            .target_dir
            .join("criterion")
            .join(format!(
                "{}::{}",
                &self.benchmark.crate_name, &self.benchmark.name
            ))
            .join("new");

        info!("postprocessing");

        let runtime_estimates_path = path.join("estimates.json");

        debug!(
            "reading runtime estimates from disk @ {}",
            runtime_estimates_path.display()
        );
        let runtime_estimates_json = fs::read_to_string(runtime_estimates_path)?;

        debug!("parsing runtime estimates");
        let runtime_estimates: Statistic = serde_json::from_str(&runtime_estimates_json)?;

        let mut metrics_estimates = Estimates::new();

        metrics_estimates.insert(String::from("nanoseconds"), runtime_estimates);

        let metrics_estimates_path = path.join("metrics-estimates.json");
        debug!("reading metrics estimates from disk");
        if let Ok(metrics_estimates_json) = fs::read_to_string(metrics_estimates_path) {
            debug!("parsing metrics estimates");
            let estimates: Estimates = serde_json::from_str(&metrics_estimates_json)?;
            metrics_estimates.extend(estimates);
        } else {
            warn!(
                "couldn't read metrics-estimates.json for {:?}",
                &self.benchmark
            );
        }

        Ok(metrics_estimates)
    }
}

// the below is adapted from criterion

pub type Estimates = BTreeMap<String, Statistic>;

// TODO(anp): tests for this with criterion output
#[derive(Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
pub struct Statistic {
    #[serde(rename = "Mean")]
    mean: Estimate,
    #[serde(rename = "Median")]
    median: Estimate,
    #[serde(rename = "MedianAbsDev")]
    median_abs_dev: Estimate,
    #[serde(rename = "Slope")]
    slope: Estimate,
    #[serde(rename = "StdDev")]
    std_dev: Estimate,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
struct ConfidenceInterval {
    confidence_level: f64,
    lower_bound: f64,
    upper_bound: f64,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
struct Estimate {
    /// The confidence interval for this estimate
    confidence_interval: ConfidenceInterval,
    ///
    point_estimate: f64,
    /// The standard error of this estimate
    standard_error: f64,
}

pub fn end_to_end_test(
    crate_name: &str,
    bench_name: &str,
    bench_source_name: &str,
    binary_name: &str,
) {
    let _ = ::simple_logger::init();

    let target_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target");
    let binary_path = target_dir.join("release").join(binary_name);

    let plan = RunPlan {
        shield: None,
        toolchain: String::from("stable"),
        source_path: Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("benches")
            .join(crate_name)
            .join("src")
            .join("bin")
            .join(bench_source_name),
        target_dir,
        manifest_path: Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("benches")
            .join(crate_name)
            .join("Cargo.toml"),
        benchmark: Benchmark {
            runner: None,
            name: String::from(bench_name),
            crate_name: String::from(crate_name),
        },
        binary_path,
        bench_config: Some(CriterionConfig {
            confidence_level: r32(0.95),
            measurement_time_ms: 500,
            nresamples: 2,
            noise_threshold: r32(0.0),
            sample_size: 5,
            significance_level: r32(0.05),
            warm_up_time_ms: 1,
        }),
    };

    if let Err(why) = plan.run() {
        panic!("{}", why);
    }
}
