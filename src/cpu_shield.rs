use super::prelude::*;

use std::ffi::OsStr;
use std::io;
use std::ops::{Deref, DerefMut};
use std::process::{Child, Command, ExitStatus, Output};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct ShieldSpec {
    cpu_mask: String,
    kthread_on: bool,
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

pub struct RenameThisCommandWrapper {
    shielded: Command,
    spec: Option<ShieldSpec>,
}

impl RenameThisCommandWrapper {
    pub fn new<S: AsRef<OsStr>>(cmd: S, spec: Option<ShieldSpec>) -> Self {
        let shielded = if cfg!(target_os = "linux") {
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

            shield_create.status()?;

            // end cset shield setup

            let result = f(&mut self.shielded);

            // tear down shield
            let reset_res = Command::new("sudo")
                .arg("cset")
                .arg("shield")
                .arg("--reset")
                .status();

            match reset_res {
                Err(why) => println!("error destroying shield: {:#?}", why),
                _ => (),
            };

            Ok(result)
        } else {
            Ok(f(&mut self.shielded))
        }
    }

    pub fn output(&mut self) -> Result<Output> {
        Ok(self.maybe_with_shielded(|cmd| cmd.output())??)
    }

    pub fn status(&mut self) -> Result<ExitStatus> {
        Ok(self.maybe_with_shielded(|cmd| cmd.status())??)
    }
}

impl Deref for RenameThisCommandWrapper {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.shielded
    }
}

impl DerefMut for RenameThisCommandWrapper {
    fn deref_mut(&mut self) -> &mut Command {
        &mut self.shielded
    }
}
