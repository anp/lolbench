use super::Result;

use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result as FmtResult},
    path::{Path, PathBuf},
    process::Command,
};

use ring::digest::{digest, SHA256};

use marky_mark::Benchmark;

use cpu_shield::{RenameThisCommandWrapper, ShieldSpec};
use toolchain::Toolchain;
use CriterionConfig;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct RunPlan {
    pub binary_name: String,
    pub benchmark: Benchmark,
    pub toolchain: Option<Toolchain>,
    pub bench_config: Option<CriterionConfig>,
    pub shield: Option<ShieldSpec>,
    pub source_path: PathBuf,
    pub manifest_path: PathBuf,
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
        toolchain: Option<Toolchain>,
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
        self.target_dir().join("release").join(&self.binary_name)
    }

    fn target_dir(&self) -> Cow<Path> {
        use std::env::var as envvar;

        match self.toolchain {
            Some(ref t) => Cow::Borrowed(t.target_dir()),
            None => Cow::Owned(PathBuf::from(
                envvar("CARGO_TARGET_DIR").unwrap_or_else(|_| String::from("target")),
            )),
        }
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
        let target_name = self.source_path.file_stem().unwrap().to_string_lossy();
        info!("building {} with {:?}", target_name, self.toolchain);

        let mut cmd = Command::new("cargo");

        if let Some(ref t) = self.toolchain {
            cmd.arg(format!("+{}", t));
        }

        let output = cmd
            .arg("build")
            .arg("--release")
            .arg("--manifest-path")
            .arg(&self.manifest_path)
            .arg("--bin")
            .arg(&*target_name)
            .env("CARGO_TARGET_DIR", &*self.target_dir())
            .output()?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!(
                "Unable to build {} with {:?} (None means current, adam is lazy).\nstdout:{}\nstderr:{}",
                target_name,
                self.toolchain,
                stdout,
                stderr
            );
        }

        info!("done building {}", self.source_path.display());

        let bin_path = self.binary_path();

        debug!("reading contents of {} at {}", self, bin_path.display());
        let bin_contents = ::std::fs::read(&bin_path)?;

        debug!("hashing binary contents");
        Ok(digest(&SHA256, &bin_contents).as_ref().to_owned())
    }

    /// Runs the benchmark target, implicitly writing criterion results to the target directory.
    pub fn exec(&self) -> Result<()> {
        debug!("configuring command for {}", self);

        let mut cmd = RenameThisCommandWrapper::new("cargo", self.shield.clone());

        if let Some(ref t) = self.toolchain {
            cmd.arg(format!("+{}", t));
        }

        cmd.args(&["run", "--release", "--manifest-path"]);
        cmd.arg(&self.manifest_path); // silly types
        cmd.args(&["--bin", &self.binary_name]);

        cmd.env("CARGO_TARGET_DIR", &*self.target_dir());

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
