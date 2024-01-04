use iai::main;

use remove_query_string::*;

fn bench_remove_query_string() -> String {
    let query_str = "abc&cd=ef&world";
    remove_query_string(query_str, &"cd")
}

fn bench_remove_query_string_with_strip_prefix() -> String {
    let query_str = "abc&cd=ef&world";
    remove_query_string_with_strip_prefix(query_str, &"cd")
}

iai::main!(
    bench_remove_query_string,
    bench_remove_query_string_with_strip_prefix,
);
