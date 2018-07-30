//! Some microbenchmarks for splitting strings

use rand::{Rng, SeedableRng, XorShiftRng};
use rayon::prelude::*;

lazy_static! {
    static ref HAYSTACK: String = {
        let mut rng = XorShiftRng::from_seed([0, 1, 2, 3]);
        let mut bytes: Vec<u8> = "abcdefg ".bytes().cycle().take(1_000_000).collect();
        rng.shuffle(&mut bytes);
        String::from_utf8(bytes).unwrap()
    };
    static ref COUNT: usize = { HAYSTACK.split(' ').count() };
}

fn get_string_count() -> (&'static str, usize) {
    (&HAYSTACK, *COUNT)
}

wrap_libtest! {
    str_split,
    fn parallel_space_char(b: &mut Bencher) {
        let (string, count) = get_string_count();
        b.iter(|| assert_eq!(string.par_split(' ').count(), count))
    }
}

wrap_libtest! {
    str_split,
    fn parallel_space_fn(b: &mut Bencher) {
        let (string, count) = get_string_count();
        b.iter(|| assert_eq!(string.par_split(|c| c == ' ').count(), count))
    }
}

wrap_libtest! {
    str_split,
    fn serial_space_char(b: &mut Bencher) {
        let (string, count) = get_string_count();
        b.iter(|| assert_eq!(string.split(' ').count(), count))
    }
}

wrap_libtest! {
    str_split,
    fn serial_space_fn(b: &mut Bencher) {
        let (string, count) = get_string_count();
        b.iter(|| assert_eq!(string.split(|c| c == ' ').count(), count))
    }
}

wrap_libtest! {
    str_split,
    fn serial_space_str(b: &mut Bencher) {
        let (string, count) = get_string_count();
        b.iter(|| assert_eq!(string.split(" ").count(), count))
    }
}
