extern crate rayon_1_0_0 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; rayon_1_0_0 ::
join_microbench :: increment_all_max ( & mut crit ) ; }