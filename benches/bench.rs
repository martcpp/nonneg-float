use criterion::{criterion_group, criterion_main, Criterion};
use nonneg_float::NonNegative;

fn bench_try_new(c: &mut Criterion) {
    c.bench_function("try_nonneg 1.0", |b| {
        b.iter(|| {
            let value = std::hint::black_box(1.0f64);              // input
            let result = NonNegative::try_new(value).unwrap();  // run the code
            std::hint::black_box(result);                          // prevent compiler optimizations
        })
    });
}

criterion_group!(benches, bench_try_new);
criterion_main!(benches);
