use criterion::{criterion_group, criterion_main, Criterion};
use mrakdown_engine::schema::{HtmlSchema, Schema};

fn bench_header_generation(c: &mut Criterion) {
    c.bench_function("generate_h1_legacy", |b| {
        b.iter(|| {
            let start = HtmlSchema::h1_start();
            let end = HtmlSchema::h1_end();
            format!("{}Test{}", start, end)
        })
    });
}

criterion_group!(benches, bench_header_generation);
criterion_main!(benches);