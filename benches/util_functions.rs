use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kasi_kule::{consts, sse, utils, HPE, LMS};

fn transform_scalar(v: [f32; 4]) -> [f32; 4] {
    [
        utils::c_transform(v[0], consts::D65_LMS.l),
        utils::c_transform(v[1], consts::D65_LMS.m),
        utils::c_transform(v[2], consts::D65_LMS.s),
        0.0,
    ]
}

fn adapt_scalar(cones: [f32; 4]) -> [f32; 4] {
    [
        utils::nonlinear_adaptation_scalar(cones[0], consts::VC::fl),
        utils::nonlinear_adaptation_scalar(cones[1], consts::VC::fl),
        utils::nonlinear_adaptation_scalar(cones[2], consts::VC::fl),
        0.0,
    ]
}

fn cones_transform(c: &mut Criterion) {
    let mut group = c.benchmark_group("cone transform");
    let lms = LMS {
        l: 3.346795,
        m: 3.9057755,
        s: 0.61963636,
    };
    let lms = [lms.l, lms.m, lms.s, 0.0];
    group.bench_function("scalar", |b| b.iter(|| black_box(transform_scalar(lms))));
    group.bench_function("sse", |b| {
        b.iter(|| black_box(unsafe { sse::sse_transform_cones(lms) }))
    });
    group.finish();
}

fn nonlinear_adapt(c: &mut Criterion) {
    let mut group = c.benchmark_group("nonlinear adaptation");

    let color = [fastrand::u8(..), fastrand::u8(..), fastrand::u8(..)];
    let hpe = HPE::from(color);
    let hpe = [hpe.lh, hpe.mh, hpe.sh, 0.0];
    group.bench_function("scalar", |b| b.iter(|| black_box(adapt_scalar(hpe))));
    group.finish();
}

criterion_group!(cones, cones_transform, nonlinear_adapt);
criterion_main!(cones);
