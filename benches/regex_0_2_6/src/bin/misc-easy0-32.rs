//{"name":"misc :: easy0_32","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[80,137,251,252,87,104,0,112,181,64,68,254,176,206,222,150,83,0,60,220,163,204,204,231,84,195,37,233,37,141,105,199]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: easy0_32 ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: easy0_32 ( & mut crit ) ; }