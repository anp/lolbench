//{"name":"sherlock :: ing_suffix","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[14,121,58,182,83,169,192,211,189,38,85,242,129,81,103,164,199,122,174,76,23,124,239,255,185,184,41,123,19,175,167,128]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: sherlock :: ing_suffix ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: sherlock :: ing_suffix ( & mut crit ) ; }