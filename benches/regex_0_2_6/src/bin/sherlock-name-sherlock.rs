//{"name":"sherlock :: name_sherlock","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[53,65,91,41,131,122,127,42,128,27,9,49,12,111,222,206,222,93,247,80,48,36,242,213,140,242,146,184,85,69,250,22]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: sherlock :: name_sherlock ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: sherlock :: name_sherlock ( & mut crit ) ; }