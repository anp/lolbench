//{"name":"sherlock :: word_ending_n","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[96,180,79,180,254,89,251,23,88,196,183,168,32,46,156,178,121,189,128,221,145,113,98,110,47,197,26,126,63,212,9,164]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: sherlock :: word_ending_n ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: sherlock :: word_ending_n ( & mut crit ) ; }