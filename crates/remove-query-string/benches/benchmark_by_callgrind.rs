use gungraun::{library_benchmark, library_benchmark_group, main};

use std::hint::black_box;

use remove_query_string::*;

#[library_benchmark]
#[bench::normal("abc&cd=ef&world", "cd")]
fn bench_reference_fn(query_str: &str, key: &str) -> String {
    black_box(remove_query_string(query_str, key))
}

#[library_benchmark]
#[bench::normal("abc&cd=ef&world", "cd")]
fn bench_strip_prefix(query_str: &str, key: &str) -> String {
    black_box(remove_query_string_with_strip_prefix(query_str, key))
}

library_benchmark_group!(
    name = bench_remove_query_string;
    benchmarks =
        bench_reference_fn,
        bench_strip_prefix,
);

main!(library_benchmark_groups = bench_remove_query_string);
