//{"name":"regexdna :: subst9","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[214,126,225,111,190,218,249,76,137,251,90,89,61,42,92,75,218,32,228,185,40,68,96,71,14,213,122,45,220,82,41,40]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: regexdna :: subst9 ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: regexdna :: subst9 ( & mut crit ) ; }