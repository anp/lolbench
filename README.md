# lolbench

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
```

3. in root Cargo.toml, add path dependency on newly created crate:

```toml
inflate_0_3_4 = { path = "./inflate_0_3_4" }
```

4. Create benchmark functions in `subcrate/lib.rs`:

```rs
extern crate criterion;
extern crate inflate;

use inflate::inflate_bytes;

pub fn decode(c: &mut criterion::Criterion) {
    c.bench_function(concat!(env!("CARGO_PKG_NAME"), "_decode"), |b| {
        let compressed = include_bytes!("./1m_random_deflated");
        b.iter(|| inflate_bytes(compressed).unwrap())
    });
}
```

5. In `src/main.rs`, import the subcrate:

```rs
extern crate inflate_0_3_4;
```

6. In `src/main.rs`, create a benchmark group (note: multiple functions can be appended to this macro call)

```rs
criterion_group!(inflate_0_3_4, inflate_0_3_4::decode);
```

7. In `src/main.rs`, add your benchmark group to the `criterion_main!` macro:

```rs
criterion_main!(inflate_0_3_4);
```

8. Run the benchmark suite to ensure everything is being measured correctly
