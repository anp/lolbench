extern crate clap_2_31_2 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; clap_2_31_2 ::
_02_simple :: add_opt_ref ( & mut crit ) ; }