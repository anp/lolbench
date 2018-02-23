#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate inflate_0_3_4;

criterion_group!(inflate_0_3_4, inflate_0_3_4::decode);
criterion_main!(inflate_0_3_4);
