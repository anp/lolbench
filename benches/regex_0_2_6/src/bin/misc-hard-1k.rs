//{"name":"misc :: hard_1K","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[142,112,31,132,206,238,242,44,43,21,249,231,148,28,179,216,182,159,211,70,215,255,44,244,15,4,148,91,177,91,132,159]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: hard_1K ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: hard_1K ( & mut crit ) ; }