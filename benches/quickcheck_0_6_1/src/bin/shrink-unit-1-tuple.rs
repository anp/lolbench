extern crate quickcheck_0_6_1 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; quickcheck_0_6_1
:: shrink_unit_1_tuple ( & mut crit ) ; }