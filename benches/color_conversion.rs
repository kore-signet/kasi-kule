use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kasi_kule::{Jab, UCS};
use lab::Lab;

fn colors(c: &mut Criterion) {
    let mut group = c.benchmark_group("color conversion");
    let color = [fastrand::u8(..), fastrand::u8(..), fastrand::u8(..)];
    group.bench_function("lab", |b| b.iter(|| black_box(Lab::from_rgb(&color))));
    group.bench_function("jab", |b| b.iter(|| black_box(Jab::<UCS>::from(color))));
    group.finish();
}

criterion_group!(benches, colors);
criterion_main!(benches);
