//{"name":"misc :: short_haystack_4x","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[40,13,102,237,222,22,151,140,121,16,66,148,175,107,78,205,149,168,35,114,238,162,219,182,123,222,23,79,234,214,151,187]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: short_haystack_4x ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: short_haystack_4x ( & mut crit ) ; }