//{"name":"sort :: par_sort_unstable_strings","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[94,104,31,23,1,150,159,60,228,143,196,127,16,96,103,164,177,53,229,5,156,163,97,33,77,249,93,112,244,231,240,130]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: sort :: par_sort_unstable_strings ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: sort :: par_sort_unstable_strings ( & mut crit ) ; }