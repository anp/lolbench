//{"name":"defer :: single_defer","crate":"crossbeam_epoch_0_4_0","checksum":{"method":"sha256-generic-array","value":[79,165,216,3,10,26,15,149,249,167,158,247,46,118,48,197,243,216,212,32,90,219,112,190,107,97,60,41,93,95,85,235]}}
extern crate crossbeam_epoch_0_4_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; crossbeam_epoch_0_4_0 :: defer :: single_defer ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; crossbeam_epoch_0_4_0 :: defer :: single_defer ( & mut crit ) ; }