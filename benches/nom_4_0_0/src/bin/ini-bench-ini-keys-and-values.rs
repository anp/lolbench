extern crate nom_4_0_0 ; extern crate lolbench_support ; use lolbench_support
:: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; nom_4_0_0 :: ini
:: bench_ini_keys_and_values ( & mut crit ) ; }