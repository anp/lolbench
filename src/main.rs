#[macro_use]
extern crate criterion;
use criterion::Criterion;

extern crate diesel_1_1_1;
extern crate doom_9e197d7;
extern crate inflate_0_3_4;
extern crate json_benchmark_c7d3d9b;
extern crate nom_4_0_0_beta1;
extern crate rayon_1_0_0;
extern crate raytrace_8de9020;
extern crate snap_0_2_4;

criterion_group!(
    diesel_1_1_1,
    diesel_1_1_1::bench_trivial_query_selecting______1_row,
    diesel_1_1_1::bench_trivial_query_selecting______1_row_boxed,
    diesel_1_1_1::bench_trivial_query_selecting_____10_rows,
    diesel_1_1_1::bench_trivial_query_selecting_____10_rows_boxed,
    diesel_1_1_1::bench_trivial_query_selecting____100_rows,
    diesel_1_1_1::bench_trivial_query_selecting____100_rows_boxed,
    diesel_1_1_1::bench_trivial_query_selecting__1_000_rows,
    diesel_1_1_1::bench_trivial_query_selecting__1_000_rows_boxed,
    diesel_1_1_1::bench_trivial_query_selecting_10_000_rows,
    diesel_1_1_1::bench_trivial_query_selecting_10_000_rows_boxed,
    diesel_1_1_1::bench_medium_complex_query_selecting______1_row,
    diesel_1_1_1::bench_medium_complex_query_selecting______1_row_boxed,
    diesel_1_1_1::bench_medium_complex_query_selecting_____10_rows,
    diesel_1_1_1::bench_medium_complex_query_selecting_____10_rows_boxed,
    diesel_1_1_1::bench_medium_complex_query_selecting____100_rows,
    diesel_1_1_1::bench_medium_complex_query_selecting____100_rows_boxed,
    diesel_1_1_1::bench_medium_complex_query_selecting__1_000_rows,
    diesel_1_1_1::bench_medium_complex_query_selecting__1_000_rows_boxed,
    diesel_1_1_1::bench_medium_complex_query_selecting_10_000_rows,
    diesel_1_1_1::bench_medium_complex_query_selecting_10_000_rows_boxed,
    diesel_1_1_1::loading_associations_sequentially
);

criterion_group!(
    doom_9e197d7,
    doom_9e197d7::freedoom1,
    doom_9e197d7::freedoom2
);

criterion_group!(inflate_0_3_4, inflate_0_3_4::decode);

criterion_group!(
    json_benchmark_c7d3d9b,
    json_benchmark_c7d3d9b::serde_canada_dom,
    json_benchmark_c7d3d9b::serde_canada_struct,
    json_benchmark_c7d3d9b::serde_citm_dom,
    json_benchmark_c7d3d9b::serde_citm_struct,
    json_benchmark_c7d3d9b::serde_twitter_dom,
    json_benchmark_c7d3d9b::serde_twitter_struct,
    json_benchmark_c7d3d9b::serialize_canada_dom,
    json_benchmark_c7d3d9b::serialize_canada_struct,
    json_benchmark_c7d3d9b::serialize_citm_dom,
    json_benchmark_c7d3d9b::serialize_citm_struct,
    json_benchmark_c7d3d9b::serialize_twitter_dom,
    json_benchmark_c7d3d9b::serialize_twitter_struct
);

criterion_group!(
    nom_4_0_0_beta1,
    nom_4_0_0_beta1::arithmetic::arithmetic,
    nom_4_0_0_beta1::http::one_test,
    nom_4_0_0_beta1::ini_str::bench_ini_str,
    //nom_4_0_0_beta1::ini::bench_ini,
    nom_4_0_0_beta1::ini::bench_ini_keys_and_values,
    nom_4_0_0_beta1::ini::bench_ini_key_value
);

criterion_group!(
    rayon_1_0_0,
    rayon_1_0_0::factorial::factorial_iterator,
    rayon_1_0_0::factorial::factorial_par_iter,
    rayon_1_0_0::factorial::factorial_recursion,
    rayon_1_0_0::factorial::factorial_join,
    rayon_1_0_0::fibonacci::fibonacci_recursive,
    rayon_1_0_0::fibonacci::fibonacci_join_1_2,
    rayon_1_0_0::fibonacci::fibonacci_join_2_1,
    rayon_1_0_0::fibonacci::fibonacci_split_recursive,
    rayon_1_0_0::fibonacci::fibonacci_split_iterative,
    rayon_1_0_0::fibonacci::fibonacci_iterative,
    rayon_1_0_0::find::parallel_find_first,
    rayon_1_0_0::find::serial_find_first,
    rayon_1_0_0::find::parallel_find_last,
    rayon_1_0_0::find::serial_find_last,
    rayon_1_0_0::find::parallel_find_middle,
    rayon_1_0_0::find::serial_find_middle,
    rayon_1_0_0::find::parallel_find_missing,
    rayon_1_0_0::find::serial_find_missing,
    rayon_1_0_0::find::parallel_find_common,
    rayon_1_0_0::find::serial_find_common,
    rayon_1_0_0::join_microbench::increment_all,
    rayon_1_0_0::join_microbench::increment_all_min,
    rayon_1_0_0::join_microbench::increment_all_serialized,
    rayon_1_0_0::join_microbench::increment_all_max,
    rayon_1_0_0::join_microbench::increment_all_atomized,
    rayon_1_0_0::join_microbench::join_recursively,
    rayon_1_0_0::life::bench::generations,
    rayon_1_0_0::life::bench::parallel_generations,
    rayon_1_0_0::map_collect::i_to_i::with_collect,
    rayon_1_0_0::map_collect::i_to_i::with_mutex,
    rayon_1_0_0::map_collect::i_to_i::with_mutex_vec,
    rayon_1_0_0::map_collect::i_to_i::with_linked_list_collect,
    rayon_1_0_0::map_collect::i_to_i::with_linked_list_collect_vec,
    rayon_1_0_0::map_collect::i_to_i::with_linked_list_collect_vec_sized,
    rayon_1_0_0::map_collect::i_to_i::with_linked_list_map_reduce_vec_sized,
    rayon_1_0_0::map_collect::i_to_i::with_vec_vec_sized,
    rayon_1_0_0::map_collect::i_to_i::with_fold,
    rayon_1_0_0::map_collect::i_to_i::with_fold_vec,
    rayon_1_0_0::map_collect::i_mod_10_to_i::with_collect,
    rayon_1_0_0::map_collect::i_mod_10_to_i::with_mutex,
    rayon_1_0_0::map_collect::i_mod_10_to_i::with_mutex_vec,
    rayon_1_0_0::map_collect::i_mod_10_to_i::with_linked_list_collect,
    rayon_1_0_0::map_collect::i_mod_10_to_i::with_linked_list_collect_vec,
    rayon_1_0_0::map_collect::i_mod_10_to_i::with_linked_list_collect_vec_sized,
    rayon_1_0_0::map_collect::i_mod_10_to_i::with_linked_list_map_reduce_vec_sized,
    rayon_1_0_0::map_collect::i_mod_10_to_i::with_vec_vec_sized,
    rayon_1_0_0::map_collect::i_mod_10_to_i::with_fold,
    rayon_1_0_0::map_collect::i_mod_10_to_i::with_fold_vec,
    rayon_1_0_0::matmul::bench_matmul_strassen,
    rayon_1_0_0::mergesort::merge_sort_par_bench,
    rayon_1_0_0::mergesort::merge_sort_seq_bench,
    rayon_1_0_0::nbody::nbody_seq,
    rayon_1_0_0::nbody::nbody_par,
    rayon_1_0_0::pythagoras::euclid_serial,
    rayon_1_0_0::pythagoras::euclid_faux_serial,
    rayon_1_0_0::pythagoras::euclid_parallel_weightless,
    rayon_1_0_0::pythagoras::euclid_parallel_one,
    rayon_1_0_0::pythagoras::euclid_parallel_outer,
    rayon_1_0_0::pythagoras::euclid_parallel_full,
    rayon_1_0_0::quicksort::quick_sort_par_bench,
    rayon_1_0_0::quicksort::quick_sort_seq_bench,
    rayon_1_0_0::quicksort::quick_sort_splitter,
    rayon_1_0_0::sieve::bench::sieve_serial,
    rayon_1_0_0::sieve::bench::sieve_chunks,
    rayon_1_0_0::sieve::bench::sieve_parallel,
    rayon_1_0_0::sort::par_sort_ascending,
    rayon_1_0_0::sort::par_sort_descending,
    rayon_1_0_0::sort::par_sort_mostly_ascending,
    rayon_1_0_0::sort::par_sort_mostly_descending,
    rayon_1_0_0::sort::par_sort_random,
    rayon_1_0_0::sort::par_sort_big,
    rayon_1_0_0::sort::par_sort_strings,
    rayon_1_0_0::sort::par_sort_expensive,
    rayon_1_0_0::sort::par_sort_unstable_ascending,
    rayon_1_0_0::sort::par_sort_unstable_descending,
    rayon_1_0_0::sort::par_sort_unstable_mostly_ascending,
    rayon_1_0_0::sort::par_sort_unstable_mostly_descending,
    rayon_1_0_0::sort::par_sort_unstable_random,
    rayon_1_0_0::sort::par_sort_unstable_big,
    rayon_1_0_0::sort::par_sort_unstable_strings,
    rayon_1_0_0::sort::par_sort_unstable_expensive,
    rayon_1_0_0::sort::demo_merge_sort_ascending,
    rayon_1_0_0::sort::demo_merge_sort_descending,
    rayon_1_0_0::sort::demo_merge_sort_mostly_ascending,
    rayon_1_0_0::sort::demo_merge_sort_mostly_descending,
    rayon_1_0_0::sort::demo_merge_sort_random,
    rayon_1_0_0::sort::demo_merge_sort_big,
    rayon_1_0_0::sort::demo_merge_sort_strings,
    rayon_1_0_0::sort::demo_quick_sort_mostly_ascending,
    rayon_1_0_0::sort::demo_quick_sort_mostly_descending,
    rayon_1_0_0::sort::demo_quick_sort_random,
    rayon_1_0_0::sort::demo_quick_sort_big,
    rayon_1_0_0::sort::demo_quick_sort_strings,
    rayon_1_0_0::str_split::parallel_space_char,
    rayon_1_0_0::str_split::parallel_space_fn,
    rayon_1_0_0::str_split::serial_space_char,
    rayon_1_0_0::str_split::serial_space_fn,
    rayon_1_0_0::str_split::serial_space_str,
    rayon_1_0_0::tsp::dj10,
    rayon_1_0_0::vec_collect::vec_i::with_collect_into_vec,
    rayon_1_0_0::vec_collect::vec_i::with_collect_into_vec_reused,
    rayon_1_0_0::vec_collect::vec_i::with_collect,
    rayon_1_0_0::vec_collect::vec_i::with_linked_list_collect_vec,
    rayon_1_0_0::vec_collect::vec_i::with_linked_list_collect_vec_sized,
    rayon_1_0_0::vec_collect::vec_i::with_linked_list_map_reduce_vec_sized,
    rayon_1_0_0::vec_collect::vec_i::with_vec_vec_sized,
    rayon_1_0_0::vec_collect::vec_i::with_fold,
    rayon_1_0_0::vec_collect::vec_i_filtered::with_collect,
    rayon_1_0_0::vec_collect::vec_i_filtered::with_linked_list_collect_vec,
    rayon_1_0_0::vec_collect::vec_i_filtered::with_linked_list_collect_vec_sized,
    rayon_1_0_0::vec_collect::vec_i_filtered::with_linked_list_map_reduce_vec_sized,
    rayon_1_0_0::vec_collect::vec_i_filtered::with_vec_vec_sized,
    rayon_1_0_0::vec_collect::vec_i_filtered::with_fold
);

criterion_group!(raytrace_8de9020, raytrace_8de9020::raytrace_random_scenes);

criterion_group!(
    snap_0_2_4_rust,
    snap_0_2_4::rust::zflat00_html,
    snap_0_2_4::rust::zflat01_urls,
    snap_0_2_4::rust::zflat02_jpg,
    snap_0_2_4::rust::zflat03_jpg_200,
    snap_0_2_4::rust::zflat04_pdf,
    snap_0_2_4::rust::zflat05_html4,
    snap_0_2_4::rust::zflat06_txt1,
    snap_0_2_4::rust::zflat07_txt2,
    snap_0_2_4::rust::zflat08_txt3,
    snap_0_2_4::rust::zflat09_txt4,
    snap_0_2_4::rust::zflat10_pb,
    snap_0_2_4::rust::zflat11_gaviota,
    snap_0_2_4::rust::uflat00_html,
    snap_0_2_4::rust::uflat01_urls,
    snap_0_2_4::rust::uflat02_jpg,
    snap_0_2_4::rust::uflat03_jpg_200,
    snap_0_2_4::rust::uflat04_pdf,
    snap_0_2_4::rust::uflat05_html4,
    snap_0_2_4::rust::uflat06_txt1,
    snap_0_2_4::rust::uflat07_txt2,
    snap_0_2_4::rust::uflat08_txt3,
    snap_0_2_4::rust::uflat09_txt4,
    snap_0_2_4::rust::uflat10_pb,
    snap_0_2_4::rust::uflat11_gaviota
);

criterion_main!(
    nom_4_0_0_beta1,
    diesel_1_1_1,
    json_benchmark_c7d3d9b,
    rayon_1_0_0
    // doom_9e197d7,
    // inflate_0_3_4,
    // raytrace_8de9020,
    // snap_0_2_4_rust
);
