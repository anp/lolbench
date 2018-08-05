#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;

extern crate chrono;
extern crate clap;
extern crate glob;
extern crate lolbench_support;
extern crate marky_mark;
extern crate serde;
extern crate serde_json;
extern crate simple_logger;

pub mod benchmark;
mod cli;
pub mod cpu_shield;
pub mod plan;

pub mod prelude {
    pub use super::cli::*;
    pub use super::cpu_shield::*;
    pub use super::plan::*;

    pub use lolbench_support::Result;

    pub use std::collections::BTreeSet;
    pub use std::path::{Path, PathBuf};

    pub use chrono::prelude::*;
    pub use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
}

use prelude::*;

use structopt::StructOpt;

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    cli::Cli::from_args().exec()
}
