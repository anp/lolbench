pub mod life;
pub mod matmul;
pub mod mergesort;
pub mod nbody;
pub mod quicksort;
pub mod sieve;
pub mod tsp;

// these are not "full-fledged" benchmarks yet,
// they only run with cargo bench
pub mod factorial;
pub mod fibonacci;
pub mod find;
pub mod join_microbench;
pub mod map_collect;
pub mod pythagoras;
pub mod sort;
pub mod str_split;
pub mod vec_collect;

extern crate cgmath; // nbody
#[macro_use]
extern crate criterion;
extern crate fixedbitset; // tsp
#[macro_use]
extern crate glium; // nbody
#[macro_use]
extern crate lazy_static; // find
extern crate num;   // factorial
extern crate odds;  // sieve
extern crate rand;  // nbody
extern crate rayon; // all
extern crate regex; // tsp
extern crate serde; // all
#[macro_use]
extern crate serde_derive; // all
extern crate time; // nbody, sieve
#[macro_use]
extern crate wrap_libtest;

use criterion::Criterion;

criterion_group! {
    rayon_1_0_0,
    factorial::factorial_iterator,
    factorial::factorial_par_iter,
    factorial::factorial_recursion,
    factorial::factorial_join,
    fibonacci::fibonacci_recursive,
    fibonacci::fibonacci_join_1_2,
    fibonacci::fibonacci_join_2_1,
    fibonacci::fibonacci_split_recursive,
    fibonacci::fibonacci_split_iterative,
    fibonacci::fibonacci_iterative,
    find::parallel_find_first,
    find::serial_find_first,
    find::parallel_find_last,
    find::serial_find_last,
    find::parallel_find_middle,
    find::serial_find_middle,
    find::parallel_find_missing,
    find::serial_find_missing,
    find::parallel_find_common,
    find::serial_find_common,
    join_microbench::increment_all,
    join_microbench::increment_all_min,
    join_microbench::increment_all_serialized,
    join_microbench::increment_all_max,
    join_microbench::increment_all_atomized,
    join_microbench::join_recursively,
    life::bench::generations,
    life::bench::parallel_generations,
    map_collect::i_to_i::with_collect,
    map_collect::i_to_i::with_mutex,
    map_collect::i_to_i::with_mutex_vec,
    map_collect::i_to_i::with_linked_list_collect,
    map_collect::i_to_i::with_linked_list_collect_vec,
    map_collect::i_to_i::with_linked_list_collect_vec_sized,
    map_collect::i_to_i::with_linked_list_map_reduce_vec_sized,
    map_collect::i_to_i::with_vec_vec_sized,
    map_collect::i_to_i::with_fold,
    map_collect::i_to_i::with_fold_vec,
    map_collect::i_mod_10_to_i::with_collect,
    map_collect::i_mod_10_to_i::with_mutex,
    map_collect::i_mod_10_to_i::with_mutex_vec,
    map_collect::i_mod_10_to_i::with_linked_list_collect,
    map_collect::i_mod_10_to_i::with_linked_list_collect_vec,
    map_collect::i_mod_10_to_i::with_linked_list_collect_vec_sized,
    map_collect::i_mod_10_to_i::with_linked_list_map_reduce_vec_sized,
    map_collect::i_mod_10_to_i::with_vec_vec_sized,
    map_collect::i_mod_10_to_i::with_fold,
    map_collect::i_mod_10_to_i::with_fold_vec,
    matmul::bench_matmul_strassen,
    mergesort::merge_sort_par_bench,
    mergesort::merge_sort_seq_bench,
    nbody::nbody_seq,
    nbody::nbody_par,
    pythagoras::euclid_serial,
    pythagoras::euclid_faux_serial,
    pythagoras::euclid_parallel_weightless,
    pythagoras::euclid_parallel_one,
    pythagoras::euclid_parallel_outer,
    pythagoras::euclid_parallel_full,
    quicksort::quick_sort_par_bench,
    quicksort::quick_sort_seq_bench,
    quicksort::quick_sort_splitter,
    sieve::bench::sieve_serial,
    sieve::bench::sieve_chunks,
    sieve::bench::sieve_parallel,
    sort::par_sort_ascending,
    sort::par_sort_descending,
    sort::par_sort_mostly_ascending,
    sort::par_sort_mostly_descending,
    sort::par_sort_random,
    sort::par_sort_big,
    sort::par_sort_strings,
    sort::par_sort_expensive,
    sort::par_sort_unstable_ascending,
    sort::par_sort_unstable_descending,
    sort::par_sort_unstable_mostly_ascending,
    sort::par_sort_unstable_mostly_descending,
    sort::par_sort_unstable_random,
    sort::par_sort_unstable_big,
    sort::par_sort_unstable_strings,
    sort::par_sort_unstable_expensive,
    sort::demo_merge_sort_ascending,
    sort::demo_merge_sort_descending,
    sort::demo_merge_sort_mostly_ascending,
    sort::demo_merge_sort_mostly_descending,
    sort::demo_merge_sort_random,
    sort::demo_merge_sort_big,
    sort::demo_merge_sort_strings,
    sort::demo_quick_sort_mostly_ascending,
    sort::demo_quick_sort_mostly_descending,
    sort::demo_quick_sort_random,
    sort::demo_quick_sort_big,
    sort::demo_quick_sort_strings,
    str_split::parallel_space_char,
    str_split::parallel_space_fn,
    str_split::serial_space_char,
    str_split::serial_space_fn,
    str_split::serial_space_str,
    // Benchmarking rayon_1_0_0::tsp::bench::dj10: Warming up for 3.0000 sthread 'main' panicked
    // at 'called `Option::unwrap()` on a `None` value', /checkout/src/libcore/option.rs:335:21
    // rayon_1_0_0::tsp::dj10,
    vec_collect::vec_i::with_collect_into_vec,
    vec_collect::vec_i::with_collect_into_vec_reused,
    vec_collect::vec_i::with_collect,
    vec_collect::vec_i::with_linked_list_collect_vec,
    vec_collect::vec_i::with_linked_list_collect_vec_sized,
    vec_collect::vec_i::with_linked_list_map_reduce_vec_sized,
    vec_collect::vec_i::with_vec_vec_sized,
    vec_collect::vec_i::with_fold,
    vec_collect::vec_i_filtered::with_collect,
    vec_collect::vec_i_filtered::with_linked_list_collect_vec,
    vec_collect::vec_i_filtered::with_linked_list_collect_vec_sized,
    vec_collect::vec_i_filtered::with_linked_list_map_reduce_vec_sized,
    vec_collect::vec_i_filtered::with_vec_vec_sized,
    vec_collect::vec_i_filtered::with_fold
}

criterion_main! { rayon_1_0_0, }
