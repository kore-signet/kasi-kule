#![allow(non_upper_case_globals)]

use crate::{utils::*, HPE, LMS, XYZ};
use lazy_static::lazy_static;
use std::f32::consts::PI;

pub const D65_XYZ: XYZ = XYZ {
    x: 95.047,
    y: 100.0,
    z: 108.883,
};

lazy_static! {
    pub static ref D65_LMS: LMS = LMS::from(&D65_XYZ);
}

/// CIECAM02 viewing conditions
pub mod VC {
    use super::*;

    pub const la: f32 = (64.0 / PI) / 5.0;
    pub const yb: f32 = 20.0;
    pub const f: f32 = 1.0;
    pub const c: f32 = 0.69;
    pub const nc: f32 = 1.0;
    pub const n: f32 = yb / D65_XYZ.y;
    pub const k: f32 = 1.0 / ((5.0 * la) + 1.0);

    lazy_static! {
        pub static ref z: f32 = 1.48 + n.sqrt();
        pub static ref fl: f32 = (0.2 * k.powi(4) * (5.0 * la))
            + 0.1 * ((1.0 - k.powi(4)).powi(2)) * (5.0 * la).powf(1.0 / 3.0);
        pub static ref nbb: f32 = 0.725 * (1.0 / n).powf(0.2);
        pub static ref ncb: f32 = *nbb;
        pub static ref d: f32 = f * (1.0 - (1.0 / 3.6) * ((-la - 42.0) / 92.0).exp());
        pub static ref achromatic_response_to_white: f32 = {
            let lc = D65_LMS.l * (((D65_XYZ.y * *d) / D65_LMS.l) + (1.0 - *d));
            let mc = D65_LMS.m * (((D65_XYZ.y * *d) / D65_LMS.m) + (1.0 - *d));
            let sc = D65_LMS.s * (((D65_XYZ.y * *d) / D65_LMS.s) + (1.0 - *d));

            let hpe = HPE::from(&LMS {
                l: lc,
                m: mc,
                s: sc,
            });
            let lpa = nonlinear_adaptation(hpe.lh, *fl);
            let mpa = nonlinear_adaptation(hpe.mh, *fl);
            let spa = nonlinear_adaptation(hpe.sh, *fl);

            (2.0 * lpa + mpa + 0.05 * spa - 0.305) * *nbb
        };
    }
}

pub mod UCS {
    pub const k_l: f32 = 1.0;
    pub const c1: f32 = 0.007;
    pub const c2: f32 = 0.0228;
}

// // ciecam02 viewing conditions
// #[derive(Debug, Copy, Clone)]
// pub struct CIECAM02_VC {
//     D65_XYZ: XYZ,
//     D65_LMS: LMS,
//     la: f32,
//     yb: f32,
//     f: f32,
//     c: f32,
//     nc: f32,
//     n: f32,
//     z: f32,
//     fl: f32,
//     nbb: f32,
//     ncb: f32,
//     d: f32,
//     achromatic_response_to_white: f32,
// }

// impl CIECAM02_VC {
//     pub fn new() -> CIECAM02_VC {
//         let D65_XYZ =
// let D65_LMS = xyz_to_cat02(&D65_XYZ);

//         CIECAM02_VC {
//             D65_XYZ,
//             D65_LMS,
//             la,
//             yb,
//             f,
//             c,
//             nc,
//             z,
//             fl,
//             nbb,
//             ncb,
//             d,
//             achromatic_response_to_white,
//             n,
//         }
//     }
// }
