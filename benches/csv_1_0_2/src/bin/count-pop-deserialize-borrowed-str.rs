extern crate csv_1_0_2 ; extern crate lolbench_support ; use lolbench_support
:: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; csv_1_0_2 ::
count_pop_deserialize_borrowed_str ( & mut crit ) ; }