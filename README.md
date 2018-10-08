# lolbench

[![CircleCI](https://circleci.com/gh/anp/lolbench/tree/master.svg?style=shield)](https://circleci.com/gh/anp/workflows/lolbench)

This project is an effort to reproducibly benchmark "in the wild" Rust code against newer compiler versions to detect performance regressions. Still a WIP for the moment, but many are the larger building blocks are in place.

## Adding Benchmarks

Want to contribute and are looking for the [list of benchmarks we'd like help adding](https://github.com/anp/lolbench/issues/1)?

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
