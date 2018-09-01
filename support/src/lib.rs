#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[allow(unused_imports)]
#[macro_use]
extern crate lolbench_extractor;
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

#[cfg(test)]
#[macro_use]
extern crate proptest;
#[cfg(test)]
extern crate tempfile;

pub use criterion::{black_box, init_logging, Bencher, Criterion};
pub use marky_mark::Benchmark;
pub use noisy_float::prelude::*;
pub type Result<T> = std::result::Result<T, failure::Error>;

pub use self::{collector::*, cpu_shield::*, run_plan::*, storage::*, toolchain::*};
#[doc(hidden)]
pub use lolbench_extractor::*;

mod collector;
mod cpu_shield;
mod registry;
mod run_plan;
mod storage;
mod toolchain;

use chrono::NaiveDate;

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

#[derive(Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct BenchOpts {
    pub shield_spec: Option<ShieldSpec>,
    pub runner: Option<String>,
    pub toolchains: ToolchainSpec,
}

#[derive(Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub enum ToolchainSpec {
    Single(String),
    Range(NaiveDate, NaiveDate),
}

impl ToolchainSpec {
    fn all_of_em(&self) -> Vec<Toolchain> {
        use ToolchainSpec::*;
        match self {
            Single(s) => vec![Toolchain::from(s)],
            Range(start, end) => {
                let mut current = *start;
                let mut nightlies = Vec::new();

                while current <= *end {
                    nightlies.push(Toolchain::from(&format!("nightly-{}", current)));
                    current = current.succ();
                }

                nightlies
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolchain_date_range() {
        let spec = ToolchainSpec::Range(
            NaiveDate::from_ymd(2015, 5, 15),
            NaiveDate::from_ymd(2015, 6, 2),
        );

        macro_rules! lolol {
            ( $( $datefrag:expr, )* ) => {
                vec![
                $(
                    Toolchain::from(concat!("nightly-2015-", $datefrag)),
                )*
                ]
            };
        }

        assert_eq!(
            spec.all_of_em(),
            lolol![
                "05-15", "05-16", "05-17", "05-18", "05-19", "05-20", "05-21", "05-22", "05-23",
                "05-24", "05-25", "05-26", "05-27", "05-28", "05-29", "05-30", "05-31", "06-01",
                "06-02",
            ]
        )
    }
}
