//{"name":"fibonacci :: fibonacci_split_recursive","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[137,158,41,98,181,54,77,81,21,44,29,122,53,215,142,3,250,220,218,241,39,185,221,217,56,12,149,199,81,186,225,218]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: fibonacci :: fibonacci_split_recursive ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: fibonacci :: fibonacci_split_recursive ( & mut crit ) ; }