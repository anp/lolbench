//{"name":"misc :: easy0_1K","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[28,87,95,245,54,66,13,203,125,113,207,73,235,157,26,153,130,112,126,58,127,150,67,182,26,65,251,199,164,11,47,214]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: easy0_1K ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: easy0_1K ( & mut crit ) ; }