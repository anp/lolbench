extern crate clap_2_31_2 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; clap_2_31_2 ::
_03_complex :: create_app_from_usage ( & mut crit ) ; }