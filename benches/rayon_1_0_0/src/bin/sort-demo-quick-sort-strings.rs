//{"name":"sort :: demo_quick_sort_strings","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[153,176,92,76,26,130,10,89,222,209,246,248,131,112,83,113,49,86,191,129,152,154,38,124,111,190,66,16,14,146,20,13]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: sort :: demo_quick_sort_strings ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: sort :: demo_quick_sort_strings ( & mut crit ) ; }