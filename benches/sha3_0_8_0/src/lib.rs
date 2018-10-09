//! This module benchmarks sha3 hashes.
//!
//! It contains a modified version of the Digest crate's bench macro.
#[macro_use]
extern crate lolbench_support;

macro_rules! digest_bench {
    ($module:path, $name:ident, $engine:path, $bs:expr) => {
        wrap_libtest! {
            $module,
            fn $name(b: &mut Bencher) {
                let mut d = <$engine>::default();
                let data = [0; $bs];

                b.iter(|| {
                    d.input(&data[..]);
                });

                // b.bytes = $bs;
                // Criterion's bencher does not support MB/s
            }
        }
    };

    ($mod:ident, $engine:path) => {
        pub mod $mod {
            extern crate digest;
            extern crate sha3;

            use self::digest::Digest;

            digest_bench!($mod, bench1_10,    $engine, 10);
            digest_bench!($mod, bench2_100,   $engine, 100);
            digest_bench!($mod, bench3_1000,  $engine, 1000);
            digest_bench!($mod, bench4_10000, $engine, 10000);
        }
    };
}

digest_bench!(sha3_256, sha3::Sha3_256);
digest_bench!(sha3_512, sha3::Sha3_512);
