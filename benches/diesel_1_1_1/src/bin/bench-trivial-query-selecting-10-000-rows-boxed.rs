//{"name":"bench_trivial_query_selecting_10_000_rows_boxed","crate":"diesel_1_1_1","checksum":{"method":"sha256-generic-array","value":[238,65,159,139,126,114,146,144,193,43,142,213,77,20,56,71,194,135,224,200,147,142,234,240,77,228,39,110,108,132,228,216]}}
extern crate diesel_1_1_1 ; extern crate lolbench_support ; use lolbench_support :: { criterion_from_env , init_logging } ; fn main ( ) { init_logging ( ) ; let mut crit = criterion_from_env ( ) ; diesel_1_1_1 :: bench_trivial_query_selecting_10_000_rows_boxed ( & mut crit ) ; } # [ test ] fn run_bench ( ) { use std :: default :: Default ; use std :: time :: Duration ; use lolbench_support :: Criterion ; init_logging ( ) ; let mut crit = Criterion :: default ( ) ; crit = crit . sample_size ( 2 ) ; crit = crit . warm_up_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . measurement_time ( Duration :: from_micros ( 1 ) ) ; crit = crit . nresamples ( 1 ) ; diesel_1_1_1 :: bench_trivial_query_selecting_10_000_rows_boxed ( & mut crit ) ; }