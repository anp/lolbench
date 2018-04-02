use std::ffi::OsStr;
use std::io;
use std::ops::{Deref, DerefMut};
use std::process::{Child, Command, ExitStatus, Output};

pub struct RenameThisCommandWrapper {
    cmd: Command,
    cpu_mask: Option<String>,
    #[cfg_attr(not(target_os = "linux"), allow(dead_code))]
    kthread_on: bool,
}

impl RenameThisCommandWrapper {
    pub fn new<S: AsRef<OsStr>>(cmd: S) -> Self {
        // cset sh ${CMD}
        Self {
            cmd: Command::new(cmd),
            cpu_mask: None,
            kthread_on: false,
        }
    }

    pub fn cpu_mask<S: AsRef<str>>(&mut self, mask: S) -> &mut Self {
        self.cpu_mask = Some(String::from(mask.as_ref()));
        self
    }

    #[cfg(target_os = "linux")]
    fn setup_shield(&mut self) -> io::Result<()> {
        if let Some(ref mask) = self.cpu_mask {
            // cset shield --cpu=${CPU_MASK} --kthread=${on|off}

            let mut shield_create = Command::new("cset");
            shield_create.arg(format!("--cpu={}", mask));

            if self.kthread_on {
                shield_create.arg("--kthread=on");
            }
            unimplemented!("setup shield not yet implemented");
        }
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn setup_shield(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn teardown_shield(&mut self) -> io::Result<()> {
        if let Some(ref mask) = self.cpu_mask {
            Command::new("cset").arg("shield").arg("--reset").status()?;
        }
        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn teardown_shield(&mut self) -> io::Result<()> {
        Ok(())
    }

    pub fn spawn(&mut self) -> io::Result<Child> {
        unimplemented!("only status method supported");
    }

    pub fn output(&mut self) -> io::Result<Output> {
        unimplemented!("only status method supported");
    }

    pub fn status(&mut self) -> io::Result<ExitStatus> {
        self.setup_shield()?;

        // TODO run inside cpu shield
        let status = self.cmd.status();

        // TODO teardown shield
        match self.teardown_shield() {
            Ok(_) => (),
            Err(why) => (), // TODO log warning
        }

        status
    }
}

impl Deref for RenameThisCommandWrapper {
    type Target = Command;
    fn deref(&self) -> &Self::Target {
        &self.cmd
    }
}

impl DerefMut for RenameThisCommandWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cmd
    }
}
