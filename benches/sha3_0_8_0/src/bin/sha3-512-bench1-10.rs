extern crate sha3_0_8_0 ; extern crate lolbench_support ; use lolbench_support
:: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; sha3_0_8_0 ::
sha3_512 :: bench1_10 ( & mut crit ) ; }