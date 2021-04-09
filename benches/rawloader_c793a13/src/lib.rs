static EXPECTED_FILE: &str = "vendor/RAW_LEICA_VLUX1.RAW";

#[macro_use]
extern crate lolbench_support;

wrap_libtest! {
    fn benchmark(b: &mut Bencher) {
        let mut f = std::fs::File::open(EXPECTED_FILE)
            .expect("Failed to open critical test file");
        let buffer = rawloader::Buffer::new(&mut f)
            .expect("Failed to initialize buffer");
        let rawloader = rawloader::RawLoader::new();
        b.iter(|| {
            let decoder = rawloader.get_decoder(&buffer).unwrap();
            decoder.image(false).unwrap()
        })
    }
}
