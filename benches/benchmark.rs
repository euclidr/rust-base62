#[macro_use]
extern crate criterion;
extern crate base62;
use criterion::black_box;
use criterion::Criterion;

fn bench_encode(c: &mut Criterion) {
    c.bench_function("encode", |b| {
        b.iter(|| base62::encode(black_box(4294967295)))
    });
}

fn bench_decode(c: &mut Criterion) {
    c.bench_function("decode", |b| {
        b.iter(|| base62::decode(black_box(&String::from("4gfFC3"))))
    });
}

criterion_group!(benches, bench_encode, bench_decode);
criterion_main!(benches);
