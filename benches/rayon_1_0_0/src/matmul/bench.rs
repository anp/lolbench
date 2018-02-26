const ROW_SIZE: usize = 256;

wrap_libtest! {
    fn bench_matmul_strassen(b: &mut Bencher) {
        let n = ROW_SIZE * ROW_SIZE;
        let x = vec![1f32; n];
        let y = vec![2f32; n];
        let mut z = vec![0f32; n];

        b.iter(|| {
            super::matmul_strassen(&x, &y, &mut z);
        });
    }
}
