pub mod matmul;
pub mod mergesort;
pub mod nbody;
pub mod quicksort;
pub mod sieve;
pub mod tsp;
pub mod life;

// these are not "full-fledged" benchmarks yet,
// they only run with cargo bench
pub mod map_collect;
pub mod vec_collect;
pub mod factorial;
pub mod pythagoras;
pub mod fibonacci;
pub mod find;
pub mod join_microbench;
pub mod str_split;
pub mod sort;

extern crate cgmath; // nbody
extern crate criterion;
extern crate fixedbitset; // tsp
#[macro_use]
extern crate glium; // nbody
#[macro_use]
extern crate lazy_static; // find
extern crate num;         // factorial
extern crate odds;        // sieve
extern crate rand;        // nbody
extern crate rayon;       // all
extern crate regex;       // tsp
extern crate serde;       // all
#[macro_use]
extern crate serde_derive; // all
extern crate time;        // nbody, sieve
#[macro_use]
extern crate wrap_libtest;
