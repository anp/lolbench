extern crate snap_0_2_4 ; extern crate lolbench_support ; use lolbench_support
:: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; snap_0_2_4 ::
uflat01_urls ( & mut crit ) ; }