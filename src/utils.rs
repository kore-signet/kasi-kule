#![allow(dead_code)]

use crate::consts::{self, VC};

#[inline(always)]
pub(crate) fn linearize_channel(c: u8) -> f32 {
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

pub(crate) fn c_transform(cone: f32, d65_cone: f32) -> f32 {
    cone * (((consts::D65_XYZ.y * *VC::d) / d65_cone) + (1.0f32 - *VC::d))
}
