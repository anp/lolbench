//{"name":"sherlock :: name_sherlock_holmes","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[32,187,28,77,10,229,68,45,96,156,129,169,84,128,83,240,26,95,92,59,171,36,250,86,94,226,243,7,39,81,153,104]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: sherlock :: name_sherlock_holmes ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: sherlock :: name_sherlock_holmes ( & mut crit ) ; }