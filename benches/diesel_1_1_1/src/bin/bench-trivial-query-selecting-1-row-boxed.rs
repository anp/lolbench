extern crate diesel_1_1_1 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; diesel_1_1_1 ::
bench_trivial_query_selecting______1_row_boxed ( & mut crit ) ; }