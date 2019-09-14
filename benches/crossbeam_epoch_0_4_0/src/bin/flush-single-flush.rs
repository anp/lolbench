extern crate crossbeam_epoch_0_4_0 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ;
crossbeam_epoch_0_4_0 :: flush :: single_flush ( & mut crit ) ; }