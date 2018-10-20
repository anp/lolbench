use clap::App;

wrap_libtest! {
    _01_default,
    fn build_app(b: &mut Bencher) {
        b.iter(|| App::new("claptests"));
    }
}

wrap_libtest! {
    _01_default,
    fn parse_clean(b: &mut Bencher) {
        b.iter(|| App::new("claptests").get_matches_from(vec![""]));
    }
}
