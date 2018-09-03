use super::Result;

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, ErrorKind};
use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use failure::ResultExt;
use git2::{Oid, Repository, StashFlags};
use ring::digest::{Context as RingContext, SHA256};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

use cpu_shield::ShieldSpec;
use run_plan::RunPlan;
use toolchain::Toolchain;

pub struct GitStore {
    path: PathBuf,
    repo: Repository,
}

impl GitStore {
    pub fn ensure_initialized(at: impl AsRef<Path>) -> Result<Self> {
        info!(
            "ensuring {} is a git repository and opening it.",
            at.as_ref().display()
        );
        let repo = match Repository::open(at.as_ref()) {
            Ok(r) => r,
            Err(_) => {
                let mut repo = Repository::init(at.as_ref())?;
                let output = ::std::process::Command::new("git")
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

    pub fn commit(&self, msg: &str) -> Result<Oid> {
        info!("committing current changes with message '{}'", msg);
        debug!("fetching repository's default signer");
        let signer = self.repo.signature()?;

        debug!("opening the repository's index to add files");
        let mut index = self.repo.index()?;
        debug!("adding all files in the repository to the index");
        index.add_all(&[self.path.join("*")], Default::default(), None)?;
        debug!("writing changed index to tree");
        let raw_oid = index.write_tree()?;

        debug!("finding actual tree from returned oid");
        let commit_tree = self.repo.find_tree(raw_oid)?;
        debug!("finding repo head and converting reference to commit");
        let parent = self.repo.head()?.peel_to_commit()?;

        debug!("doing actual commit and people say git cli has no ux");
        let oid = self.repo.commit(
            Some("HEAD"),
            &signer,
            &signer,
            msg,
            &commit_tree,
            &[&parent],
        )?;
        Ok(oid)
    }

    fn stash(&mut self) -> Result<()> {
        info!("stashing git storage's working directory");
        let stasher = self.repo.signature()?;
        self.repo
            .stash_save(
                &stasher,
                "stashing untracked changes in data dir",
                Some(StashFlags::INCLUDE_UNTRACKED),
            )
            .with_context(|e| format!("unable to stash changes: {}", e))?;
        Ok(())
    }

    /// `git pull --rebase`
    fn pull(&self) -> Result<()> {
        ensure!(
            ::std::process::Command::new("git")
                .arg("pull")
                .arg("--rebase")
                .current_dir(&self.path)
                .status()?
                .success(),
            "unable to pull from data directory's origin"
        );
        Ok(())
    }

    /// `git push`
    fn push(&self) -> Result<()> {
        ensure!(
            ::std::process::Command::new("git")
                .arg("push")
                .current_dir(&self.path)
                .status()?
                .success(),
            "unable to push data to data directory's origin"
        );
        Ok(())
    }

    pub fn sync(&mut self) -> Result<()> {
        info!("ensuring repo is sync'd with origin if it exists");
        if self
            .repo
            .remotes()?
            .iter()
            .find(|&r| r == Some("origin"))
            .is_some()
        {
            info!("synchronizing git storage with origin");
            self.stash()?;
            info!("pulling from remote");
            self.pull()?;
            info!("pushing to remote");
            self.push()?;
            info!("done synchronizing");
        } else {
            warn!("git storage does not have an origin remote, won't do any remote sync'ing.");
        }

        Ok(())
    }

    pub fn get<K: StorageKey>(&mut self, key: &K) -> Result<Option<K::Contents>> {
        let own_path = key.abs_path(&self.path);

        self.sync()?;

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
        self.sync()?;

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

        self.commit("setting key in storage (adam make this message better!)")?;
        self.sync()?;

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

    fn parents(&self) -> Vec<String>;
    fn basename(&self) -> String;

    fn abs_path(&self, data_dir: &Path) -> PathBuf {
        let mut path = data_dir.to_owned();

        for p in self.parents() {
            path.push(p);
        }

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

pub enum Entry<K, T> {
    New(K, T),
    Existing(T),
}

impl<K, T> ::std::ops::Deref for Entry<K, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Entry::New(_, t) => t,
            Entry::Existing(t) => t,
        }
    }
}

impl<K, T> Entry<K, T>
where
    K: StorageKey<Contents = T> + DeserializeOwned + Serialize,
    T: DeserializeOwned + Serialize,
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
        type Contents = Vec<u8>;

        fn parents(&self) -> Vec<String> {
            vec![String::from("run-plans"), slugify(&self.benchmark_key)]
        }

        fn basename(&self) -> String {
            slugify(
                self.toolchain
                    .as_ref()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| String::from("current-toolchain")),
            )
        }
    }
}

pub mod measurement {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
    pub struct Key {
        binary_hash: Vec<u8>,
        runner: String,
        cpu_shield: Option<ShieldSpec>,
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
        type Contents = ::std::result::Result<Estimates, ::collector::Error>;

        fn parents(&self) -> Vec<String> {
            vec![String::from("measurements")]
        }

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
    mean: Estimate,
    #[serde(rename = "Median")]
    median: Estimate,
    #[serde(rename = "MedianAbsDev")]
    median_abs_dev: Estimate,
    #[serde(rename = "Slope")]
    slope: Estimate,
    #[serde(rename = "StdDev")]
    std_dev: Estimate,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
struct ConfidenceInterval {
    confidence_level: f64,
    lower_bound: f64,
    upper_bound: f64,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
struct Estimate {
    /// The confidence interval for this estimate
    confidence_interval: ConfidenceInterval,
    ///
    point_estimate: f64,
    /// The standard error of this estimate
    standard_error: f64,
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

        fn parents(&self) -> Vec<String> {
            vec![self.a.to_string(), self.b.to_string()]
        }

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
