//{"name":"find :: serial_find_middle","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[128,41,108,152,110,108,39,89,109,226,93,221,69,215,65,168,132,159,52,238,169,52,162,226,169,98,68,14,16,156,35,21]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: find :: serial_find_middle ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: find :: serial_find_middle ( & mut crit ) ; }