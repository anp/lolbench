//{"name":"sort :: par_sort_unstable_expensive","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[245,38,223,170,95,31,138,121,207,254,143,178,243,37,20,179,151,117,244,104,97,201,213,8,64,73,80,84,206,228,228,191]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: sort :: par_sort_unstable_expensive ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: sort :: par_sort_unstable_expensive ( & mut crit ) ; }