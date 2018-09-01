use super::Result;

use std::path::Path;

const MANIFEST: &str = r#"[package]
name = "{{ crate_name }}"
version = "0.1.0"
authors = []

[dependencies]
lolbench_support = { path = "../../support" }

[dev-dependencies]
lolbench = { path = "../../" }
"#;

const LIB_RS: &str = r#"#[macro_use]
extern crate lolbench_support;

wrap_libtest! {
    fn example(b: &mut Bencher) {
        println!("Put any expensive setup code here.");

        b.iter(|| {
            println!("Put the code to measure here.");
        });
    }
}
"#;

pub fn generate_new_benchmark_crate(name: &str) -> Result<()> {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("benches")
        .join(name);

    let manifest_path = crate_dir.join("Cargo.toml");
    let src_dir = crate_dir.join("src");
    let source_path = src_dir.join("lib.rs");

    let manifest_contents = MANIFEST.replace("{{ crate_name }}", name);

    use std::fs;
    fs::create_dir_all(&src_dir)?;
    fs::write(&manifest_path, &manifest_contents)?;
    fs::write(&source_path, LIB_RS)?;
    Ok(())
}
