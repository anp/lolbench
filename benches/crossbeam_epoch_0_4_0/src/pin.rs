use epoch;
use utils::scoped::scope;

wrap_libtest! {
    pin,
    fn single_pin(b: &mut Bencher) {
        b.iter(|| epoch::pin());
    }
}

wrap_libtest! {
    pin,
    fn single_default_handle_pin(b: &mut Bencher) {
        b.iter(|| epoch::default_handle().pin());
    }
}

wrap_libtest! {
    pin,
    fn multi_pin(b: &mut Bencher) {
        const THREADS: usize = 16;
        const STEPS: usize = 100_000;

        b.iter(|| {
            scope(|s| {
                for _ in 0..THREADS {
                    s.spawn(|| {
                        for _ in 0..STEPS {
                            epoch::pin();
                        }
                    });
                }
            });
        });
    }
}
