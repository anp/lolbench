//{"name":"regexdna :: variant8","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[4,215,173,211,228,159,247,129,216,135,253,196,196,120,177,143,79,206,215,103,189,41,15,75,237,111,69,167,47,226,104,187]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: regexdna :: variant8 ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: regexdna :: variant8 ( & mut crit ) ; }