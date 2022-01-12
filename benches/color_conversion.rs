use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kasi_kule::{JCh, Jab, LinearRGB, LMS, UCS, XYZ};
use lab::Lab;

fn colors(c: &mut Criterion) {
    let mut group = c.benchmark_group("color conversion");
    let color = [fastrand::u8(..), fastrand::u8(..), fastrand::u8(..)];
    let lrgb = LinearRGB::from(color);
    let xyz = XYZ::from(color);
    let lms = LMS::from(color);
    let jch = JCh::from(color);

    group.bench_function("rgb -> lab (reference)", |b| {
        b.iter(|| black_box(Lab::from_rgb(&color)))
    });
    group.bench_function("rgb -> jab (ucs)", |b| {
        b.iter(|| black_box(Jab::<UCS>::from(color)))
    });
    group.bench_function("rgb -> linear rgb", |b| {
        b.iter(|| black_box(LinearRGB::from(color)))
    });
    group.bench_function("linear rgb -> xyz", |b| {
        b.iter(|| black_box(XYZ::from(&lrgb)))
    });
    group.bench_function("xyz -> lms", |b| b.iter(|| black_box(LMS::from(&xyz))));
    group.bench_function("lms -> jch", |b| b.iter(|| black_box(JCh::from(&lms))));
    group.bench_function("jch -> jab (ucs)", |b| {
        b.iter(|| black_box(Jab::<UCS>::from(&jch)))
    });
    group.finish();
}

criterion_group!(benches, colors);
criterion_main!(benches);
