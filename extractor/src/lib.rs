#[macro_use]
extern crate proc_macro_hack;

extern crate failure;
extern crate marky_mark;
extern crate slug;

use std::env;
use std::path::Path;

use marky_mark::Benchmark;
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
    let bins_dir = Path::new(&manifest_dir).join("src").join("bin");

    let full_path = bins_dir.join(&format!("{}.rs", slugify(bench_path.to_string())));

    Benchmark::new(&crate_name, bench_path).write(&full_path)?;

    returned
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::path::Path;
    use std::process::Command;

    use marky_mark::Benchmark;

    #[test]
    fn test_inflate() {
        let inflate_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("benches")
            .join("inflate_0_3_4");

        let output = Command::new("cargo")
            .arg("test")
            .current_dir(&inflate_dir)
            .output()
            .unwrap();

        assert!(
            output.status.success(),
            "failed to run inflate's tests: {}",
            String::from_utf8_lossy(&output.stdout),
        );

        let decode_path = inflate_dir.join("src").join("bin").join("decode.rs");
        let decode_contents = read_to_string(&decode_path).unwrap();
        Benchmark::parse(&decode_contents).unwrap();
    }
}
