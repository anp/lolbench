//{"name":"shrink_vec_u8_1_tuple","crate":"quickcheck_0_6_1","checksum":{"method":"sha256-generic-array","value":[79,139,227,139,247,42,67,191,164,129,197,162,93,194,241,26,89,232,16,11,122,91,235,221,245,140,158,27,252,64,99,158]}}
extern crate quickcheck_0_6_1 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; quickcheck_0_6_1 :: shrink_vec_u8_1_tuple ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; quickcheck_0_6_1 :: shrink_vec_u8_1_tuple ( & mut crit ) ; }