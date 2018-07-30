#[macro_use]
extern crate proc_macro_hack;

extern crate criterion;
extern crate failure;

pub use criterion::{black_box, init_logging, Bencher, Criterion};
pub type Result<T> = std::result::Result<T, failure::Error>;

// This is what allows the users to depend on just your
// declaration crate rather than both crates.
#[allow(unused_imports)]
#[macro_use]
extern crate lolbench_extractor;
#[doc(hidden)]
pub use lolbench_extractor::*;

// lolbench_entrypoint_impl is provided by the lolbench_extractor crate.
proc_macro_expr_decl! {
    /// Generate a binary entrypoint for the benchmark function under the containing crate's
    /// `src/bin` directory.
    lolbench_entrypoint! => lolbench_entrypoint_impl
}

#[macro_export]
macro_rules! wrap_libtest {
    (fn $name:ident($bencher:ident : &mut Bencher) $body:block) => {
        pub fn $name(c: &mut ::lolbench_support::Criterion) {
            lolbench_entrypoint!($name);

            #[allow(unused_imports)]
            use lolbench_support::black_box;
            c.bench_function(
                concat!(module_path!(), "::", stringify!($name)),
                |$bencher| $body,
            );
        }
    };
    (fn $name:ident($bencher:ident : &mut test::Bencher) $body:block) => {
        wrap_libtest! {
            fn $name($bencher: &mut Bencher) {
                $body
            }
        }
    };
    (fn $name:ident($bencher:ident : &mut::test::Bencher) $body:block) => {
        wrap_libtest! {
            fn $name($bencher: &mut Bencher) {
                $body
            }
        }
    };
    ($module:path,fn $name:ident($bencher:ident : &mut Bencher) $body:block) => {
        pub fn $name(c: &mut ::lolbench_support::Criterion) {
            lolbench_entrypoint!($module::$name);

            #[allow(unused_imports)]
            use lolbench_support::black_box;
            c.bench_function(
                concat!(module_path!(), "::", stringify!($name)),
                |$bencher| $body,
            );
        }
    };
    ($module:path,fn $name:ident($bencher:ident : &mut test::Bencher) $body:block) => {
        wrap_libtest! {
            $module,
            fn $name($bencher: &mut Bencher) {
                $body
            }
        }
    };
    ($module:path,fn $name:ident($bencher:ident : &mut::test::Bencher) $body:block) => {
        wrap_libtest! {
            $module,
            fn $name($bencher: &mut Bencher) {
                $body
            }
        }
    };
}

pub fn criterion_from_env() -> Criterion {
    macro_rules! crit {
        ($( $build_method:ident, )*) => {
            {
                let mut crit = ::criterion::Criterion::default().without_plots();

                $(
                    if let Ok(v) = ::std::env::var(
                        concat!("lolbench_", stringify!($build_method))
                    ) {
                        println!("setting {}", stringify!($build_method));
                        crit = crit.$build_method(v.parse().unwrap());
                    }
                )*

                crit
            }
        };
    }

    crit!(
        sample_size,
        warm_up_time_ms,
        measurement_time_ms,
        nresamples,
        noise_threshold,
        confidence_level,
        significance_level,
    )
}

pub trait CriterionExt: Sized {
    fn warm_up_time_ms(self, ms: usize) -> ::criterion::Criterion;
    fn measurement_time_ms(self, ms: usize) -> ::criterion::Criterion;
}

impl CriterionExt for ::criterion::Criterion {
    #[inline]
    fn warm_up_time_ms(self, ms: usize) -> Self {
        self.warm_up_time(::std::time::Duration::from_millis(ms as u64))
    }

    #[inline]
    fn measurement_time_ms(self, ms: usize) -> Self {
        self.measurement_time(::std::time::Duration::from_millis(ms as u64))
    }
}
