//{"name":"shrink_i64_7_tuple","crate":"quickcheck_0_6_1","checksum":{"method":"sha256-generic-array","value":[14,244,251,162,25,59,46,213,130,219,198,40,3,75,187,90,201,16,230,150,156,113,166,182,224,213,210,68,233,87,161,57]}}
extern crate quickcheck_0_6_1 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; quickcheck_0_6_1 :: shrink_i64_7_tuple ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; quickcheck_0_6_1 :: shrink_i64_7_tuple ( & mut crit ) ; }