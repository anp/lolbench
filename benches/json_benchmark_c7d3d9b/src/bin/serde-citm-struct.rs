extern crate json_benchmark_c7d3d9b ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ;
json_benchmark_c7d3d9b :: serde_citm_struct ( & mut crit ) ; }