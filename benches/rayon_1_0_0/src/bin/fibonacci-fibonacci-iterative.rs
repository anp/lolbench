//{"name":"fibonacci :: fibonacci_iterative","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[239,165,6,32,239,139,203,200,23,232,198,187,93,254,129,132,161,236,103,135,254,98,119,160,255,27,115,30,188,144,248,203]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: fibonacci :: fibonacci_iterative ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: fibonacci :: fibonacci_iterative ( & mut crit ) ; }