extern crate byteorder_1_2_6 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; byteorder_1_2_6
:: u32 :: write_little_endian ( & mut crit ) ; }