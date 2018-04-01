//! A solver for the Travelling Salesman Problem.
//!
//! Based on code developed at ETH by Christoph von Praun, Florian
//! Schneider, Nicholas Matsakis, and Thomas Gross.

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

mod bench;
pub use self::bench::*;
mod graph;
mod parser;
mod solver;
mod step;
mod tour;
mod weight;

use self::graph::{Graph, Node};
use self::solver::SolverCx;

fn run_solver(datafile: &Path, seq_threshold: usize, from: usize) -> Result<(), ()> {
    let graph = match parse_solver(datafile) {
        Ok(g) => g,
        Err(e) => {
            println!("failed to parse `{}`: {}", datafile.display(), e);
            return Err(());
        }
    };

    println!("Graph size   : {} nodes.", graph.num_nodes());
    println!("Seq threshold: {} nodes.", seq_threshold);

    if from >= graph.num_nodes() {
        println!("Invalid node index given for `--from`: {}", from);
        return Err(());
    }

    let mut solver = SolverCx::new(&graph, seq_threshold);
    let par_start = Instant::now();
    solver.search_from(Node::new(from));
    let par_time = par_start.elapsed();

    let (path, weight) = solver.into_result();

    println!("Total search time: {:?}", par_time);
    if let Some(path) = path {
        println!("Cheapest path cost: {}", weight.to_usize());
        let mut output = format!("Cheapest path:");
        for node in path {
            output.push_str(&format!(" {}", node.index()));
        }
        println!("{}", output);
    } else {
        println!("No path found.");
    }

    Ok(())
}

fn parse_solver(datafile: &Path) -> Result<Graph, Box<Error>> {
    let mut file = File::open(datafile)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    let graph = parser::parse_tsp_data(&text)?;
    Ok(graph)
}
