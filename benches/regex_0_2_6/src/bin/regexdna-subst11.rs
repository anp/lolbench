//{"name":"regexdna :: subst11","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[22,108,95,67,195,8,227,162,7,8,43,175,19,154,224,238,35,250,80,210,87,124,215,73,88,207,8,18,130,158,81,86]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: regexdna :: subst11 ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: regexdna :: subst11 ( & mut crit ) ; }