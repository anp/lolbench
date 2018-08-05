use super::prelude::*;

use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

use serde_json;

use cpu_shield::RenameThisCommandWrapper;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct RunPlan {
    shield: Option<ShieldSpec>,
    toolchain: String,
    source_path: PathBuf,
    output_dir: PathBuf,
    target_dir: PathBuf,
    manifest_path: PathBuf,
    benchmark: Benchmark,
    binary_path: PathBuf,
}

impl RunPlan {
    pub fn new(
        benchmark: Benchmark,
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

        Ok(Self {
            benchmark,
            shield,
            toolchain,
            source_path,
            output_dir,
            target_dir,
            manifest_path,
            binary_path,
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

        self.build()?;

        RenameThisCommandWrapper::new(&self.binary_path, self.shield.clone())
            .env("CARGO_TARGET_DIR", &self.target_dir)
            .status()?;

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
            .arg(&self.benchmark.name)
            .env("CARGO_TARGET_DIR", &self.target_dir)
            .output()?;

        if !build_output.status.success() {
            let stdout = String::from_utf8(build_output.stdout).unwrap();
            let stderr = String::from_utf8(build_output.stderr).unwrap();
            bail!(
                "failed to build {:?}. stdout: {}, stderr: {}",
                self,
                stdout,
                stderr
            );
        }

        Ok(())
    }

    fn post_process(&self) -> Result<Estimates> {
        let criterion_dir = self.target_dir.join("criterion");
        let path: PathBuf = panic!("TODO some sort of magic to infer criterion output dir");

        info!("postprocessing");
        let benchmark = path.file_name()
            .expect("finding the filename")
            .to_string_lossy()
            .to_string();

        let runtime_estimates_path = path.join("new").join("estimates.json");
        let metrics_estimates_path = path.join("new").join("metrics-estimates.json");

        let runtime_estimates_json = fs::read_to_string(runtime_estimates_path)?;
        let metrics_estimates_json = fs::read_to_string(metrics_estimates_path)?;

        let runtime_estimates: Statistic = serde_json::from_str(&runtime_estimates_json)?;
        let mut metrics_estimates: Estimates = serde_json::from_str(&metrics_estimates_json)?;

        metrics_estimates.insert(String::from("nanoseconds"), runtime_estimates);

        Ok(metrics_estimates)
    }
}

// the below is adapted from criterion

pub type Estimates = BTreeMap<String, Statistic>;

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
