extern crate criterion;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate wrap_libtest;

extern crate time;

extern crate serde;
extern crate serde_json;

extern crate rustc_serialize;

#[macro_use]
pub mod enums;

pub mod adapter;
pub mod color;
pub mod empty;
pub mod prim_str;
pub mod timer;

pub mod copy;
pub use self::copy::*;

pub mod canada;

pub use std::io::{self, Read, Write};

use std::env;
use std::fs::File;

macro_rules! bench {
    {
        name: $name:expr,
        canada_dom_fn: $canada_dom_fn:ident,
        canada_struct_fn: $canada_struct_fn:ident,
        citm_dom_fn: $citm_dom_fn:ident,
        citm_struct_fn: $citm_struct_fn:ident,
        twitter_dom_fn: $twitter_dom_fn:ident,
        twitter_struct_fn: $twitter_struct_fn:ident,
        $($args:tt)*
    } => {
        bench_file! {
            path: concat!(env!("CARGO_MANIFEST_DIR"), "/data/canada.json"),
            dom_fn_name: $canada_dom_fn,
            struct_fn_name: $canada_struct_fn,
            structure: canada::Canada,
            $($args)*
        }

        bench_file! {
            path: concat!(env!("CARGO_MANIFEST_DIR"), "/data/citm_catalog.json"),
            dom_fn_name: $citm_dom_fn,
            struct_fn_name: $citm_struct_fn,
            structure: citm_catalog::CitmCatalog,
            $($args)*
        }

        bench_file! {
            path: concat!(env!("CARGO_MANIFEST_DIR"), "/data/twitter.json"),
            dom_fn_name: $twitter_dom_fn,
            struct_fn_name: $twitter_struct_fn,
            structure: twitter::Twitter,
            $($args)*
        }
    }
}

macro_rules! bench_file {
    {
        path: $path:expr,
        dom_fn_name: $dom_fn_name:ident,
        struct_fn_name: $struct_fn_name:ident,
        structure: $structure:ty,
        dom: $dom:ty,
        parse_dom: $parse_dom:expr,
        stringify_dom: $stringify_dom:expr,
        parse_struct: $parse_struct:expr,
        stringify_struct: $stringify_struct:expr,
    } => {


        wrap_libtest! {
            fn $dom_fn_name(b: &mut Bencher) {
                let contents = {
                    let mut vec = Vec::new();
                    File::open($path).unwrap().read_to_end(&mut vec).unwrap();
                    vec
                };
                b.iter(|| {
                    let dom: $dom = $parse_dom(&contents).unwrap();
                    black_box($stringify_dom(Vec::new(), &dom).unwrap());
                });
            }
        }

        wrap_libtest! {
            fn $struct_fn_name(b: &mut Bencher) {
                let contents = {
                    let mut vec = Vec::new();
                    File::open($path).unwrap().read_to_end(&mut vec).unwrap();
                    vec
                };
                b.iter(|| {
                    let parsed: $structure = $parse_struct(&contents).unwrap();
                    black_box($stringify_struct(Vec::new(), &parsed).unwrap());
                });
            }
        }

    }
}

bench! {
    name: "serde_json",
    canada_dom_fn: serde_canada_dom,
    canada_struct_fn: serde_canada_struct,
    citm_dom_fn: serde_citm_dom,
    citm_struct_fn: serde_citm_struct,
    twitter_dom_fn: serde_twitter_dom,
    twitter_struct_fn: serde_twitter_struct,
    dom: serde_json::Value,
    parse_dom: serde_json_parse_dom,
    stringify_dom: serde_json::to_writer,
    parse_struct: serde_json_parse_struct,
    stringify_struct: serde_json::to_writer,
}

bench! {
    name: "rustc_serialize",
    canada_dom_fn: serialize_canada_dom,
    canada_struct_fn: serialize_canada_struct,
    citm_dom_fn: serialize_citm_dom,
    citm_struct_fn: serialize_citm_struct,
    twitter_dom_fn: serialize_twitter_dom,
    twitter_struct_fn: serialize_twitter_struct,
    dom: rustc_serialize::json::Json,
    parse_dom: rustc_serialize_parse_dom,
    stringify_dom: rustc_serialize_stringify,
    parse_struct: rustc_serialize_parse_struct,
    stringify_struct: rustc_serialize_stringify,
}

fn serde_json_parse_dom(bytes: &[u8]) -> serde_json::Result<serde_json::Value> {
    use std::str;
    let s = str::from_utf8(bytes).unwrap();
    serde_json::from_str(s)
}

fn serde_json_parse_struct<'de, T>(bytes: &'de [u8]) -> serde_json::Result<T>
where
    T: serde::Deserialize<'de>,
{
    use std::str;
    let s = str::from_utf8(bytes).unwrap();
    serde_json::from_str(s)
}

fn rustc_serialize_parse_dom(
    mut bytes: &[u8],
) -> Result<rustc_serialize::json::Json, rustc_serialize::json::BuilderError> {
    rustc_serialize::json::Json::from_reader(&mut bytes)
}

fn rustc_serialize_parse_struct<T>(bytes: &[u8]) -> rustc_serialize::json::DecodeResult<T>
where
    T: rustc_serialize::Decodable,
{
    use std::str;
    let s = str::from_utf8(bytes).unwrap();
    rustc_serialize::json::decode(s)
}

fn rustc_serialize_stringify<W, T: ?Sized>(
    writer: W,
    value: &T,
) -> rustc_serialize::json::EncodeResult<()>
where
    W: Write,
    T: rustc_serialize::Encodable,
{
    let mut writer = adapter::IoWriteAsFmtWrite::new(writer);
    let mut encoder = rustc_serialize::json::Encoder::new(&mut writer);
    value.encode(&mut encoder)
}
