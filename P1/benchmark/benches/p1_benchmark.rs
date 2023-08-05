use benchmark::bm;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p1 benchmark", |b| {
        b.iter(|| {
            bm(
                black_box(vec![20, 49, 24, 03, 08, 23, 60, 56, 17]),
                black_box(vec![51, 11, 01, 52, 39, 42, 07, 16, 14]),
                black_box(51),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
