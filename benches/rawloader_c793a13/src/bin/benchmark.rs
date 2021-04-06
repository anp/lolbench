extern crate rawloader_c793a13 ; extern crate lolbench_support ; use
lolbench_support :: { criterion_from_env , init_logging } ; fn main (  ) {
init_logging (  ) ; let mut crit = criterion_from_env (  ) ; rawloader_c793a13
:: benchmark ( & mut crit ) ; }