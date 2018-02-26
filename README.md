# lolbench

## SIMD?

Some benchmarks will be more interesting/accurate of real usage with:

```sh
export RUSTFLAGS="-C target-cpu=native"
```

## Adding new benchmarks

1. `cargo new cratename_version`, e.g. `inflate_0_3_4`
2. in newly created crate's Cargo.toml (example from inflate), add a `*` dep on criterion and an exact version dep on the crate to be tested:

```toml
[package]
name = "inflate_0_3_4"
version = "0.1.0"
authors = ["Adam Perry <adam.n.perry@gmail.com>"]

[dependencies]
# important to allow "any" criterion version because this will be compiled into the parent binary
criterion = "*"
inflate = "0.3.4"
wrap_libtest = { path = "../../wrap_libtest" }
```

3. in root Cargo.toml, add path dependency on newly created crate:

```toml
inflate_0_3_4 = { path = "./inflate_0_3_4" }
```

4. Create benchmark functions in `subcrate/lib.rs`. If you're porting from the libtest bench harness to criterion, the [criterion user guide](https://japaric.github.io/criterion.rs/book/criterion_rs.html) is a good place to start. A convenience macro is provided that will wrap an existing cargo benchmark in a criterion bench runner: `wrap_libtest!`. See the below example from inflate for usage:

```rs
extern crate criterion;
extern crate inflate;
#[macro_use]
extern crate wrap_libtest;

use inflate::inflate_bytes;

wrap_libtest!
    fn decode(b: &mut Bencher) {
        let compressed = include_bytes!("./1m_random_deflated");
        b.iter(|| inflate_bytes(compressed).unwrap());
    }
}
```

There are two important modifications you'll have to make to a cargo benchmark:

* remove the `#[bench]` directive
* ensure that any calls to `test::black_box` are called as just `black_box` (no module path). The wrapper macro will handle importing the equivalent criterion API that will work on any stable/beta/nightly compiler.

5. In `src/main.rs`, import the subcrate:

```rs
extern crate inflate_0_3_4;
```

6. In `src/main.rs`, create a benchmark group (note: multiple functions can be appended to this macro call)

```rs
criterion_group!(inflate_0_3_4, inflate_0_3_4::decode);
```

You can append more benchmark functions after the first.

7. In `src/main.rs`, add your benchmark group to the `criterion_main!` macro:

```rs
criterion_main!(inflate_0_3_4);
```

8. Run the benchmark suite to ensure everything is being measured correctly
