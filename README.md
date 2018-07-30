# lolbench

# TODO

* new command could easily rewrite #[bench], criterion_group, and wrap_libtest, all in place (run rustfmt after)
* extractor should add benchmarks to an unassigned section of the configuration
* subcommand to time benchmarks, test multiple values for each tweakable environment variable in the new harness
* create config file pointing different boxes at each other
* set up CI to build all of the test binaries and run each once
* LTO, strip, cgu=1, incrcomp=false
* hash single function binaries
* flesh out contributing guide

---

deps:

* rustup
* on linux, cpu shield will call commands with sudo
  * need to make sure that `cset` can be invoked with sudo without a password
* on linux, clang is required to build perf_events wrapper

## SIMD?

Some benchmarks will be more interesting/accurate of real usage with:

```sh
export RUSTFLAGS="-C target-cpu=native"
```

## Adding new benchmarks

1. inside the `benches` dir, `cargo new cratename_version`, e.g. `inflate_0_3_4`
2. in newly created crate's Cargo.toml (example from inflate), add an exact version dep on the crate to be tested and a path dep on the support crate:

```toml
[package]
name = "inflate_0_3_4"
version = "0.1.0"
authors = ["Adam Perry <adam.n.perry@gmail.com>"]

[dependencies]
inflate = "0.3.4"
lolbench_support = { path = "../../lolbench_support" }
```

3. in root Cargo.toml, add path dependency on newly created crate:

```toml
inflate_0_3_4 = { path = "./inflate_0_3_4" }
```

4. Create benchmark functions in `subcrate/lib.rs`. If you're porting from the libtest bench harness to criterion, the [criterion user guide](https://japaric.github.io/criterion.rs/book/criterion_rs.html) is a good place to start. A convenience macro is provided that will wrap an existing cargo benchmark in a criterion bench runner: `wrap_libtest!`. See the below example from inflate for usage:

```rs
extern crate inflate;
#[macro_use]
extern crate lolbench_support;

use inflate::inflate_bytes;

wrap_libtest!
    fn decode(b: &mut Bencher) {
        let compressed = include_bytes!("./1m_random_deflated");
        b.iter(|| inflate_bytes(compressed).unwrap());
    }
}
```

There are three important modifications you'll have to make to a cargo benchmark:

* remove the `#[bench]` directive
* if the benchmark function isn't at the crate root, either re-export it from the crate root
  or add its intra-crate module path to the macro invocation (see examples in rayon)
* ensure that any calls to `test::black_box` are called as just `black_box` (no module path). The wrapper macro will handle importing the equivalent criterion API that will work on any stable/beta/nightly compiler.

5. Run the benchmark suite to ensure everything is being measured correctly


## performance on benchmark machine

see ansible playbooks and cpu_shield.rs for info about bench noise mitigation

NOTE: you have to run as root. I tried to use cpuset's "exec as user/group" feature, but rustup had problems with that (thought that /root/.cargo was where it should be installed). For me, this meant `rustup default stable && rustup update` as root and everything worked.

cpuset has a fun trick to move all kernel threads onto the not-used-for-benchmarks core too, which in theory should greatly improve predictability of results.
