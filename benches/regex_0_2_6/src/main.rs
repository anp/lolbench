#[macro_use]
extern crate criterion;
extern crate regex;
extern crate regex_syntax;
#[macro_use]
extern crate wrap_libtest;

#[macro_use]
mod bench;
pub mod misc;
pub mod regexdna;
// pub mod rust_compile;
pub mod rust_parse;
pub mod sherlock;

use criterion::Criterion;

criterion_group! {
    regex_0_2_6,
    misc::literal,
    misc::not_literal,
    misc::match_class,
    misc::match_class_in_range,
    misc::anchored_literal_short_non_match,
    misc::anchored_literal_long_non_match,
    misc::anchored_literal_short_match,
    misc::anchored_literal_long_match,
    misc::one_pass_short,
    misc::one_pass_short_not,
    misc::one_pass_long_prefix,
    misc::one_pass_long_prefix_not,
    misc::long_needle1,
    misc::long_needle2,
    misc::reverse_suffix_no_quadratic,
    misc::replace_all,
    misc::easy0_32,
    misc::easy0_1K,
    misc::easy0_32K,
    misc::easy0_1MB,
    misc::easy1_32,
    misc::easy1_1K,
    misc::easy1_32K,
    misc::easy1_1MB,
    misc::medium_32,
    misc::medium_1K,
    misc::medium_32K,
    misc::medium_1MB,
    misc::hard_32,
    misc::hard_1K,
    misc::hard_32K,
    misc::hard_1MB,
    misc::reallyhard_32,
    misc::reallyhard_1K,
    misc::reallyhard_32K,
    misc::reallyhard_1MB,
    misc::reallyhard2_1K,
    misc::short_haystack_1x,
    misc::short_haystack_2x,
    misc::short_haystack_3x,
    misc::short_haystack_4x,
    misc::short_haystack_10x,
    misc::short_haystack_100x,
    misc::short_haystack_1000x,
    misc::short_haystack_10000x,
    misc::short_haystack_100000x,
    misc::short_haystack_1000000x,
    regexdna::find_new_lines,
    regexdna::variant1,
    regexdna::variant2,
    regexdna::variant3,
    regexdna::variant4,
    regexdna::variant5,
    regexdna::variant6,
    regexdna::variant7,
    regexdna::variant8,
    regexdna::variant9,
    regexdna::subst1,
    regexdna::subst2,
    regexdna::subst3,
    regexdna::subst4,
    regexdna::subst5,
    regexdna::subst6,
    regexdna::subst7,
    regexdna::subst8,
    regexdna::subst9,
    regexdna::subst10,
    regexdna::subst11,
    // compilation errors
    // rust_compile::compile_simple,
    // rust_compile::compile_simple_bytes,
    // rust_compile::compile_simple_full,
    // rust_compile::compile_small,
    // rust_compile::compile_small_bytes,
    // rust_compile::compile_small_full,
    // rust_compile::compile_huge,
    // end compilation errors
    //     // Benchmarking regex_0_2_6::rust_compile::compile_huge_bytes: Warming up for 3.0000 sthread 'main' panicked at
//     // 'called `Result::unwrap()` on an `Err` value: CompiledTooBig(10485760)', /checkout/src/libcore/result.rs:916:5
//     // regex_0_2_6::rust_compile::compile_huge_bytes,

//     // Benchmarking regex_0_2_6::rust_compile::compile_huge_full: Warming up for 3.0000 sthread 'main' panicked at
//     // 'called `Result::unwrap()` on an `Err` value: CompiledTooBig(10485760)', /checkout/src/libcore/result.rs:916:5
//     // regex_0_2_6::rust_compile::compile_huge_full,
    rust_parse::parse_simple,
    rust_parse::parse_simple2,
    rust_parse::parse_small,
    rust_parse::parse_huge,
    sherlock::name_sherlock,
    sherlock::name_holmes,
    sherlock::name_sherlock_holmes,
    sherlock::name_sherlock_nocase,
    sherlock::name_holmes_nocase,
    sherlock::name_sherlock_holmes_nocase,
    sherlock::name_whitespace,
    sherlock::name_alt1,
    sherlock::name_alt2,
    sherlock::name_alt3,
    sherlock::name_alt3_nocase,
    sherlock::name_alt4,
    sherlock::name_alt4_nocase,
    sherlock::name_alt5,
    sherlock::name_alt5_nocase,
    sherlock::no_match_uncommon,
    sherlock::no_match_common,
    sherlock::no_match_really_common,
    sherlock::the_lower,
    sherlock::the_upper,
    sherlock::the_nocase,
    sherlock::the_whitespace,
    sherlock::everything_greedy,
    sherlock::everything_greedy_nl,
    sherlock::letters,
    sherlock::letters_upper,
    sherlock::letters_lower,
    sherlock::words,
    sherlock::before_holmes,
    sherlock::before_after_holmes,
    sherlock::holmes_cochar_watson,
    sherlock::holmes_coword_watson,
    sherlock::quotes,
    sherlock::line_boundary_sherlock_holmes,
    sherlock::word_ending_n,
    sherlock::repeated_class_negation,
    sherlock::ing_suffix,
    sherlock::ing_suffix_limited_space
}

criterion_main! {
    regex_0_2_6,
}
