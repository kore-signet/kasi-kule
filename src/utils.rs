#![allow(dead_code)]

use crate::consts::{self, VC};

#[cfg(feature = "approximates")]
#[allow(unused_imports)]
use micromath::F32Ext;

pub fn linearize_channel(c: u8) -> f32 {
    let c = c as f32 / 255.0;
    if c > 0.04045 {
        ((c + 0.055) / 1.055).powf(2.4)
    } else {
        c / 12.92
    }
}

#[inline(always)]
pub(crate) fn nonlinear_adaptation(cones: [f32; 4], fl: f32) -> [f32; 4] {
    // #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    // {
    //     unsafe {
    //         if is_x86_feature_detected!("sse2") {
    //             return sse::sse2_nonlinear_adaptation(cones, fl);
    //         }
    //     }
    // }

    [
        nonlinear_adaptation_scalar(cones[0], fl),
        nonlinear_adaptation_scalar(cones[1], fl),
        nonlinear_adaptation_scalar(cones[2], fl),
        0.0,
    ]
}

#[inline(always)]
pub fn nonlinear_adaptation_scalar(cone_response: f32, fl: f32) -> f32 {
    let p = ((fl * cone_response) / 100.0).powf(0.42);
    ((400.0 * p) / (27.13 + p)) + 0.1
}

#[inline(always)]
pub fn c_transform(cone: f32, d65_cone: f32) -> f32 {
    cone * (((consts::D65_XYZ.y * VC::d) / d65_cone) + (1.0f32 - VC::d))
}

#[inline(always)]
pub fn transform_cones(cones: [f32; 4]) -> [f32; 4] {
    [
        c_transform(cones[0], consts::D65_LMS.l),
        c_transform(cones[1], consts::D65_LMS.m),
        c_transform(cones[2], consts::D65_LMS.s),
        0.0,
    ]
}

// #[inline(always)]
// pub fn transform_cones(cones: [f32; 4]) -> [f32; 4] {
//     // #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
//     // {
//     //     unsafe {
//     //         if is_x86_feature_detected!("sse") {
//     //             return sse::sse_transform_cones(cones);
//     //         }
//     //     }
//     // }

//     transform_cones_scalar(cones)
// }
