use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hello_benchmark::vec_functions::{iterate_for_loop, iterate_iter};

fn benchmark(c: &mut Criterion) {
    
    let lotsa_numbas: Vec<i32> = (0..1000000).collect();

    c.bench_function("for loop", |b| b.iter(|| iterate_for_loop(black_box(&lotsa_numbas))));
    c.bench_function("iter method", |b| b.iter(|| iterate_iter(black_box(&lotsa_numbas))));
}

fn criterion_benchmark() -> Criterion {
    Criterion::default()
        .sample_size(100) // Set the sample size to 10
}

criterion_group! {
    name = benches;
    config = criterion_benchmark();
    targets = benchmark
}
criterion_main!(benches);