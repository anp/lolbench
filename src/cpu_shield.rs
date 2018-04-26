use std::ffi::OsStr;
use std::io;
use std::ops::{Deref, DerefMut};
use std::process::{Child, Command, ExitStatus, Output};

pub struct RenameThisCommandWrapper {
    shielded: Command,
    cpu_mask: Option<String>,
    kthread_on: bool,
}

impl RenameThisCommandWrapper {
    pub fn new<S: AsRef<OsStr>>(cmd: S) -> Self {
        let shielded = if cfg!(target_os = "linux") {
            let mut shielded = Command::new("cset");
            shielded.arg("sh");
            shielded.arg(cmd);
            shielded
        } else {
            Command::new(cmd)
        };

        Self {
            shielded: shielded,
            cpu_mask: None,
            kthread_on: false,
        }
    }

    pub fn cpu_mask<S: AsRef<str>>(&mut self, mask: S) -> &mut Self {
        if cfg!(not(target_os = "linux")) {
            unimplemented!();
        }

        self.cpu_mask = Some(String::from(mask.as_ref()));
        self
    }

    pub fn move_kthreads(&mut self, move_them: bool) -> &mut Self {
        if cfg!(not(target_os = "linux")) {
            unimplemented!();
        }

        self.kthread_on = move_them;
        self
    }

    fn setup_shield(&self) -> io::Result<ExitStatus> {
        // sudo cset shield --cpu=${CPU_MASK} --kthread=${on|off}

        // TODO notify sudo dep, find another way to do this
        let mut shield_create = Command::new("sudo");
        shield_create.arg("cset");
        shield_create.arg("shield");
        // NOTE: this should only get called if the cpu_mask has already been validated
        let mask = self.cpu_mask.iter().cloned().next().unwrap();
        shield_create.arg(format!("--cpu={}", mask));

        if self.kthread_on {
            shield_create.arg("--kthread=on");
        }

        shield_create.status()
    }

    fn teardown_shield(&self) {
        let reset_res = Command::new("sudo")
            .arg("cset")
            .arg("shield")
            .arg("--reset")
            .status();

        match reset_res {
            Err(why) => println!("error destroying shield: {:#?}", why),

            _ => (),
        };
    }

    pub fn spawn(&mut self) -> io::Result<Child> {
        unimplemented!();
    }

    pub fn output(&mut self) -> io::Result<Output> {
        if self.cpu_mask.is_some() {
            self.setup_shield()?;
            let output = self.shielded.output();
            self.teardown_shield();
            output
        } else {
            self.shielded.output()
        }
    }

    pub fn status(&mut self) -> io::Result<ExitStatus> {
        // make sure the shield is running if need be
        if self.cpu_mask.is_some() {
            self.setup_shield()?;
            // if mask provided, cset sh ${CMD}
            let status = self.shielded.status();
            self.teardown_shield();
            status
        } else {
            self.shielded.status()
        }
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
