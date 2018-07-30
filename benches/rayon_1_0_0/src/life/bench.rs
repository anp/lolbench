use super::Board;

wrap_libtest! {
    life::bench,
    fn generations(b: &mut Bencher) {
        b.iter(|| super::generations(Board::new(200, 200).random(), 100));
    }
}

wrap_libtest! {
    life::bench,
    fn parallel_generations(b: &mut Bencher) {
        b.iter(|| super::parallel_generations(Board::new(200, 200).random(), 100));
    }
}
