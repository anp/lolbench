use clap::{App, Arg};

macro_rules! create_app {
    () => ({
        App::new("claptests")
                .version("0.1")
                .about("tests clap library")
                .author("Kevin K. <kbknapp@gmail.com>")
                .args_from_usage("-f --flag         'tests flags'
                                  -o --option=[opt] 'tests options'
                                  [positional]      'tests positional'")
    })
}

wrap_libtest! {
    _02_simple,
    fn build_app(b: &mut Bencher) {

        b.iter(|| create_app!());
    }
}

wrap_libtest! {
    _02_simple,
    fn add_flag(b: &mut Bencher) {
        fn build_app() -> App<'static, 'static> {
            App::new("claptests")
        }

        b.iter(|| build_app().arg(Arg::from_usage("-s, --some 'something'")));
    }
}

wrap_libtest! {
    _02_simple,
    fn add_flag_ref(b: &mut Bencher) {
        fn build_app() -> App<'static, 'static> {
            App::new("claptests")
        }

        b.iter(|| {
            let arg = Arg::from_usage("-s, --some 'something'");
            build_app().arg(&arg)
        });
    }
}

wrap_libtest! {
    _02_simple,
    fn add_opt(b: &mut Bencher) {
        fn build_app() -> App<'static, 'static> {
            App::new("claptests")
        }

        b.iter(|| build_app().arg(Arg::from_usage("-s, --some <FILE> 'something'")));
    }
}

wrap_libtest! {
    _02_simple,
    fn add_opt_ref(b: &mut Bencher) {
        fn build_app() -> App<'static, 'static> {
            App::new("claptests")
        }

        b.iter(|| {
            let arg = Arg::from_usage("-s, --some <FILE> 'something'");
            build_app().arg(&arg)
        });
    }
}

wrap_libtest! {
    _02_simple,
    fn add_pos(b: &mut Bencher) {
        fn build_app() -> App<'static, 'static> {
            App::new("claptests")
        }

        b.iter(|| build_app().arg(Arg::with_name("some")));
    }
}

wrap_libtest! {
    _02_simple,
    fn add_pos_ref(b: &mut Bencher) {
        fn build_app() -> App<'static, 'static> {
            App::new("claptests")
        }

        b.iter(|| {
            let arg = Arg::with_name("some");
            build_app().arg(&arg)
        });
    }
}

wrap_libtest! {
    _02_simple,
    fn parse_clean(b: &mut Bencher) {
        b.iter(|| create_app!().get_matches_from(vec![""]));
    }
}

wrap_libtest! {
    _02_simple,
    fn parse_flag(b: &mut Bencher) {
        b.iter(|| create_app!().get_matches_from(vec!["myprog", "-f"]));
    }
}

wrap_libtest! {
    _02_simple,
    fn parse_option(b: &mut Bencher) {
        b.iter(|| create_app!().get_matches_from(vec!["myprog", "-o", "option1"]));
    }
}

wrap_libtest! {
    _02_simple,
    fn parse_positional(b: &mut Bencher) {
        b.iter(|| create_app!().get_matches_from(vec!["myprog", "arg1"]));
    }
}

wrap_libtest! {
    _02_simple,
    fn parse_complex(b: &mut Bencher) {
        b.iter(|| create_app!().get_matches_from(vec!["myprog", "-o", "option1", "-f", "arg1"]));
    }
}
