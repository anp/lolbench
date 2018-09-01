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
    pub binary_name: String,
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
        benchmark: Benchmark,
        bench_config: Option<CriterionConfig>,
        shield: Option<ShieldSpec>,
        toolchain: Toolchain,
        source_path: PathBuf,
    ) -> Result<Self> {
        let binary_name = source_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

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
            manifest_path,
            binary_name,
            bench_config,
        })
    }

    fn binary_path(&self) -> PathBuf {
        self.toolchain
            .target_dir()
            .join("release")
            .join(&self.binary_name)
    }

    pub fn validate(&self) -> Result<()> {
        ensure!(self.source_path.is_file(), "source_path is not a file");
        ensure!(self.manifest_path.is_file(), "manifest_path is not a file");

        // pub shield: Option<ShieldSpec>,
        if let Some(_spec) = &self.shield {
            // TODO(anp) validate it!
        }

        if let Some(_cfg) = &self.bench_config {
            // TODO(anp) validate it!
        }

        Ok(())
    }

    /// Builds the benchmark target and returns the SHA256 sum of the binary.
    pub fn build(&self) -> Result<Vec<u8>> {
        self.toolchain
            .build_benchmark(&self.source_path, &self.manifest_path)?;

        let bin_path = self.binary_path();

        debug!("reading contents of {} at {}", self, bin_path.display());
        let bin_contents = ::std::fs::read(&bin_path)?;

        debug!("hashing binary contents");
        Ok(digest(&SHA256, &bin_contents).as_ref().to_owned())
    }

    /// Runs the benchmark target, implicitly writing criterion results to the target directory.
    pub fn exec(&self) -> Result<()> {
        debug!("configuring command for {}", self);

        let mut cmd = RenameThisCommandWrapper::new("rustup", self.shield.clone());
        cmd.args(&[
            "run",
            &self.toolchain.to_string(),
            "cargo",
            "run",
            "--release",
            "--manifest-path",
        ]);
        cmd.arg(&self.manifest_path); // silly types
        cmd.args(&["--bin", &self.binary_name]);

        cmd.env("CARGO_TARGET_DIR", &self.toolchain.target_dir());

        if let Some(cfg) = &self.bench_config {
            debug!("applying criterion config");
            for (k, v) in cfg.envs() {
                cmd.env(k, v);
            }
        }

        debug!("running {} with {:?}", self, cmd);
        let output = cmd.output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !output.status.success() {
            bail!("benchmark failed! stdout: {}, stderr: {}", stdout, stderr);
        }

        Ok(())
    }
}
