//{"name":"misc :: long_needle2","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[14,121,143,31,40,205,33,90,158,221,202,7,20,80,112,244,237,6,82,31,68,154,71,191,165,223,118,244,253,151,139,158]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: misc :: long_needle2 ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: misc :: long_needle2 ( & mut crit ) ; }