extern crate sha2_0_8_0 ; extern crate lolbench_support ; use lolbench_support
:: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; sha2_0_8_0 ::
sha256 :: bench3_1000 ( & mut crit ) ; }