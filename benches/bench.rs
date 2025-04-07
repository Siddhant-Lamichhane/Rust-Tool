use criterion::{black_box, criterion_group, criterion_main, Criterion};

// A simple function to add numbers from a slice.
fn add_numbers(numbers: &[f64]) -> f64 {
    numbers.iter().sum()
}

// This function sets up the benchmark.
fn criterion_benchmark(c: &mut Criterion) {
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    c.bench_function("add_numbers", |b| {
        b.iter(|| add_numbers(black_box(&data)))
    });
}

// Macro to define benchmark groups.
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
