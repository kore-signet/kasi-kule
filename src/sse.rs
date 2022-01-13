use crate::{
    consts::{self, VC},
    LinearRGB, LMS, XYZ,
};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse")]
pub unsafe fn sse_xyz(rgb: &LinearRGB) -> [f32; 4] {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let r = _mm_mul_ps(_mm_setr_ps(0.4124, 0.2126, 0.0193, 0.0), _mm_set1_ps(rgb.r));
    let g = _mm_mul_ps(_mm_setr_ps(0.3576, 0.7152, 0.1192, 0.0), _mm_set1_ps(rgb.g));
    let b = _mm_mul_ps(_mm_setr_ps(0.1805, 0.0722, 0.9505, 0.0), _mm_set1_ps(rgb.b));

    std::mem::transmute::<__m128, [f32; 4]>(_mm_mul_ps(
        _mm_add_ps(_mm_add_ps(r, g), b),
        _mm_set1_ps(100.0),
    ))
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse")]
pub unsafe fn sse_lms(xyz: &XYZ) -> [f32; 4] {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let x = _mm_mul_ps(
        _mm_setr_ps(0.7328, -0.7036, 0.0030, 0.0),
        _mm_set1_ps(xyz.x),
    );
    let y = _mm_mul_ps(_mm_setr_ps(0.4296, 1.6975, 0.0136, 0.0), _mm_set1_ps(xyz.y));
    let z = _mm_mul_ps(
        _mm_setr_ps(-0.1624, 0.0061, 0.9834, 0.0),
        _mm_set1_ps(xyz.z),
    );

    std::mem::transmute::<__m128, [f32; 4]>(_mm_add_ps(_mm_add_ps(x, y), z))
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse")]
pub unsafe fn sse_hpe(lms: &LMS) -> [f32; 4] {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    let l = _mm_mul_ps(
        _mm_setr_ps(0.7409792, 0.2853532, -0.0096280, 0.0),
        _mm_set1_ps(lms.l),
    );
    let m = _mm_mul_ps(
        _mm_setr_ps(0.2180250, 0.6242014, -0.0056980, 0.0),
        _mm_set1_ps(lms.m),
    );
    let s = _mm_mul_ps(
        _mm_setr_ps(0.0410058, 0.0904454, 1.0153260, 0.0),
        _mm_set1_ps(lms.s),
    );

    std::mem::transmute::<__m128, [f32; 4]>(_mm_add_ps(_mm_add_ps(l, m), s))
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse")]
pub unsafe fn transform_cones_sse(cones: [f32; 4]) -> [f32; 4] {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    std::mem::transmute::<__m128, [f32; 4]>(_mm_mul_ps(
        _mm_load_ps(cones[..].as_ptr()),
        _mm_add_ps(
            _mm_div_ps(
                _mm_set1_ps(consts::D65_XYZ.y * VC::d),
                _mm_setr_ps(consts::D65_LMS.l, consts::D65_LMS.m, consts::D65_LMS.s, 0.0),
            ),
            _mm_set1_ps(1.0 - VC::d),
        ),
    ))
}
