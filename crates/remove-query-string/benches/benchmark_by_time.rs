use criterion::{criterion_group, criterion_main, Criterion};

use remove_query_string::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("format", move |b| {
        let query_str = "abc&cd=ef&world";
        b.iter(|| remove_query_string(query_str, &"cd"))
    });

    c.bench_function("strip_prefix", move |b| {
        let query_str = "abc&cd=ef&world";
        b.iter(|| remove_query_string_with_strip_prefix(query_str, &"cd"))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
