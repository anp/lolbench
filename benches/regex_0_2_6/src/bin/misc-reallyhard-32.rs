//{"name":"misc :: reallyhard_32","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[144,195,232,246,217,158,232,133,134,127,186,135,175,98,67,140,129,45,104,206,27,148,196,127,206,81,83,98,43,186,93,229]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: reallyhard_32 ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: reallyhard_32 ( & mut crit ) ; }