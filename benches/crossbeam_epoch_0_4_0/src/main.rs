#[macro_use]
extern crate criterion;
extern crate crossbeam_epoch as epoch;
extern crate crossbeam_utils as utils;
#[macro_use]
extern crate wrap_libtest;

use criterion::Criterion;

pub mod defer;
pub mod flush;
pub mod pin;

criterion_group! {
    crossbeam_epoch_0_4_0,
    defer::single_alloc_defer_free,
    defer::single_defer,
    defer::multi_alloc_defer_free,
    defer::multi_defer,
    flush::single_flush,
    flush::multi_flush,
    pin::single_pin,
    pin::single_default_handle_pin,
    pin::multi_pin
}

criterion_main! {
    crossbeam_epoch_0_4_0
}
