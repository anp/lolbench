//{"name":"misc :: short_haystack_100x","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[214,135,44,232,220,211,189,66,194,223,92,219,233,236,98,168,78,95,189,255,100,189,43,119,170,73,136,159,21,68,218,236]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: short_haystack_100x ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: short_haystack_100x ( & mut crit ) ; }