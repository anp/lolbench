#![recursion_limit = "256"]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate serde_derive;

extern crate digest;
extern crate fs2;
extern crate generic_array;
extern crate proc_macro2;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate syn;
extern crate toml;

#[cfg(test)]
extern crate tempfile;

use std::collections::BTreeMap;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::{Path, PathBuf};

use failure::*;
use fs2::FileExt;
use proc_macro2::Span;
use regex::Regex;
use syn::{Ident as SynIdent, Path as SynPath};

type Result<T> = ::std::result::Result<T, failure::Error>;

macro_rules! ecx {
    ($ctx:expr, $inner:expr) => {
        $inner.with_context(|e| format!(concat!($ctx, ": {}"), e))
    };
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct Benchmark {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner: Option<String>,
    pub name: String,
    #[serde(rename = "crate")]
    pub crate_name: String,
    pub entrypoint_path: PathBuf,
}

impl Benchmark {
    pub fn new(crate_name: &str, name: &str, path: &Path) -> Self {
        let mut n = Self {
            name: name.to_string(),
            crate_name: crate_name.to_string(),
            runner: None,
            entrypoint_path: path.to_owned(),
        };
        n.strip();
        n
    }

    pub fn strip(&mut self) {
        lazy_static! {
            static ref WHITESPACE: Regex = Regex::new("[[:space:]]+").unwrap();
        }

        let crate_name = WHITESPACE.replace_all(&self.crate_name, "").to_string();
        let name = WHITESPACE.replace_all(&self.name, "").to_string();
        self.crate_name = crate_name;
        self.name = name;

        let relative_bin_path = self
            .entrypoint_path
            .strip_prefix(&*LOLBENCH_ROOT)
            .map(Path::to_path_buf)
            .ok();

        if let Some(p) = relative_bin_path {
            self.entrypoint_path = p;
        }
    }

    pub fn set_runner(&mut self, runner: &str) {
        self.runner = Some(runner.to_owned());
    }

    pub fn key(&self) -> String {
        format!("{}::{}", self.crate_name, self.name)
    }

    fn source(&self) -> String {
        let name_syn: SynPath = syn::parse_str(&self.name).unwrap();
        let crate_name_syn = SynIdent::new(&self.crate_name, Span::call_site());

        let source = quote! {
            extern crate #crate_name_syn;
            extern crate lolbench_support;

            use lolbench_support::{criterion_from_env, init_logging};

            fn main() {
                init_logging();
                let mut crit = criterion_from_env();
                #crate_name_syn::#name_syn(&mut crit);
            }
        };

        source.to_string()

        // TODO(anp): guarantee that rustfmt is available somehow and run it on the file
    }

    pub fn write_and_register(&mut self, full_path: &Path) -> Result<bool> {
        self.strip();
        let (mut registry, _f) = Registry::from_disk()?;
        ecx!("Updating registry", registry.update(self))?;
        write_if_changed(&self.source(), &full_path)
    }

    /// Like absorb, but a typo that's fun.
    fn absorg(&mut self, other: &Self) {
        macro_rules! assign_opt {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field.clone();
                }
            };
        }

        assign_opt!(runner);
    }
}

pub fn test_source(bench_name: &str, crate_name: &str) -> String {
    let source = quote! {
        extern crate lolbench;

        #[test]
        fn end_to_end() {
            lolbench::end_to_end_test(
                #crate_name,
                #bench_name,
            );
        }
    };

    source.to_string()
}

pub fn write_if_changed(file_contents: &str, test_path: &Path) -> Result<bool> {
    let need_to_write = match read_to_string(&test_path) {
        Ok(existing) => existing != file_contents,
        _ => true,
    };

    if need_to_write {
        create_dir_all(test_path.parent().unwrap())?;
        write(&test_path, file_contents.as_bytes())?;
    }

    Ok(need_to_write)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Registry {
    pub workers: Vec<String>,
    pub benchmarks: BTreeMap<String, Benchmark>,
}

lazy_static! {
    static ref LOLBENCH_ROOT: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();
    static ref REGISTRY_TOML: PathBuf = LOLBENCH_ROOT.join("registry.toml");
}

use std::fs::File;

impl Registry {
    pub fn from_disk() -> Result<(Self, File)> {
        use std::io::prelude::*;

        let mut reg_file = ecx!(
            "Opening benchmark registry file",
            ::std::fs::File::open(&*REGISTRY_TOML)
        )?;

        ecx!("Locking registry file", reg_file.lock_exclusive())?;
        let mut self_str = String::new();

        ecx!(
            "Reading registry file",
            reg_file.read_to_string(&mut self_str)
        )?;

        Ok((
            ecx!(
                "Parsing benchmark registry from disk contents",
                ::toml::from_str(&self_str)
            )?,
            reg_file,
        ))
    }

    pub fn benches(&self) -> Vec<Benchmark> {
        self.benchmarks.values().cloned().collect()
    }

    fn update(&mut self, benchmark: &Benchmark) -> Result<()> {
        self.benchmarks
            .entry(benchmark.key())
            .and_modify(|b| b.absorg(benchmark))
            .or_insert_with(|| benchmark.clone());
        self.write()
    }

    pub fn write(&self) -> Result<()> {
        let contents = ecx!(
            "Serializing benchmark registry to TOML",
            ::toml::to_string_pretty(self)
        )?;

        ecx!(
            "Writing benchmark registry to disk",
            write_if_changed(&contents, &*REGISTRY_TOML)
        )?;

        Ok(())
    }
}
