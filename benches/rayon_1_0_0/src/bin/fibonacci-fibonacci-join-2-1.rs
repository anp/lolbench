//{"name":"fibonacci :: fibonacci_join_2_1","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[145,62,12,241,183,246,162,111,83,76,6,53,144,248,143,55,185,231,60,250,52,239,80,133,55,231,71,127,111,81,125,182]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: fibonacci :: fibonacci_join_2_1 ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: fibonacci :: fibonacci_join_2_1 ( & mut crit ) ; }