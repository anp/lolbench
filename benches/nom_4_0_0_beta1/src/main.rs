#[macro_use]
extern crate criterion;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate wrap_libtest;

use criterion::Criterion;

pub mod arithmetic;
pub mod http;
pub mod ini;
pub mod ini_str;
// pub mod json;

criterion_group! {
    nom_4_0_0_beta1,
    arithmetic::arithmetic,
    http::one_test,
    ini_str::bench_ini_str,
    ini::bench_ini,
    ini::bench_ini_keys_and_values,
    ini::bench_ini_key_value
}

criterion_main! {
    nom_4_0_0_beta1,
}
