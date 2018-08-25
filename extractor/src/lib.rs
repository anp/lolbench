#[macro_use]
extern crate proc_macro_hack;

extern crate failure;
extern crate marky_mark;
extern crate slug;

use std::env;
use std::path::Path;

use marky_mark::*;
use slug::slugify;

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

    let crate_name = ::std::env::var("CARGO_PKG_NAME")?;
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;

    let bin_name = slugify(bench_path.to_string());
    let source_name = format!("{}.rs", bin_name);
    let full_path = Path::new(&manifest_dir)
        .join("src")
        .join("bin")
        .join(&source_name);

    let test_path = Path::new(&manifest_dir).join("tests").join(&source_name);
    let test_source = test_source(&bench_path, &crate_name, &bin_name);

    Benchmark::new(&crate_name, bench_path).write(&full_path)?;
    write_if_changed(&test_source, &test_path)?;

    returned
}
