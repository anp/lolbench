extern crate diesel_1_1_1 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; diesel_1_1_1 ::
bench_medium_complex_query_selecting__1_000_rows_boxed ( & mut crit ) ; }