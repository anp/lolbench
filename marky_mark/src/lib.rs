#![recursion_limit = "256"]
#[macro_use]
extern crate failure;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate serde_derive;

extern crate digest;
extern crate generic_array;
extern crate proc_macro2;
extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate syn;
extern crate toml;

#[cfg(test)]
extern crate tempfile;

use std::fs::{create_dir_all, read_to_string, write};
use std::path::Path;

use proc_macro2::Span;
use syn::{Ident as SynIdent, Path as SynPath};

type Result<T> = ::std::result::Result<T, failure::Error>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Benchmark {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner: Option<String>,
    pub name: String,
    #[serde(rename = "crate")]
    pub crate_name: String,
}

impl Benchmark {
    pub fn new(crate_name: &str, name: &str) -> Self {
        Self {
            name: name.to_string(),
            crate_name: crate_name.to_string(),
            runner: None,
        }
    }

    pub fn set_runner(&mut self, runner: &str) {
        self.runner = Some(runner.to_owned());
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

            #[test]
            fn run_bench() {
                use std::default::Default;
                use std::time::Duration;
                use lolbench_support::Criterion;
                init_logging();
                let mut crit = Criterion::default();

                crit = crit.sample_size(2);
                crit = crit.warm_up_time(Duration::from_micros(1));
                crit = crit.measurement_time(Duration::from_micros(1));
                crit = crit.nresamples(1);

                #crate_name_syn::#name_syn(&mut crit);
            }
        };

        source.to_string()

        // TODO(anp): guarantee that rustfmt is available somehow and run it on the file
    }

    pub fn rendered(&mut self) -> String {
        let source = self.source();
        format!("//{}\n{}", serde_json::to_string(&self).unwrap(), source)
    }

    pub fn parse(s: &str) -> Result<(Self, String)> {
        let mut lines = s.lines();

        let first_line = match lines.next() {
            Some(l) => l.trim_left_matches("//"),
            None => bail!("missing first line"),
        };

        let remaining = lines.fold(String::new(), |remaining, line| remaining + line);

        Ok((serde_json::from_str(first_line)?, remaining))
    }

    pub fn write(&mut self, full_path: &Path) -> Result<bool> {
        let mut file_contents = self.rendered();

        let mut need_to_write = true;

        // if there's an existing file for this bench's path, we need to know about two questions
        //
        // 1. is there persistent config that was written before which we need to preserve?
        // 2. can we skip writing altogether to limit disk thrash?
        if let Ok(existing_contents) = read_to_string(&full_path) {
            // for now, the only persistent config is what runner, if any, has been configured.
            // however, we don't want to preserve the last runner config if we're *currently in the
            // process of setting it*
            if self.runner.is_none() {
                // we don't care about errors here at all
                if let Ok((existing, _)) = Self::parse(&existing_contents) {
                    if let Some(r) = existing.runner {
                        self.runner = Some(r);
                    }
                }
            }

            // need to re-render the contents after potentially mutating
            file_contents = self.rendered();

            need_to_write = file_contents != existing_contents;
        }

        if need_to_write {
            create_dir_all(full_path.parent().unwrap())?;
            write(&full_path, file_contents.as_bytes())?;
        }

        Ok(need_to_write)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrips() {
        let mut header = Benchmark::new("test_crate", "test_bench");
        let rendered = header.rendered();

        let (mut parsed, remaining) = Benchmark::parse(&rendered).unwrap();
        assert_eq!(header, parsed);
        assert_eq!(remaining, header.source());

        let parsed_rendered = parsed.rendered();
        assert_eq!(rendered, parsed_rendered);
    }

    #[test]
    fn write() {
        let mut header = Benchmark::new("test_crate", "test_bench");
        let rendered = header.rendered();

        let tmpfile = tempfile::NamedTempFile::new().unwrap();
        let bench_path = tmpfile.path();
        header.write(&bench_path).unwrap();

        let written = read_to_string(&bench_path).unwrap();

        assert_eq!(rendered, written);
    }

    #[test]
    fn preserve_runner() {
        let runner = "they-call-me-tim";

        let mut header = Benchmark::new("test_crate", "test_bench");
        let mut without_runner = header.clone();
        header.set_runner(runner);

        let tmpfile = tempfile::NamedTempFile::new().unwrap();
        let bench_path = tmpfile.path();
        header.write(&bench_path).unwrap();

        let written = read_to_string(&bench_path).unwrap();
        let (written_header, _) = Benchmark::parse(&written).unwrap();

        assert_eq!(
            written_header.runner, header.runner,
            "runner should be preserved in writing"
        );

        without_runner.write(&bench_path).unwrap();
        let written = read_to_string(&bench_path).unwrap();
        let (written_header, _) = Benchmark::parse(&written).unwrap();

        let written_runner = written_header
            .runner
            .expect("runner should have been preserved");
        assert_eq!(written_runner, runner);
    }
}
