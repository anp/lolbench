use super::Result;

use std::collections::BTreeMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};

use chrono::NaiveDateTime;
use ring::digest::{Context as RingContext, SHA256};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

use cpu_shield::ShieldSpec;
use run_plan::RunPlan;
use toolchain::Toolchain;

/// A trait which allows a struct to behave as the key in a very simple persistent K/V filesystem
/// store.
pub(crate) trait StorageKey
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

    fn get(&self, data_dir: &Path) -> Result<Option<(NaiveDateTime, Self::Contents)>> {
        let own_path = self.abs_path(data_dir);

        use std::io::ErrorKind;

        Ok(match ::std::fs::read_to_string(&own_path) {
            Ok(s) => {
                let sc: Container<Self, Self::Contents> = serde_json::from_str(&s)?;

                assert_eq!(
                    &sc.key, self,
                    "stored key doesn't match ours, even though they agree on path! it's a bug!"
                );

                Some((sc.generated_at, sc.contents))
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

    fn set(&self, data_dir: &Path, to_store: Self::Contents) -> Result<()> {
        use std::fs::File;
        use std::io::BufWriter;

        let to_write = Container {
            generated_at: ::chrono::Utc::now().naive_utc(),
            key: self.clone(),
            contents: to_store,
        };

        let own_path = self.abs_path(data_dir);
        ::std::fs::create_dir_all(&own_path.parent().unwrap())?;

        let file = File::create(&own_path)?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &to_write)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Container<K, V> {
    /// UTC
    pub generated_at: NaiveDateTime,
    pub key: K,
    pub contents: V,
}

pub(crate) enum Entry<K, T> {
    New(K, T, PathBuf),
    Existing(T),
}

impl<K, T> ::std::ops::Deref for Entry<K, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Entry::New(_, t, _) => t,
            Entry::Existing(t) => t,
        }
    }
}

impl<K, T> Entry<K, T>
where
    K: StorageKey<Contents = T> + DeserializeOwned + Serialize,
    T: DeserializeOwned + Serialize,
{
    pub fn ensure_persisted(self) -> Result<()> {
        Ok(match self {
            Entry::New(k, t, data_dir) => k.set(&data_dir, t)?,
            _ => (),
        })
    }
}

pub(crate) mod index {
    use super::*;

    #[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
    pub struct Key {
        pub benchmark_key: String,
        pub toolchain: Toolchain,
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
            slugify(self.toolchain.to_string())
        }
    }
}

pub(crate) mod measurement {
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
            use super::StorageKey;

            let tempdir = tempdir().unwrap();
            let data_dir = tempdir.path();

            let key = TestKey {
                a, b, c
            };

            let contents = contents.to_owned();
            let expected = contents.clone();

            assert_eq!(key.get(data_dir).unwrap(), None);
            key.set(data_dir, contents).unwrap();
            assert_eq!(key.get(data_dir).unwrap().unwrap().1, expected);
        }
    }

}
