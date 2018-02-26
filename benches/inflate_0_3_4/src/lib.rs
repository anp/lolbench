extern crate criterion;
extern crate inflate;
#[macro_use]
extern crate wrap_libtest;

use inflate::inflate_bytes;

wrap_libtest! {
    fn decode(b: &mut Bencher) {
        let compressed = include_bytes!("./1m_random_deflated");
        b.iter(|| inflate_bytes(compressed).unwrap());
    }
}
