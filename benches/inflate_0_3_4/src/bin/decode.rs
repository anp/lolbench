extern crate inflate_0_3_4 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; inflate_0_3_4 ::
decode ( & mut crit ) ; }