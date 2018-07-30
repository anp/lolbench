extern crate crossbeam_epoch as epoch;
extern crate crossbeam_utils as utils;
#[macro_use]
extern crate lolbench_support;

pub mod defer;
pub mod flush;
pub mod pin;

pub use {
    defer::multi_alloc_defer_free, defer::multi_defer, defer::single_alloc_defer_free,
    defer::single_defer, flush::multi_flush, flush::single_flush, pin::multi_pin,
    pin::single_default_handle_pin, pin::single_pin,
};
