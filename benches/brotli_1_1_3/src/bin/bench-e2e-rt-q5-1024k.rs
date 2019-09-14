extern crate brotli_1_1_3 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; brotli_1_1_3 ::
bench_e2e_rt_q5_1024k ( & mut crit ) ; }