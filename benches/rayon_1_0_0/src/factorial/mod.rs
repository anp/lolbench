//! Benchmark Factorial N! = 1×2×⋯×N

use num::{BigUint, One};
use rayon;
use rayon::prelude::*;
use std::ops::Mul;

const N: u32 = 9999;

/// Compute the Factorial using a plain iterator.
fn factorial(n: u32) -> BigUint {
    (1..n + 1).map(BigUint::from).fold(BigUint::one(), Mul::mul)
}

/// Benchmark the Factorial using a plain iterator.
wrap_libtest! {
    factorial,
    fn factorial_iterator(b: &mut Bencher) {
        let f = factorial(N);
        b.iter(|| assert_eq!(factorial(black_box(N)), f));
    }
}

/// Compute the Factorial using rayon::par_iter.
wrap_libtest! {
    factorial,
    fn factorial_par_iter(b: &mut Bencher) {
        fn fact(n: u32) -> BigUint {
            (1 .. n + 1).into_par_iter()
                .map(BigUint::from)
                .reduce_with(Mul::mul)
                .unwrap()
        }

        let f = factorial(N);
        b.iter(|| assert_eq!(fact(black_box(N)), f));
    }
}

/// Compute the Factorial using divide-and-conquer serial recursion.
wrap_libtest! {
    factorial,
    fn factorial_recursion(b: &mut Bencher) {

        fn product(a: u32, b: u32) -> BigUint {
            if a == b { return a.into(); }
            let mid = (a + b) / 2;
            product(a, mid) * product(mid + 1, b)
        }

        let f = factorial(N);
        b.iter(|| assert_eq!(product(1, black_box(N)), f));
    }
}

/// Compute the Factorial using divide-and-conquer parallel join.
wrap_libtest! {
    factorial,
    fn factorial_join(b: &mut Bencher) {
        fn product(a: u32, b: u32) -> BigUint {
            if a == b { return a.into(); }
            let mid = (a + b) / 2;
            let (x, y) = rayon::join(
                || product(a, mid),
                || product(mid + 1, b));
            x * y
        }

        let f = factorial(N);
        b.iter(|| assert_eq!(product(1, black_box(N)), f));
    }
}
