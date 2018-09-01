# lolbench

[![CircleCI](https://circleci.com/gh/anp/lolbench/tree/master.svg?style=shield)](https://circleci.com/gh/anp/lolbench/tree/master)

This project is an effort to reproducibly benchmark "in the wild" Rust code against newer compiler versions to detect performance regressions. Still a WIP for the moment, but many are the larger building blocks are in place.

## tldr

### Dependencies

* rustup
* clang (Linux only)

Want to dive in? Make sure you have 50GB or more of disk space available if you want to build everything!

```
$ git clone ...
$ git submodule update --init
$ cargo build --all
$ cargo build --all --release
$ cargo test-core
```

# TODO

* LTO, strip, cgu=1, incrcomp=false
* flesh out contributing guide

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
lolbench_support = { path = "../../support" }
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

5. Run `cargo build` in your benchmark crate's directory, followed by `cargo test`. If the benchmarks succeed, commit the relevant binary and test targets and the changes to `registry.toml`.

## CPU Shielding

On Linux, this flag uses [cpuset](https://github.com/lpechacek/cpuset) to migrate all non-lolbench processes to other CPUs than the provided range in order to improve reliability of our benchmark measurements. To use this feature you must have cpuset installed and run lolbench under a user account that can run sudo without an interactive prompt. Unless you need to specifically investigate the behavior of the CPU shield, it's recommended to run lolbench without root privileges.
