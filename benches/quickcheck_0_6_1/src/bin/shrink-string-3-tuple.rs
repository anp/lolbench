//{"name":"shrink_string_3_tuple","crate":"quickcheck_0_6_1","checksum":{"method":"sha256-generic-array","value":[188,92,232,138,229,36,77,224,2,82,195,151,100,191,241,91,137,70,189,114,80,224,242,239,173,118,155,70,59,176,170,52]}}
extern crate quickcheck_0_6_1 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; quickcheck_0_6_1 :: shrink_string_3_tuple ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; quickcheck_0_6_1 :: shrink_string_3_tuple ( & mut crit ) ; }