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

extern crate rayon; // all
extern crate criterion;
#[macro_use]
extern crate serde_derive; // all
extern crate serde; // all
extern crate cgmath; // nbody
#[macro_use]
extern crate glium; // nbody
extern crate rand; // nbody
extern crate time; // nbody, sieve
extern crate odds; // sieve
extern crate num; // factorial
#[macro_use]
extern crate lazy_static; // find
extern crate fixedbitset; // tsp
extern crate regex; // tsp
#[macro_use]
extern crate wrap_libtest;
