use std::ffi::OsStr;
use std::io;
#[cfg(not(target_os = "linux"))]
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Output, Stdio};

#[cfg(not(target_os = "linux"))]
pub struct RenameThisCommandWrapper(Command);

#[cfg(not(target_os = "linux"))]
impl RenameThisCommandWrapper {
    pub fn new<S: AsRef<OsStr>>(cmd: S) -> Self {
        RenameThisCommandWrapper(Command::new(cmd))
    }
}

#[cfg(not(target_os = "linux"))]
impl Deref for RenameThisCommandWrapper {
    type Target = Command;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(not(target_os = "linux"))]
impl DerefMut for RenameThisCommandWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(target_os = "linux")]
pub struct RenameThisCommandWrapper {
    shielded: Command,
    cpu_mask: Option<String>,
    kthread_on: bool,
}

#[cfg(target_os = "linux")]
impl RenameThisCommandWrapper {
    pub fn new<S: AsRef<OsStr>>(cmd: S) -> Self {
        let mut shielded = Command::new("cset");
        shielded.arg("sh");
        shielded.arg(cmd);

        Self {
            shielded: shielded,
            cpu_mask: None,
            kthread_on: false,
        }
    }

    pub fn cpu_mask<S: AsRef<str>>(&mut self, mask: S) -> &mut Self {
        self.cpu_mask = Some(String::from(mask.as_ref()));
        self
    }

    pub fn move_kthreads(&mut self, move_them: bool) -> &mut Self {
        self.kthread_on = move_them;
        self
    }

    pub fn args<I, S>(&mut self, _args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        unimplemented!();
        // self.cmd.args(args);
        // self.shielded.args(args);
        // self
    }
    pub fn env<K, V>(&mut self, _key: K, _val: V) -> &mut Self
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        unimplemented!();
        // self.cmd.env(key, val);
        // self.shielded.env(key, val);
        // self
    }
    pub fn envs<I, K, V>(&mut self, _vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        unimplemented!();
        // self.cmd.envs(vars);
        // self.shielded.envs(vars);
        // self
    }

    pub fn env_remove<K: AsRef<OsStr>>(&mut self, _key: K) -> &mut Self {
        unimplemented!();
        // self.cmd.env_remove(key.as_ref());
        // self.shielded.env_remove(key);
        // self
    }

    pub fn env_clear(&mut self) -> &mut Self {
        unimplemented!();
        // self.cmd.env_clear();
        // self.shielded.env_clear();
        // self
    }

    pub fn current_dir<P: AsRef<Path>>(&mut self, _dir: P) -> &mut Self {
        unimplemented!();
        // self.cmd.current_dir(dir.as_ref());
        // self.shielded.current_dir(dir);
        // self
    }

    pub fn stdin<T: Into<Stdio>>(&mut self, _cfg: T) -> &mut Self {
        unimplemented!();
        // self.cmd.stdin(cfg);
        // self.shielded.stdin(cfg);
        // self
    }

    pub fn stdout<T: Into<Stdio>>(&mut self, _cfg: T) -> &mut Self {
        unimplemented!();
        // self.cmd.stdout(cfg);
        // self.shielded.stdout(cfg);
        // self
    }

    pub fn stderr<T: Into<Stdio>>(&mut self, _cfg: T) -> &mut Self {
        unimplemented!();
        // self.cmd.stderr(cfg);
        // self.shielded.stderr(cfg);
        // self
    }

    #[cfg(target_os = "linux")]
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

    #[cfg(target_os = "linux")]
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
            unimplemented!();
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
            unimplemented!();
        }
    }
}
