//! Benchmarks from byteorder 1.2.6
//!
//! The code in this module was adapted from the Byteorder crate by
//! BurntSushi and contributors.
//!
//! It's available under the MIT and UNLICENSE licenses.
//!
//! See https://github.com/BurntSushi/byteorder
#[macro_use]
extern crate lolbench_support;

extern crate byteorder;
extern crate rand;

macro_rules! bench_num {
    ($name:ident, $read:ident, $bytes:expr, $data:expr) => (
        pub mod $name {
            use byteorder::{ByteOrder, BigEndian, NativeEndian, LittleEndian};

            const NITER: usize = 100_000;

            wrap_libtest! {
                $name,
                fn read_big_endian(b: &mut Bencher) {
                    let buf = $data;
                    b.iter(|| {
                        for _ in 0..NITER {
                            black_box(BigEndian::$read(&buf, $bytes));
                        }
                    });
                }
            }

            wrap_libtest! {
                $name,
                fn read_little_endian(b: &mut Bencher) {
                    let buf = $data;
                    b.iter(|| {
                        for _ in 0..NITER {
                            black_box(LittleEndian::$read(&buf, $bytes));
                        }
                    });
                }
            }

            wrap_libtest! {
                $name,
                fn read_native_endian(b: &mut Bencher) {
                    let buf = $data;
                    b.iter(|| {
                        for _ in 0..NITER {
                            black_box(NativeEndian::$read(&buf, $bytes));
                        }
                    });
                }
            }
        }
    );
    ($ty:ident, $max:ident,
     $read:ident, $write:ident, $size:expr, $data:expr) => (
        pub mod $ty {
            use std::$ty;
            use byteorder::{ByteOrder, BigEndian, NativeEndian, LittleEndian};

            const NITER: usize = 100_000;

            wrap_libtest! {
                $ty,
                fn read_big_endian(b: &mut Bencher) {
                    let buf = $data;
                    b.iter(|| {
                        for _ in 0..NITER {
                            black_box(BigEndian::$read(&buf));
                        }
                    });
                }
            }

            wrap_libtest! {
                $ty,
                fn read_little_endian(b: &mut Bencher) {
                    let buf = $data;
                    b.iter(|| {
                        for _ in 0..NITER {
                            black_box(LittleEndian::$read(&buf));
                        }
                    });
                }
            }

            wrap_libtest! {
                $ty,
                fn read_native_endian(b: &mut Bencher) {
                    let buf = $data;
                    b.iter(|| {
                        for _ in 0..NITER {
                            black_box(NativeEndian::$read(&buf));
                        }
                    });
                }
            }

            wrap_libtest! {
                $ty,
                fn write_big_endian(b: &mut Bencher) {
                    let mut buf = $data;
                    let n = $ty::$max;
                    b.iter(|| {
                        for _ in 0..NITER {
                            black_box(BigEndian::$write(&mut buf, n));
                        }
                    });
                }
            }

            wrap_libtest! {
                $ty,
                fn write_little_endian(b: &mut Bencher) {
                    let mut buf = $data;
                    let n = $ty::$max;
                    b.iter(|| {
                        for _ in 0..NITER {
                            black_box(LittleEndian::$write(&mut buf, n));
                        }
                    });
                }
            }

            wrap_libtest! {
                $ty,
                fn write_native_endian(b: &mut Bencher) {
                    let mut buf = $data;
                    let n = $ty::$max;
                    b.iter(|| {
                        for _ in 0..NITER {
                            black_box(NativeEndian::$write(&mut buf, n));
                        }
                    });
                }
            }
        }
    );
}

bench_num!(u16, MAX, read_u16, write_u16, 2, [1, 2]);
bench_num!(i16, MAX, read_i16, write_i16, 2, [1, 2]);
bench_num!(u32, MAX, read_u32, write_u32, 4, [1, 2, 3, 4]);
bench_num!(i32, MAX, read_i32, write_i32, 4, [1, 2, 3, 4]);
bench_num!(u64, MAX, read_u64, write_u64, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
bench_num!(i64, MAX, read_i64, write_i64, 8, [1, 2, 3, 4, 5, 6, 7, 8]);
bench_num!(f32, MAX, read_f32, write_f32, 4, [1, 2, 3, 4]);
bench_num!(f64, MAX, read_f64, write_f64, 8,
           [1, 2, 3, 4, 5, 6, 7, 8]);

bench_num!(uint_1, read_uint, 1, [1]);
bench_num!(uint_2, read_uint, 2, [1, 2]);
bench_num!(uint_3, read_uint, 3, [1, 2, 3]);
bench_num!(uint_4, read_uint, 4, [1, 2, 3, 4]);
bench_num!(uint_5, read_uint, 5, [1, 2, 3, 4, 5]);
bench_num!(uint_6, read_uint, 6, [1, 2, 3, 4, 5, 6]);
bench_num!(uint_7, read_uint, 7, [1, 2, 3, 4, 5, 6, 7]);
bench_num!(uint_8, read_uint, 8, [1, 2, 3, 4, 5, 6, 7, 8]);

bench_num!(int_1, read_int, 1, [1]);
bench_num!(int_2, read_int, 2, [1, 2]);
bench_num!(int_3, read_int, 3, [1, 2, 3]);
bench_num!(int_4, read_int, 4, [1, 2, 3, 4]);
bench_num!(int_5, read_int, 5, [1, 2, 3, 4, 5]);
bench_num!(int_6, read_int, 6, [1, 2, 3, 4, 5, 6]);
bench_num!(int_7, read_int, 7, [1, 2, 3, 4, 5, 6, 7]);
bench_num!(int_8, read_int, 8, [1, 2, 3, 4, 5, 6, 7, 8]);

#[cfg(feature = "i128")]
bench_num!(u128, MAX, read_u128, write_u128,
    16, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
#[cfg(feature = "i128")]
bench_num!(i128, MAX, read_i128, write_i128,
    16, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

#[cfg(feature = "i128")]
bench_num!(uint128_1, read_uint128,
    1, [1]);
#[cfg(feature = "i128")]
bench_num!(uint128_2, read_uint128,
    2, [1, 2]);
#[cfg(feature = "i128")]
bench_num!(uint128_3, read_uint128,
    3, [1, 2, 3]);
#[cfg(feature = "i128")]
bench_num!(uint128_4, read_uint128,
    4, [1, 2, 3, 4]);
#[cfg(feature = "i128")]
bench_num!(uint128_5, read_uint128,
    5, [1, 2, 3, 4, 5]);
#[cfg(feature = "i128")]
bench_num!(uint128_6, read_uint128,
    6, [1, 2, 3, 4, 5, 6]);
#[cfg(feature = "i128")]
bench_num!(uint128_7, read_uint128,
    7, [1, 2, 3, 4, 5, 6, 7]);
#[cfg(feature = "i128")]
bench_num!(uint128_8, read_uint128,
    8, [1, 2, 3, 4, 5, 6, 7, 8]);
#[cfg(feature = "i128")]
bench_num!(uint128_9, read_uint128,
    9, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
#[cfg(feature = "i128")]
bench_num!(uint128_10, read_uint128,
    10, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
#[cfg(feature = "i128")]
bench_num!(uint128_11, read_uint128,
    11, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
#[cfg(feature = "i128")]
bench_num!(uint128_12, read_uint128,
    12, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
#[cfg(feature = "i128")]
bench_num!(uint128_13, read_uint128,
    13, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
#[cfg(feature = "i128")]
bench_num!(uint128_14, read_uint128,
    14, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
#[cfg(feature = "i128")]
bench_num!(uint128_15, read_uint128,
    15, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
#[cfg(feature = "i128")]
bench_num!(uint128_16, read_uint128,
    16, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

#[cfg(feature = "i128")]
bench_num!(int128_1, read_int128,
    1, [1]);
#[cfg(feature = "i128")]
bench_num!(int128_2, read_int128,
    2, [1, 2]);
#[cfg(feature = "i128")]
bench_num!(int128_3, read_int128,
    3, [1, 2, 3]);
#[cfg(feature = "i128")]
bench_num!(int128_4, read_int128,
    4, [1, 2, 3, 4]);
#[cfg(feature = "i128")]
bench_num!(int128_5, read_int128,
    5, [1, 2, 3, 4, 5]);
#[cfg(feature = "i128")]
bench_num!(int128_6, read_int128,
    6, [1, 2, 3, 4, 5, 6]);
#[cfg(feature = "i128")]
bench_num!(int128_7, read_int128,
    7, [1, 2, 3, 4, 5, 6, 7]);
#[cfg(feature = "i128")]
bench_num!(int128_8, read_int128,
    8, [1, 2, 3, 4, 5, 6, 7, 8]);
#[cfg(feature = "i128")]
bench_num!(int128_9, read_int128,
    9, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
#[cfg(feature = "i128")]
bench_num!(int128_10, read_int128,
    10, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
#[cfg(feature = "i128")]
bench_num!(int128_11, read_int128,
    11, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
#[cfg(feature = "i128")]
bench_num!(int128_12, read_int128,
    12, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
#[cfg(feature = "i128")]
bench_num!(int128_13, read_int128,
    13, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
#[cfg(feature = "i128")]
bench_num!(int128_14, read_int128,
    14, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
#[cfg(feature = "i128")]
bench_num!(int128_15, read_int128,
    15, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
#[cfg(feature = "i128")]
bench_num!(int128_16, read_int128,
    16, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

macro_rules! bench_slice {
    ($name:ident, $numty:ty, $read:ident, $write:ident) => {
        pub mod $name {
            use std::mem::size_of;

            use byteorder::{ByteOrder, BigEndian, LittleEndian};
            use rand::{self, Rng};
            use rand::distributions;

            wrap_libtest! {
                $name,
                fn read_big_endian(b: &mut Bencher) {
                    let mut numbers: Vec<$numty> = rand::thread_rng()
                        .sample_iter(&distributions::Standard)
                        .take(100000)
                        .collect();
                    let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];
                    BigEndian::$write(&numbers, &mut bytes);

                    // b.bytes = bytes.len() as u64;  // test::Bencher uses this for MB/s
                    b.iter(|| {
                        BigEndian::$read(&bytes, &mut numbers);
                    });
                }
            }

            wrap_libtest! {
                $name,
                fn read_little_endian(b: &mut Bencher) {
                    let mut numbers: Vec<$numty> = rand::thread_rng()
                        .sample_iter(&distributions::Standard)
                        .take(100000)
                        .collect();
                    let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];
                    LittleEndian::$write(&numbers, &mut bytes);

                    // b.bytes = bytes.len() as u64;  // test::Bencher uses this for MB/s
                    b.iter(|| {
                        LittleEndian::$read(&bytes, &mut numbers);
                    });
                }
            }

            
            wrap_libtest! {
                $name,
                fn write_big_endian(b: &mut Bencher) {
                    let numbers: Vec<$numty> = rand::thread_rng()
                        .sample_iter(&distributions::Standard)
                        .take(100000)
                        .collect();
                    let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];

                    // b.bytes = bytes.len() as u64;  // test::Bencher uses this for MB/s
                    b.iter(|| {
                        BigEndian::$write(&numbers, &mut bytes);
                    });
                }
            }

            wrap_libtest! {
                $name,
                fn write_little_endian(b: &mut Bencher) {
                    let numbers: Vec<$numty> = rand::thread_rng()
                        .sample_iter(&distributions::Standard)
                        .take(100000)
                        .collect();
                    let mut bytes = vec![0; numbers.len() * size_of::<$numty>()];

                    // b.bytes = bytes.len() as u64;  // test::Bencher uses this for MB/s
                    b.iter(|| {
                    LittleEndian::$write(&numbers, &mut bytes);
                    });
                }
            }
        }
    }
}

bench_slice!(slice_u64, u64, read_u64_into, write_u64_into);
