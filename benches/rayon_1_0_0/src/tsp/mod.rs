//! A solver for the Travelling Salesman Problem.
//!
//! Based on code developed at ETH by Christoph von Praun, Florian
//! Schneider, Nicholas Matsakis, and Thomas Gross.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub mod bench;
mod graph;
mod parser;
mod solver;
mod step;
mod tour;
mod weight;

use self::graph::Graph;

fn parse_solver(datafile: &Path) -> Result<Graph, Box<Error>> {
    let mut file = File::open(datafile)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let graph = parser::parse_tsp_data(&text)?;
    Ok(graph)
}
