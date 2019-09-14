extern crate raytrace_8de9020 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; raytrace_8de9020
:: raytrace_random_scenes ( & mut crit ) ; }