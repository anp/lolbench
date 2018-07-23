use super::*;

use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::Path;

use quote::ToTokens;
use toml::Value;
use walkdir::WalkDir;

pub fn extract_and_write_crate(krate_path: &Path, output_path: &Path) -> Result<()> {
    // get the contents of the targeted crate's manifest
    let manifest_contents = read_to_string(krate_path.join("Cargo.toml"))?;

    // find the name of the crate from Cargo.toml
    let crate_name = match toml::from_str(&manifest_contents)? {
        Value::Table(t) => t["package"]["name"].clone(),
        _ => bail!("couldn't parse Cargo.toml"),
    }.as_str()
        .unwrap()
        .to_owned();

    // collect all of the exported benchmark functions
    let mut functions = Vec::new();

    for path in WalkDir::new(krate_path)
        .into_iter()
        .filter_map(|e| -> Option<PathBuf> {
            let entry = e.ok()?;

            // only try to parse *files* which have an `rs` extension
            match (entry.file_type().is_file(), entry.path()) {
                (true, p) if p.extension() == Some(OsStr::new("rs")) => Some(p.to_owned()),
                _ => None,
            }
        }) {
        println!("reading {:?}", &path);
        let contents = read_to_string(&path)?;
        println!("parsing {:?}", path);
        functions.extend(benchmark_fns_from_file(&contents)?);
    }

    for bench_fn in functions {
        println!("building cli entry point for {}", &bench_fn);

        let crate_name = syn::Ident::new(&crate_name, proc_macro2::Span::call_site());

        let file_contents = quote! {
            #[macro_use]
            extern crate lolbench;
            lolbench!( #crate_name, #bench_fn );
        };

        let krate = crate_name.clone();
        let filename = format!("{k}_{f}.rs", k = krate, f = bench_fn);
        let to_write = file_contents.to_string();

        ::std::fs::write(output_path.join(filename), to_write)?;
    }

    Ok(())
}

fn benchmark_fns_from_file(contents: &str) -> Result<Vec<syn::Ident>> {
    use syn::*;
    let file_ast = parse_file(contents)?;

    let mut macro_calls = Vec::new();
    add_all_macro_items(&file_ast.items, &mut macro_calls);

    Ok(macro_calls
        .iter()
        .map(|im| im.mac.clone())
        .filter(|m| {
            m.path
                .clone()
                .into_token_stream()
                .to_string()
                .contains("wrap_libtest")
        })
        .map(|m| syn::parse2::<syn::ItemFn>(m.tts.clone()))
        .collect::<::std::result::Result<Vec<_>, _>>()?
        .into_iter()
        .map(|f| f.ident)
        .collect())
}

use syn::{Item, ItemMacro, ItemMod};

fn add_all_macro_items(items: &[Item], macros: &mut Vec<ItemMacro>) {
    for item in items {
        use syn::Item::*;
        match item {
            Macro(m) => macros.push(m.clone()),

            Mod(ItemMod {
                content: Some((_, items)),
                ..
            }) => add_all_macro_items(&items, macros),

            // pretty sure we aren't going to find ItemMacros anywhere else in the ast
            _ => (),
        }
    }
}
