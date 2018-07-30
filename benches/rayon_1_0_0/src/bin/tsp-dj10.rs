extern crate lolbench_support;
extern crate rayon_1_0_0;
use lolbench_support::{criterion_from_env, init_logging};
fn main() {
    init_logging();
    let mut crit = criterion_from_env();
    rayon_1_0_0::tsp::bench::dj10(&mut crit);
}
#[test]
fn run_bench() {
    use std::default::Default;
    init_logging();
    let mut crit = Criterion::default();
    rayon_1_0_0::tsp::bench::dj10(&mut crit);
}
