//{"name":"serialize_citm_struct","crate":"json_benchmark_c7d3d9b","checksum":{"method":"sha256-generic-array","value":[226,199,26,164,212,140,62,13,204,62,133,129,29,255,123,204,128,138,233,52,194,194,165,39,235,108,245,198,252,243,228,83]}}
extern crate json_benchmark_c7d3d9b ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; json_benchmark_c7d3d9b :: serialize_citm_struct ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; json_benchmark_c7d3d9b :: serialize_citm_struct ( & mut crit ) ; }