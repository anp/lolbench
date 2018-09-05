use super::Result;

use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use chrono::NaiveDateTime;
use git2::Repository;
use ring::digest::{Context as RingContext, SHA256};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use walkdir::WalkDir;

use collector::CollectionResult;
use cpu_shield::ShieldSpec;
use run_plan::RunPlan;
use toolchain::Toolchain;

pub struct GitStore {
    path: PathBuf,
    repo: Repository,
}

impl GitStore {
    pub fn all_stored_estimates(
        &self,
    ) -> Result<BTreeMap<String, BTreeMap<Option<Toolchain>, Estimates>>> {
        info!("finding all stored estimates in {}", self.path.display());

        let measures =
            self.all_stored::<measurement::Key, <measurement::Key as StorageKey>::Contents>()?;
        let plans = self.all_stored::<index::Key, Vec<u8>>()?;

        let measures_by_binhash: BTreeMap<Vec<u8>, Estimates> = measures
            .into_iter()
            .filter_map(|sc| {
                let Container { key, contents, .. } = sc;
                contents.ok().map(|estimates| (key.binary_hash, estimates))
            })
            .collect::<BTreeMap<_, _>>();

        let mut all: BTreeMap<String, BTreeMap<Option<Toolchain>, Estimates>> = BTreeMap::new();

        for Container {
            key,
            contents: binary_hash,
            ..
        } in plans
        {
            all.entry(key.benchmark_key)
                .or_default()
                .insert(key.toolchain, measures_by_binhash[&binary_hash].clone());
        }

        Ok(all)
    }

    fn all_stored<K: StorageKey, V: DeserializeOwned>(&self) -> Result<Vec<Container<K, V>>> {
        let mut found = Vec::new();

        for e in WalkDir::new(self.path.join(K::DIRECTORY)) {
            let entry = e?;
            let epath = entry.path();

            if epath.extension() != Some(OsStr::new("json")) {
                continue;
            }

            let contents = ::std::fs::read_to_string(epath)?;
            match serde_json::from_str::<Container<K, V>>(&contents) {
                Ok(sc) => found.push(sc),
                Err(why) => {
                    warn!(
                        "tried to deserialize {} as an index::Key but failed: {:?}

                        file contents:
                        {}",
                        epath.display(),
                        why,
                        contents
                    );
                }
            }
        }

        Ok(found)
    }

    pub fn ensure_initialized(at: impl AsRef<Path>) -> Result<Self> {
        info!(
            "ensuring {} is a git repository and opening it.",
            at.as_ref().display()
        );
        let repo = match Repository::open(at.as_ref()) {
            Ok(r) => r,
            Err(_) => {
                let mut repo = Repository::init(at.as_ref())?;
                let output = Command::new("git")
                    .arg("commit")
                    .arg("--allow-empty")
                    .arg("--message")
                    .arg("initial")
                    .current_dir(at.as_ref())
                    .output()?;

                if !output.status.success() {
                    bail!(
                        "failed to make initial empty commit: {} {}",
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    );
                }

                repo
            }
        };

        Ok(Self {
            path: at.as_ref().to_owned(),
            repo,
        })
    }

    pub fn commit(&self, msg: &str) -> Result<()> {
        info!("committing current changes with message '{}'", msg);

        ensure!(
            Command::new("git")
                .arg("add")
                .arg(".")
                .current_dir(&self.path)
                .status()?
                .success(),
            "unable to stage changes in data dir"
        );

        let mut commit_child = Command::new("git")
            .arg("commit")
            .arg("-F")
            .arg("-")
            .current_dir(&self.path)
            .stdin(Stdio::piped())
            .spawn()?;

        {
            commit_child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(msg.as_bytes())?;
        }

        let output = commit_child.wait_with_output()?;

        if !output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!(
                "failed to commit changes to data directory: {} {}",
                stdout,
                stderr
            );
        }

        Ok(())
    }

    /// `git stash --include-untracked`
    fn stash(&self) -> Result<()> {
        ensure!(
            Command::new("git")
                .arg("stash")
                .arg("--include-untracked")
                .current_dir(&self.path)
                .status()?
                .success(),
            "unable to stash uncommitted changes"
        );
        Ok(())
    }

    /// `git pull --rebase`
    fn pull(&self) -> Result<()> {
        if self.has_origin()? {
            ensure!(
                Command::new("git")
                    .arg("pull")
                    .arg("--rebase")
                    .arg("origin")
                    .arg("master")
                    .current_dir(&self.path)
                    .status()?
                    .success(),
                "unable to pull from data directory's origin"
            );
        } else {
            warn!("no origin remote found, skipping pull");
        }
        Ok(())
    }

    /// `git push`
    pub fn push(&self) -> Result<()> {
        if self.has_origin()? {
            ensure!(
                Command::new("git")
                    .arg("push")
                    .arg("origin")
                    .arg("master")
                    .current_dir(&self.path)
                    .status()?
                    .success(),
                "unable to push to data directory's origin"
            );
        } else {
            warn!("no origin remote found, skipping push");
        }
        Ok(())
    }

    fn has_origin(&self) -> Result<bool> {
        Ok(self
            .repo
            .remotes()?
            .iter()
            .find(|&r| r == Some("origin"))
            .is_some())
    }

    pub fn sync_down(&mut self) -> Result<()> {
        info!("sync'ing down, first stashing uncommitted changes");
        self.stash()?;
        if self.has_origin()? {
            info!("we have an origin remote, pulling");
            self.pull()?;
            info!("done pulling from remote");
        } else {
            info!("git storage does not have an origin remote, won't do any remote sync'ing.");
        }

        Ok(())
    }

    pub fn get<K: StorageKey>(&mut self, key: &K) -> Result<Option<K::Contents>> {
        let own_path = key.abs_path(&self.path);

        Ok(match ::std::fs::read_to_string(&own_path) {
            Ok(s) => {
                let sc: Container<K, K::Contents> = serde_json::from_str(&s)?;

                assert_eq!(
                    &sc.key, key,
                    "stored key doesn't match ours, even though they agree on path! it's a bug!"
                );

                Some(sc.contents)
            }

            Err(why) => match why.kind() {
                ErrorKind::NotFound => None,
                _ => bail!(
                    "unable to find out whether {} exists: {:?}",
                    own_path.display(),
                    why
                ),
            },
        })
    }

    pub fn set<K: StorageKey>(&mut self, key: &K, value: &K::Contents) -> Result<()> {
        let to_write = Container {
            generated_at: ::chrono::Utc::now().naive_utc(),
            key: key.clone(),
            contents: value,
        };

        let own_path = key.abs_path(&self.path);
        ::std::fs::create_dir_all(&own_path.parent().unwrap())?;

        let file = File::create(&own_path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &to_write)?;

        // hopefully flush everything before we try to commit
        drop(writer);

        Ok(())
    }
}

/// A trait which allows a struct to behave as the key in a very simple persistent K/V filesystem
/// store.
pub trait StorageKey
where
    Self: Clone + Debug + PartialEq + DeserializeOwned + Serialize,
{
    type Contents: DeserializeOwned + Serialize;
    const DIRECTORY: &'static str;

    fn basename(&self) -> String;

    fn abs_path(&self, data_dir: &Path) -> PathBuf {
        let mut path = data_dir.to_owned();

        path.push(Self::DIRECTORY);
        path.push(format!("{}.json", self.basename()));
        path
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Container<K, V> {
    /// UTC
    pub generated_at: NaiveDateTime,
    pub key: K,
    pub contents: V,
}

#[derive(Clone)]
pub enum Entry<K: StorageKey> {
    New(K, K::Contents),
    Existing(K::Contents),
}

impl<K> ::std::ops::Deref for Entry<K>
where
    K: StorageKey,
{
    type Target = K::Contents;

    fn deref(&self) -> &Self::Target {
        match self {
            Entry::New(_, t) => t,
            Entry::Existing(t) => t,
        }
    }
}

impl<K> Entry<K>
where
    K: StorageKey + DeserializeOwned + Serialize,
{
    pub fn ensure_persisted(self, store: &mut GitStore) -> Result<()> {
        Ok(match self {
            Entry::New(k, t) => store.set(&k, &t)?,
            _ => (),
        })
    }
}

pub mod index {
    use super::*;

    #[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
    pub struct Key {
        pub benchmark_key: String,
        pub toolchain: Option<Toolchain>,
    }

    impl Key {
        pub fn new(rp: &RunPlan) -> Self {
            Self {
                benchmark_key: rp.benchmark.key(),
                toolchain: rp.toolchain.clone(),
            }
        }
    }

    use slug::slugify;
    impl StorageKey for Key {
        type Contents = CollectionResult<Vec<u8>>;
        const DIRECTORY: &'static str = "run-plans";

        fn basename(&self) -> String {
            slugify(format!(
                "{}-{}",
                self.benchmark_key,
                self.toolchain
                    .as_ref()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| String::from("current-toolchain"))
            ))
        }
    }
}

pub mod measurement {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
    pub struct Key {
        pub binary_hash: Vec<u8>,
        pub runner: String,
        pub cpu_shield: Option<ShieldSpec>,
    }

    impl Key {
        pub fn new(
            hash: impl Into<Vec<u8>>,
            runner: Option<String>,
            cpu_shield: Option<ShieldSpec>,
        ) -> Self {
            let runner = runner.unwrap_or_else(|| "anonymous".to_string());
            Self {
                binary_hash: hash.into(),
                runner,
                cpu_shield,
            }
        }
    }

    impl StorageKey for Key {
        type Contents = CollectionResult<Estimates>;
        const DIRECTORY: &'static str = "measurements";

        fn basename(&self) -> String {
            use std::hash::{Hash, Hasher};

            struct MyHasherWtf(::ring::digest::Context);

            impl Hasher for MyHasherWtf {
                fn finish(&self) -> u64 {
                    use byteorder::{ByteOrder, NativeEndian};
                    NativeEndian::read_u64(self.0.clone().finish().as_ref())
                }

                fn write(&mut self, bytes: &[u8]) {
                    self.0.update(bytes);
                }
            }

            let mut hasher = MyHasherWtf(RingContext::new(&SHA256));
            self.hash(&mut hasher);
            format!("{:x}", hasher.finish())
        }
    }
}

// the below is adapted from criterion

pub type Estimates = BTreeMap<String, Statistic>;

// TODO(anp): tests for this with criterion output
#[derive(Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
pub struct Statistic {
    #[serde(rename = "Mean")]
    pub mean: Estimate,
    #[serde(rename = "Median")]
    pub median: Estimate,
    #[serde(rename = "MedianAbsDev")]
    pub median_abs_dev: Estimate,
    #[serde(rename = "Slope")]
    pub slope: Estimate,
    #[serde(rename = "StdDev")]
    pub std_dev: Estimate,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
pub struct ConfidenceInterval {
    pub confidence_level: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
pub struct Estimate {
    /// The confidence interval for this estimate
    pub confidence_interval: ConfidenceInterval,
    ///
    pub point_estimate: f64,
    /// The standard error of this estimate
    pub standard_error: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    struct TestKey {
        a: u8,
        b: u8,
        c: u8,
    }

    impl StorageKey for TestKey {
        type Contents = Vec<String>;
        const DIRECTORY: &'static str = "fivef";

        fn basename(&self) -> String {
            self.c.to_string()
        }
    }

    proptest! {
        #[test]
        fn roundtrips(
            a in 0..255u8,
            b in 0..255u8,
            c in 0..255u8,
            ref contents in ::proptest::collection::vec(".*", 1..100)
        ) {
            let tempdir = tempdir().unwrap();
            let data_dir = tempdir.path();

            let mut storage = GitStore::ensure_initialized(data_dir).unwrap();

            let key = TestKey {
                a, b, c
            };

            let contents = contents.to_owned();
            let expected = contents.clone();

            assert_eq!(storage.get(&key).unwrap(), None);
            storage.set(&key, &contents).unwrap();
            assert_eq!(storage.get(&key).unwrap().unwrap(), expected);
        }
    }

}
