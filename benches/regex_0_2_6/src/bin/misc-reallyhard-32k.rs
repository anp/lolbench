//{"name":"misc :: reallyhard_32K","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[219,148,128,231,88,123,9,48,155,184,51,68,206,56,191,196,26,201,174,54,92,185,111,146,235,28,228,37,133,182,245,62]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: reallyhard_32K ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: reallyhard_32K ( & mut crit ) ; }