#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate proc_macro_hack;
#[macro_use]
extern crate serde_derive;

extern crate byteorder;
extern crate chrono;
extern crate criterion;
extern crate glob;
extern crate marky_mark;
extern crate noisy_float;
extern crate ring;
extern crate serde;
extern crate serde_json;
extern crate simple_logger;
extern crate slug;

pub use criterion::{black_box, init_logging, Bencher, Criterion};
pub use marky_mark::Benchmark;
pub use noisy_float::prelude::*;
pub type Result<T> = std::result::Result<T, failure::Error>;

pub use self::{collector::*, cpu_shield::*, planner::*, run_plan::*, storage::*, toolchain::*};

mod collector;
mod cpu_shield;
mod planner;
mod registry;
mod run_plan;
mod storage;
mod toolchain;

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
macro_rules! crit {
    ($( $build_method:ident: $build_method_ty:ty, )*) => {
        pub fn criterion_from_env() -> Criterion {
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

        #[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
        pub struct CriterionConfig {
            $(
                pub $build_method: $build_method_ty,
            )*
        }

        impl CriterionConfig {
            pub fn envs(&self) -> Vec<(String, String)> {
                vec![
                    $((
                        format!("lolbench_{}", stringify!($build_method)),
                        self.$build_method.to_string(),
                    ),)*
                ]
            }
        }
    };
}

crit! {
    sample_size: usize,
    warm_up_time_ms: usize,
    measurement_time_ms: usize,
    nresamples: usize,
    noise_threshold: R32,
    confidence_level: R32,
    significance_level: R32,
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
