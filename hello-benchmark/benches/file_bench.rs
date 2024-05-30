use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use hello_benchmark::file_functions::{read_then_process, read_while_processing};

static BIG_FILE_PATH: &str = "src/big_file.txt";

fn benchmark(c: &mut Criterion) {
    c.bench_function("read file fully, then process", |b| b.iter(|| read_then_process(black_box(&BIG_FILE_PATH))));
    c.bench_function("read file while processing", |b| b.iter(|| read_while_processing(black_box(&BIG_FILE_PATH))));
}

fn criterion_benchmark() -> Criterion {
    Criterion::default()
        .sample_size(10) // Set the sample size to 10 because the functions take a long time
        .measurement_time(Duration::new(40, 0))
}

criterion_group! {
    name = benches;
    config = criterion_benchmark();
    targets = benchmark
}
criterion_main!(benches);