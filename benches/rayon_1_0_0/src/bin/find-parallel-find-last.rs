//{"name":"find :: parallel_find_last","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[244,198,227,45,167,255,243,0,21,21,64,213,176,62,176,66,90,24,232,58,54,5,206,133,215,225,51,211,151,55,95,83]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: find :: parallel_find_last ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: find :: parallel_find_last ( & mut crit ) ; }