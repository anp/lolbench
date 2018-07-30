#[macro_use]
extern crate proc_macro_hack;
#[macro_use]
extern crate quote;

#[allow(unused_imports)]
#[macro_use]
extern crate failure;
extern crate proc_macro2;
extern crate slug;
extern crate syn;
extern crate toml;

use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use proc_macro2::Span;
use slug::slugify;
use syn::{Ident as SynIdent, Path as SynPath};

type Result<T> = std::result::Result<T, failure::Error>;

proc_macro_expr_impl! {
    /// Prepare a single-function entry-point for
    pub fn lolbench_entrypoint_impl(bench_path: &str) -> String {
        BenchEntrypoint::new(bench_path).unwrap().write().unwrap();
        // TODO update benchmarks manifest

        format!(
            "println!(\"entering lolbench-generated benchmark {}\");",
            bench_path.trim()
        )
    }
}

#[derive(Debug)]
struct BenchEntrypoint {
    file_contents: String,
    full_path: PathBuf,
}

impl BenchEntrypoint {
    fn new(bench_path: &str) -> Result<BenchEntrypoint> {
        let fn_name: SynPath = syn::parse_str(bench_path)?;

        let crate_name = env::var("CARGO_PKG_NAME")?;
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
        let bins_dir = Path::new(&manifest_dir).join("src").join("bin");

        let crate_name_ident = SynIdent::new(crate_name.trim(), Span::call_site());

        let file_contents = quote! {
            extern crate #crate_name_ident;
            extern crate lolbench_support;

            use std::default::Default;

            use lolbench_support::{Criterion, criterion_from_env, init_logging};

            fn main() {
                init_logging();
                let mut crit = criterion_from_env();
                #crate_name_ident::#fn_name(&mut crit);
            }

            #[test]
            fn run_bench() {
                init_logging();
                let mut crit = Criterion::default();
                #crate_name_ident::#fn_name(&mut crit);
            }
        };

        let full_path = bins_dir.join(&format!("{}.rs", slugify(bench_path.to_string())));
        let file_contents = file_contents.to_string(); //rustfmt_file_contents(&file_contents.to_string())?;

        Ok(BenchEntrypoint {
            file_contents,
            full_path,
        })
    }

    fn write(&self) -> Result<()> {
        // first lets check if we would change anything on disk,
        // bc if not we don't need to write anything. this isn't important
        // for perf, but it makes cargo-watch work much better
        match read_to_string(&self.full_path) {
            Ok(ref current) if current == &self.file_contents => return Ok(()),
            _ => (),
        }

        if let Some(parent) = self.full_path.parent() {
            ::std::fs::create_dir_all(parent)?;
        }

        // TODO(anp): guarantee that rustfmt is available somehow and run it on the file
        ::std::fs::write(&self.full_path, self.file_contents.as_bytes())?;
        Ok(())
    }
}
