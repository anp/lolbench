//{"name":"map_collect::i_to_i :: with_linked_list_collect_vec","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[247,138,69,166,23,13,29,127,195,60,107,26,228,154,174,142,105,217,118,26,254,89,147,54,54,179,36,142,102,88,228,54]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: map_collect :: i_to_i :: with_linked_list_collect_vec ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: map_collect :: i_to_i :: with_linked_list_collect_vec ( & mut crit ) ; }