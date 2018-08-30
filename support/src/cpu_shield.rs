use super::Result;

use std::ffi::OsStr;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io;
use std::process::{Child, Command, ExitStatus, Output};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub struct ShieldSpec {
    pub cpu_mask: String,
    pub kthread_on: bool,
}

impl ShieldSpec {
    pub fn new(cpu_mask: String, kthread_on: bool) -> Result<Self> {
        // FIXME(anp): these arguments need to be validated
        Ok(Self {
            cpu_mask,
            kthread_on,
        })
    }
}

impl Display for ShieldSpec {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_fmt(format_args!(
            "{}{}",
            if self.kthread_on { "k" } else { "" },
            self.cpu_mask
        ))
    }
}

pub struct RenameThisCommandWrapper {
    shielded: Command,
    spec: Option<ShieldSpec>,
}

impl RenameThisCommandWrapper {
    pub fn new<S: AsRef<OsStr>>(cmd: S, spec: Option<ShieldSpec>) -> Self {
        let shielded = if spec.is_some() {
            if !cfg!(target_os = "linux") {
                panic!("cpu shielding not supported on non-linux OSes");
            }

            let mut shielded = Command::new("cset");
            shielded.arg("sh");
            shielded.arg(cmd);
            shielded
        } else {
            Command::new(cmd)
        };

        Self { shielded, spec }
    }

    pub fn spawn(&mut self) -> io::Result<Child> {
        unimplemented!();
    }

    fn maybe_with_shielded<F, R>(&mut self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Command) -> R,
    {
        if let Some(spec) = self.spec.as_ref() {
            info!("creating cpu shield");
            // sudo cset shield --cpu=${CPU_MASK} --kthread=${on|off}

            // TODO notify sudo dep, find another way to do this
            let mut shield_create = Command::new("sudo");
            shield_create.arg("cset");
            shield_create.arg("shield");

            // NOTE: this should only get called if the cpu_mask has already been validated
            shield_create.arg(format!("--cpu={}", spec.cpu_mask));

            if spec.kthread_on {
                shield_create.arg("--kthread=on");
            }

            let output = shield_create.output()?;

            if !output.status.success() {
                let stdout = String::from_utf8(output.stdout)?;
                let stderr = String::from_utf8(output.stderr)?;
                bail!(
                    "unable to create cpu shield. stdout: {}, stderr: {}",
                    stdout,
                    stderr
                );
            }

            // end cset shield setup

            let result = f(&mut self.shielded);

            // tear down shield
            let reset_res = Command::new("sudo")
                .arg("cset")
                .arg("shield")
                .arg("--reset")
                .status();

            match reset_res {
                Err(why) => error!("error destroying shield: {:#?}", why),
                _ => (),
            };

            Ok(result)
        } else {
            info!("running binary");
            Ok(f(&mut self.shielded))
        }
    }

    pub fn output(&mut self) -> Result<Output> {
        Ok(self.maybe_with_shielded(|cmd| cmd.output())??)
    }

    pub fn status(&mut self) -> Result<ExitStatus> {
        Ok(self.maybe_with_shielded(|cmd| cmd.status())??)
    }

    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.shielded.env(key, val);
        self
    }
}
