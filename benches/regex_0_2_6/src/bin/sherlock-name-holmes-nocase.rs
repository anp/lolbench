//{"name":"sherlock :: name_holmes_nocase","crate":"regex_0_2_6","checksum":{"method":"sha256-generic-array","value":[164,62,232,22,244,228,33,67,112,153,165,223,61,19,194,202,205,192,29,219,129,106,225,78,253,191,206,33,19,208,41,134]}}
extern crate regex_0_2_6 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; regex_0_2_6 :: sherlock :: name_holmes_nocase ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; regex_0_2_6 :: sherlock :: name_holmes_nocase ( & mut crit ) ; }