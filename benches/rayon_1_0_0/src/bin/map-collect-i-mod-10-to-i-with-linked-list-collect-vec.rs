//{"name":"map_collect::i_mod_10_to_i ::\n                    with_linked_list_collect_vec","crate":"rayon_1_0_0","checksum":{"method":"sha256-generic-array","value":[182,51,190,71,112,223,119,161,72,118,131,164,151,109,76,193,96,124,254,138,107,10,152,87,176,225,52,172,227,184,15,214]}}
extern crate rayon_1_0_0 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; rayon_1_0_0 :: map_collect :: i_mod_10_to_i :: with_linked_list_collect_vec ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; rayon_1_0_0 :: map_collect :: i_mod_10_to_i :: with_linked_list_collect_vec ( & mut crit ) ; }