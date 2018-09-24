use super::Result;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::path::PathBuf;
use std::process::Command;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Toolchain {
    spec: String,
}

impl Toolchain {
    pub fn from(s: &str) -> Self {
        Toolchain {
            spec: s.to_string(),
        }
    }

    pub fn target_dir(&self) -> PathBuf {
        PathBuf::from(format!("/tmp/target-{}", self.spec))
    }

    fn is_installed(&self) -> Result<bool> {
        let installed_toolchains_output = Command::new("rustup")
            .arg("toolchain")
            .arg("list")
            .output()?;

        let stdout = String::from_utf8_lossy(&installed_toolchains_output.stdout);
        Ok(stdout.contains(&self.spec))
    }

    pub fn ensure_installed(&self) -> Result<Option<InstallGuard>> {
        if self.is_installed()? {
            info!("{} already installed, skipping installation", self);
            return Ok(None);
        }

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

        Ok(Some(InstallGuard(self.clone())))
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

impl Display for Toolchain {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_fmt(format_args!("{}", self.spec))
    }
}

pub struct InstallGuard(Toolchain);

impl Drop for InstallGuard {
    fn drop(&mut self) {
        if let Err(e) = ::std::fs::remove_dir_all(self.0.target_dir()) {
            warn!(
                "unable to clean up {}'s target directory ({}): {:?}",
                self.0,
                self.0.target_dir().display(),
                e
            );
        }
        if let Err(e) = self.0.uninstall() {
            warn!("unable to uninstall {}: {:?}", self.0, e);
        }
    }
}
