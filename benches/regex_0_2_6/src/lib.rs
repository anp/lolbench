extern crate criterion;
extern crate regex;
extern crate regex_syntax;
#[macro_use]
extern crate wrap_libtest;

#[macro_use]
mod bench;
pub mod misc;
pub mod regexdna;
pub mod rust_compile;
pub mod rust_parse;
pub mod sherlock;
