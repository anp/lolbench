extern crate criterion;
extern crate inflate;

use inflate::inflate_bytes;

pub fn decode(c: &mut criterion::Criterion) {
    c.bench_function(concat!(env!("CARGO_PKG_NAME"), "_decode"), |b| {
        let compressed = include_bytes!("./1m_random_deflated");
        b.iter(|| inflate_bytes(compressed).unwrap())
    });
}
