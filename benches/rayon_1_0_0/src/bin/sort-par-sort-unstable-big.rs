//{"name":"sort :: par_sort_unstable_big","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[200,243,70,38,250,39,104,39,3,43,152,253,128,238,4,139,125,98,68,253,18,5,210,223,28,199,58,153,68,43,135,172]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: sort :: par_sort_unstable_big ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: sort :: par_sort_unstable_big ( & mut crit ) ; }