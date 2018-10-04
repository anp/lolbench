# lolbench

[![CircleCI](https://circleci.com/gh/anp/lolbench/tree/master.svg?style=shield)](https://circleci.com/gh/anp/workflows/lolbench)

This project is an effort to reproducibly benchmark "in the wild" Rust code against newer compiler versions to detect performance regressions. Still a WIP for the moment, but many are the larger building blocks are in place.

## Getting Started

### Dependencies

* rustup
* clang (Linux only)

### Building & Running

```
$ git submodule update --init
$ cargo test-core
```

If for some reason you want to build everything, make sure you have roughly **50GB** of free disk space and run:

```
$ cargo build-all [--release]
```

Every benchmark comes with its own test target too, which can be run like so:

```
$ cargo test-all
```

## Adding new benchmarks

### What kinds of benchmarks?

All benchmarks contributed to lolbench should compile against the current stable compiler after being ported from Rust's nightly-only benchmark harness to lolbench's harness. Benchmarks should have low variance and should aim to have a single iteration take less than a second or two unless the length of time is very important to measuring some quality of the code.

Ideally the runtime characteristics of a benchmark would be unaffected by:

* random number generation
* I/O
* OS process & thread scheduling
* significant amounts of non-Rust code (FFI code is good to measure, but the FFI'd-to code is less interesting)

### Instructions

These instructions assume you want to add new benchmarks in the form of a new crate in the `benches/` directory. If you want to add new benchmarks to an existing benchmark crate, please follow the examples already present in that crate.

#### Creating the crate

Run `cargo new-bench-crate CRATE_NAME` with the name of the new crate. Names should describe what will be benchmarked and *should also include a version substring* after an underscore (see existing for examples). This is important to allow us to add multiple versions of a crate in our benchmarks in the future.

Add the path of the new benchmark crate (relative to the repo root) to the workspace members in the root `Cargo.toml`.

Add any necessary dependencies to the benchmark crate, making sure to specify exact semver bounds for any crates.io dependencies.

Add individual benchmarks functions to the new crate. A convenience macro is provided that will wrap an existing cargo benchmark in a criterion bench runner: `wrap_libtest!`, and an example benchmark which uses it is included with the generated benchmark crate. See more below for information on adapting benchmarks from existing frameworks.

Add the benchmark crate to the CI config under `.circleci/config.yml`. Make sure the new build is added under both the `jobs` key and the `workflow` key -- follow existing examples. The build job should be named the same as the new crate and thus its `benches/` subdirectory.

#### Adapting cargo/libtest benchmarks

There are a few important modifications you'll have to make to a cargo benchmark in addition to wrapping it in the `wrap_libtest!` macro:

1. Remove the `#[bench]` directive above the benchmark function.
2. If the benchmark function isn't in the crate's root module, add its intra-crate module path to the `wrap_libtest` macro invocation. See `benches/rayon_1_0_0/src/fibonacci/mod.rs` for examples where the `fibonacci` module has been included in the macro invocation.
3. Ensure that any calls to `test::black_box` are called as just `black_box` without a module path. The wrapper macro will handle importing the equivalent criterion API that will work on any stable/beta/nightly compiler but it is not able to rewrite fully-qualified uses.

An example of porting rayon's [`fibonacci_join_1_2`][rayon-benchmark-source] benchmark.

The original:

```rust
// rayon/rayon-demo/src/fibonacci/mod.rs

#[bench]
/// Compute the Fibonacci number recursively, using rayon::join.
/// The larger branch F(N-1) is computed first.
fn fibonacci_join_1_2(b: &mut test::Bencher) {
    fn fib(n: u32) -> u32 {
        if n < 2 { return n; }

        let (a, b) = rayon::join(
            || fib(n - 1),
            || fib(n - 2));
        a + b
    }

    b.iter(|| assert_eq!(fib(test::black_box(N)), FN));
}
```

[becomes][lolbench-rayon-benchmark-source]:

```rust
// lolbench/benches/rayon_1_0_0/src/fibonacci/mod.rs

/// Compute the Fibonacci number recursively, using rayon::join.
/// The larger branch F(N-1) is computed first.
wrap_libtest! {
    fibonacci,
    fn fibonacci_join_1_2(b: &mut Bencher) {
        fn fib(n: u32) -> u32 {
            if n < 2 { return n; }

            let (a, b) = rayon::join(
                || fib(n - 1),
                || fib(n - 2));
            a + b
        }

        b.iter(|| assert_eq!(fib(black_box(N)), FN));
    }
}
```

#### Testing it out

In the new bench crate's directory, run:

```
$ cargo build
$ cargo build --release
$ cargo test --release
```

Don't attempt to assign the benchmark to a particular runner yet. If the benchmarks succeed, commit:

* the benchmark crate, including the generated targets under `bin` and `test`
* changes to `registry.toml`
* changes to `.circleci/config.yml`

CI will ensure that all other benchmarks still build on your PR, you don't need to run the test target for every benchmark crate locally. In your PR message please mention which 'benchmark needed' issue should be closed by your PR.

Once your PR is merged we'll re-run the benchmark load balancer (ping @anp if he hasn't written docs for that yet).

## CPU Shielding

On Linux, this feature uses [cpuset](https://github.com/lpechacek/cpuset) to migrate all non-lolbench processes to other CPUs than the provided range in order to improve reliability of our benchmark measurements. To use this feature you must have cpuset installed and run lolbench under a user account that can run sudo without an interactive prompt. Unless you need to specifically investigate the behavior of the CPU shield, it's recommended to run lolbench without root privileges.

[rayon-benchmark-source]: https://github.com/rayon-rs/rayon/blob/5107676d50a261d10b79d8749fd4674498edf9ec/rayon-demo/src/fibonacci/mod.rs#L47-L61
[lolbench-rayon-benchmark-source]: https://github.com/anp/lolbench/blob/d89ddde39fc63361614118f59732549ba2b9c5d4/benches/rayon_1_0_0/src/fibonacci/mod.rs#L48-L64
