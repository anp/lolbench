//{"name":"sort :: par_sort_ascending","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[66,1,125,204,61,89,195,160,159,235,73,217,225,208,29,152,101,70,193,199,3,32,166,145,1,87,228,44,124,37,199,130]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: sort :: par_sort_ascending ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: sort :: par_sort_ascending ( & mut crit ) ; }