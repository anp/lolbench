use super::Result;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::PathBuf;

use ring::digest::{digest, SHA256};

use marky_mark::Benchmark;

use cpu_shield::{RenameThisCommandWrapper, ShieldSpec};
use toolchain::Toolchain;
use CriterionConfig;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct RunPlan {
    pub shield: Option<ShieldSpec>,
    pub toolchain: Toolchain,
    pub source_path: PathBuf,
    pub manifest_path: PathBuf,
    pub benchmark: Benchmark,
    pub binary_path: PathBuf,
    pub bench_config: Option<CriterionConfig>,
}

impl Display for RunPlan {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_fmt(format_args!(
            "{}::{}@{:?}",
            self.benchmark.crate_name, self.benchmark.name, self.benchmark.runner,
        ))
    }
}

impl RunPlan {
    pub fn new(
        mut benchmark: Benchmark,
        bench_config: Option<CriterionConfig>,
        shield: Option<ShieldSpec>,
        toolchain: Toolchain,
        source_path: PathBuf,
    ) -> Result<Self> {
        let binary_path = toolchain
            .target_dir()
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
            manifest_path,
            binary_path,
            bench_config,
        })
    }

    pub fn validate(&self) -> Result<()> {
        ensure!(self.source_path.is_file(), "source_path is not a file");
        ensure!(self.manifest_path.is_file(), "manifest_path is not a file");

        // pub shield: Option<ShieldSpec>,
        // pub toolchain: String,
        // pub benchmark: Benchmark,
        // pub bench_config: Option<CriterionConfig>,
        // pub binary_path: PathBuf,

        Ok(())
    }

    /// Builds the benchmark target and returns the SHA256 sum of the binary.
    pub fn build(&self) -> Result<Vec<u8>> {
        self.toolchain
            .build_benchmark(&self.source_path, &self.manifest_path)?;

        let bin_contents = ::std::fs::read(&self.binary_path)?;
        Ok(digest(&SHA256, &bin_contents).as_ref().to_owned())
    }

    /// Runs the benchmark target, implicitly writing criterion results to the target directory.
    pub fn exec(&self) -> Result<()> {
        info!("running {}", self);

        let mut cmd = RenameThisCommandWrapper::new(&self.binary_path, self.shield.clone());
        cmd.env("CARGO_TARGET_DIR", &self.toolchain.target_dir());

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
        Ok(())
    }
}
