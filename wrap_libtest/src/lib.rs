#[macro_export]
macro_rules! wrap_libtest {
    (fn $name:ident($bencher:ident: &mut Bencher) $body:block ) => {
        pub fn $name(c: &mut ::criterion::Criterion) {
            #[allow(unused_imports)]
            use ::criterion::black_box;
            c.bench_function(
                concat!(env!("CARGO_PKG_NAME"), "_", stringify!($name)),
                |$bencher| {
                    $body
                }
            );
        }
    };
    (fn $name:ident($bencher:ident: &mut test::Bencher) $body:block ) => {
        wrap_libtest! {
            fn $name($bencher: &mut Bencher) {
                $body
            }
        }
    };
    (fn $name:ident($bencher:ident: &mut ::test::Bencher) $body:block ) => {
        wrap_libtest! {
            fn $name($bencher: &mut Bencher) {
                $body
            }
        }
    };
}
