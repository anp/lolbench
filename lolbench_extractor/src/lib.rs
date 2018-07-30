#![recursion_limit = "256"]
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
use std::path::Path;

use proc_macro2::Span;
use slug::slugify;
use syn::{Ident as SynIdent, Path as SynPath};

type Result<T> = std::result::Result<T, failure::Error>;

proc_macro_expr_impl! {
    /// Prepare and write to disk a single-function CLI entry-point for the passed benchmark fn.
    pub fn lolbench_entrypoint_impl(bench_path: &str) -> String {
        lolbench_entrypoint(bench_path).unwrap()
    }
}

fn lolbench_entrypoint(bench_path: &str) -> Result<String> {
    // TODO update benchmarks manifest
    let returned = Ok(format!(
        "println!(\"entering lolbench-generated benchmark {}\");",
        bench_path.trim()
    ));

    let fn_name: SynPath = syn::parse_str(bench_path)?;

    let crate_name = env::var("CARGO_PKG_NAME")?;
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let bins_dir = Path::new(&manifest_dir).join("src").join("bin");

    let crate_name_ident = SynIdent::new(crate_name.trim(), Span::call_site());

    let file_contents = quote! {
        extern crate #crate_name_ident;
        extern crate lolbench_support;

        use lolbench_support::{criterion_from_env, init_logging};

        fn main() {
            init_logging();
            let mut crit = criterion_from_env();
            #crate_name_ident::#fn_name(&mut crit);
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

            #crate_name_ident::#fn_name(&mut crit);
        }
    };

    let full_path = bins_dir.join(&format!("{}.rs", slugify(bench_path.to_string())));
    let file_contents = file_contents.to_string();

    // first lets check if we would change anything on disk,
    // bc if not we don't need to write anything. this isn't important
    // for perf, but it makes cargo-watch work much better
    match read_to_string(&full_path) {
        Ok(ref current) if current == &file_contents => return returned,
        _ => (),
    }

    if let Some(parent) = full_path.parent() {
        ::std::fs::create_dir_all(parent)?;
    }

    // TODO(anp): guarantee that rustfmt is available somehow and run it on the file
    ::std::fs::write(&full_path, file_contents.as_bytes())?;
    returned
}
