//{"name":"misc :: short_haystack_10000x","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[142,169,149,165,73,63,103,247,127,108,108,91,249,215,204,168,23,207,119,170,157,126,154,31,77,196,62,76,189,151,226,1]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: short_haystack_10000x ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: short_haystack_10000x ( & mut crit ) ; }