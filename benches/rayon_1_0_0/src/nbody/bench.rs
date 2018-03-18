use criterion::Bencher;
use rand::{SeedableRng, XorShiftRng};

use super::nbody::NBodyBenchmark;

// Because benchmarks run iteratively, use smaller constants by default:
const BENCH_BODIES: usize = 1000;

const BENCH_TICKS: usize = 10;

fn nbody_bench<TICK>(b: &mut Bencher, mut tick: TICK)
where
    TICK: FnMut(&mut NBodyBenchmark),
{
    let mut rng = XorShiftRng::from_seed([0, 1, 2, 3]);
    let mut benchmark = NBodyBenchmark::new(BENCH_BODIES, &mut rng);
    b.iter(|| {
        for _ in 0..BENCH_TICKS {
            tick(&mut benchmark);
        }
    });
}

wrap_libtest! {
    fn nbody_seq(b: &mut Bencher) {
        nbody_bench(b, |n| { n.tick_seq(); });
    }
}

wrap_libtest! {
    fn nbody_par(b: &mut Bencher) {
        nbody_bench(b, |n| { n.tick_par(); });
    }
}

wrap_libtest! {
    fn nbody_parreduce(b: &mut Bencher) {
        nbody_bench(b, |n| { n.tick_par_reduce(); });
    }
}
