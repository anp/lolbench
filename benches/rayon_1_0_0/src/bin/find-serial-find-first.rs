//{"name":"find :: serial_find_first","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[143,182,118,20,17,167,54,32,34,212,45,171,233,253,175,9,201,245,39,44,125,1,1,248,5,46,159,75,134,146,85,254]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: find :: serial_find_first ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: find :: serial_find_first ( & mut crit ) ; }