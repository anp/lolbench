static DOWNLOAD_ERR: &str = "Failed to load test file. Please download it from: \n\
                             http://www.rawsamples.ch/raws/leica/vlux1/RAW_LEICA_VLUX1.RAW";
static EXPECTED_FILE: &str = "RAW_LEICA_VLUX1.RAW";

#[macro_use]
extern crate lolbench_support;

wrap_libtest! {
    fn benchmark(b: &mut Bencher) {
        let mut f = std::fs::File::open(EXPECTED_FILE)
            .expect(&DOWNLOAD_ERR);
        let buffer = rawloader::Buffer::new(&mut f)
            .expect("Failed to initialize buffer");
        let rawloader = rawloader::RawLoader::new();
        b.iter(|| {
            let decoder = rawloader.get_decoder(&buffer).unwrap();
            decoder.image(false).unwrap()
        })
    }
}
