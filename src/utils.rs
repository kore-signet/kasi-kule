#![allow(dead_code)]

use crate::consts::{self, VC};

pub fn linearize_channel(c: u8) -> f32 {
    let c = c as f32 / 255.0;
    if c > 0.04045 {
        ((c + 0.055) / 1.055).powf(2.4)
    } else {
        c / 12.92
    }
}

pub(crate) fn nonlinear_adaptation(cone_response: f32, fl: f32) -> f32 {
    let p = ((fl * cone_response) / 100.0).powf(0.42);
    ((400.0 * p) / (27.13 + p)) + 0.1
}

pub(crate) fn inverse_nonlinear_adaptation(cone_response: f32, fl: f32) -> f32 {
    (100.0 / fl)
        * ((27.13 * (cone_response - 0.1).abs()) / (400.0 - (cone_response - 0.1).abs()))
            .powf(1.0 / 0.42)
}

pub fn c_transform(cone: f32, d65_cone: f32) -> f32 {
    cone * (((consts::D65_XYZ.y * VC::d) / d65_cone) + (1.0f32 - VC::d))
}

pub fn transform_cones(cones: [f32; 4]) -> [f32; 4] {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        unsafe {
            if is_x86_feature_detected!("sse") {
                return transform_cones_sse(cones);
            }
        }
    }

    [
        c_transform(cones[0], consts::D65_LMS.l),
        c_transform(cones[1], consts::D65_LMS.m),
        c_transform(cones[2], consts::D65_LMS.s),
        0.0,
    ]
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
