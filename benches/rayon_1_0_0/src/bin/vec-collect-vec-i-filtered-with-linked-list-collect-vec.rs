//{"name":"vec_collect::vec_i_filtered ::\n                    with_linked_list_collect_vec","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[52,14,157,68,1,103,225,101,107,243,67,45,88,154,42,86,90,100,138,15,192,136,234,167,229,12,251,225,88,159,17,174]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: vec_collect :: vec_i_filtered :: with_linked_list_collect_vec ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: vec_collect :: vec_i_filtered :: with_linked_list_collect_vec ( & mut crit ) ; }