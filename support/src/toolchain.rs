use super::Result;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Toolchain {
    spec: String,
    target_dir: PathBuf,
    uninstall_on_drop: bool,
}

impl Toolchain {
    pub fn build_benchmark(&self, source: &Path, crate_manifest: &Path) -> Result<()> {
        let target_name = source.file_stem().unwrap().to_string_lossy();
        info!("building {} with {}", target_name, self);

        let output = Command::new("rustup")
            .arg("run")
            .arg(&self.spec)
            .arg("cargo")
            .arg("build")
            .arg("--release")
            .arg("--manifest-path")
            .arg(crate_manifest)
            .arg("--bin")
            .arg(&*target_name)
            .env("CARGO_TARGET_DIR", &self.target_dir)
            .output()?;

        if !output.status.success() {
            bail!("Unable to build {} with {}", target_name, self);
        }

        info!("done building {}", source.display());
        Ok(())
    }

    pub fn from(s: &str) -> Self {
        Toolchain {
            spec: s.to_string(),
            target_dir: PathBuf::from(format!("/tmp/target-{}", s)),
            uninstall_on_drop: false,
        }
    }

    pub fn target_dir(&self) -> &Path {
        &self.target_dir
    }

    pub fn install(&self) -> Result<()> {
        // FIXME check if we should uninstall on drop!
        info!("Installing {}...", self);
        let install_output = Command::new("rustup")
            .arg("toolchain")
            .arg("install")
            .arg(&self.spec)
            .output()?;

        if !install_output.status.success() {
            let stderr = String::from_utf8(install_output.stderr).unwrap();

            if !stderr.find("no release found").is_some() {
                // we failed to install, and rustup isn't telling us that it can't find the release
                // so something is probably wrong (disk space, perms, etc)
                bail!(
                "rustup failed to install {}, but it wasn't because the release was missing: {}",
                self.spec,
                stderr
            );
            }

            bail!("No release found for {}.", self.spec);
        }
        Ok(())
    }

    pub fn uninstall(&self) -> Result<()> {
        info!("Uninstalling {}...", self);
        Command::new("rustup")
            .arg("toolchain")
            .arg("uninstall")
            .arg(&self.spec)
            .status()?;
        Ok(())
    }
}

impl Drop for Toolchain {
    fn drop(&mut self) {
        if self.uninstall_on_drop {
            if let Err(why) = self.uninstall() {
                error!("Unable to uninstall {}: {:?}", self, why);
            } else {
                info!("Uninstall successful.");
            }
        }
    }
}

impl Display for Toolchain {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_fmt(format_args!("{}", self.spec))
    }
}
