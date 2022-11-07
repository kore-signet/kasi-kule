#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

//! kasi-kule is a small rust implementation of the [CIECAM02 color space](https://en.wikipedia.org/wiki/CIECAM02) and conversion to it from standard RGB.
//! It is based on the [d3-cam02](https://github.com/connorgr/d3-cam02/) and [colorspacious](https://github.com/njsmith/colorspacious).
//!
//! The name, kasi-kule, is a translation of 'flower' into toki pona - literally, 'colorful plant'.
//! o sitelen pona!
use std::f32::consts::PI;
use std::marker::PhantomData;
pub mod consts;
pub mod utils;
use consts::VC;
pub use consts::{LCD, SCD, UCS};
use utils::*;
#[cfg(all(feature = "sse", any(target_arch = "x86", target_arch = "x86_64")))]
pub mod sse;

#[cfg(feature = "approximate_math")]
#[allow(unused_imports)]
use micromath::F32Ext;

/// sRGB color, in the 0-255 range.
#[derive(Default, Debug, Copy, Clone)]
pub struct sRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<[u8; 3]> for sRGB {
    fn from(rgb: [u8; 3]) -> sRGB {
        sRGB {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }
}

impl From<(u8, u8, u8)> for sRGB {
    fn from(rgb: (u8, u8, u8)) -> sRGB {
        sRGB {
            r: rgb.0,
            g: rgb.1,
            b: rgb.2,
        }
    }
}

/// Linearized RGB, scaled from sRGB
#[derive(Default, Debug, Copy, Clone)]
pub struct LinearRGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<&sRGB> for LinearRGB {
    fn from(srgb: &sRGB) -> LinearRGB {
        // safety: bounds checked by type; array is u8::MAX-sized and indexes are u8s
        unsafe {
            LinearRGB {
                r: *consts::sRGB_LOOKUP.get_unchecked(srgb.r as usize),
                g: *consts::sRGB_LOOKUP.get_unchecked(srgb.g as usize),
                b: *consts::sRGB_LOOKUP.get_unchecked(srgb.b as usize),
            }
        }
    }
}

impl<T: Into<sRGB>> From<T> for LinearRGB {
    fn from(rgb: T) -> LinearRGB {
        LinearRGB::from(&rgb.into())
    }
}

/// CIEXYZ 1931 Color space, in the 0-100 range.
#[derive(Debug, Copy, Clone)]
pub struct XYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<&LinearRGB> for XYZ {
    fn from(rgb: &LinearRGB) -> XYZ {
        #[cfg(all(feature = "sse",any(target_arch = "x86", target_arch = "x86_64")))]
        {
            unsafe {
                if is_x86_feature_detected!("sse") {
                    let res = sse::sse_xyz(rgb);
                    return XYZ {
                        x: res[0],
                        y: res[1],
                        z: res[2],
                    };
                }
            }
        }

        XYZ {
            x: ((rgb.r * 0.4124) + (rgb.g * 0.3576) + (rgb.b * 0.1805)) * 100.0,
            y: ((rgb.r * 0.2126) + (rgb.g * 0.7152) + (rgb.b * 0.0722)) * 100.0,
            z: ((rgb.r * 0.0193) + (rgb.g * 0.1192) + (rgb.b * 0.9505)) * 100.0,
        }
    }
}

impl<T: Into<sRGB>> From<T> for XYZ {
    fn from(rgb: T) -> XYZ {
        XYZ::from(&LinearRGB::from(&rgb.into()))
    }
}

/// Long-Medium-Short color space, derived from XYZ using the Mcat02 matrix.
#[derive(Debug, Copy, Clone)]
pub struct LMS {
    pub l: f32,
    pub m: f32,
    pub s: f32,
}

impl From<&XYZ> for LMS {
    fn from(xyz: &XYZ) -> LMS {
        #[cfg(all(feature = "sse",any(target_arch = "x86", target_arch = "x86_64")))]
        {
            unsafe {
                if is_x86_feature_detected!("sse") {
                    let res = sse::sse_lms(xyz);
                    return LMS {
                        l: res[0],
                        m: res[1],
                        s: res[2],
                    };
                }
            }
        }

        LMS {
            l: (0.7328 * xyz.x) + (0.4296 * xyz.y) - (0.1624 * xyz.z),
            m: (-0.7036 * xyz.x) + (1.6975 * xyz.y) + (0.0061 * xyz.z),
            s: (0.0030 * xyz.x) + (0.0136 * xyz.y) + (0.9834 * xyz.z),
        }
    }
}

impl<T: Into<sRGB>> From<T> for LMS {
    fn from(rgb: T) -> LMS {
        LMS::from(&XYZ::from(&LinearRGB::from(&rgb.into())))
    }
}

/// Hunt-Pointer-Estevez space, derived from CAM02 LMS.
#[derive(Debug, Copy, Clone)]
pub struct HPE {
    pub lh: f32,
    pub mh: f32,
    pub sh: f32,
}

impl From<&LMS> for HPE {
    fn from(lms: &LMS) -> HPE {
        #[cfg(all(feature = "sse",any(target_arch = "x86", target_arch = "x86_64")))]
        {
            unsafe {
                if is_x86_feature_detected!("sse") {
                    let res = sse::sse_hpe(lms);
                    return HPE {
                        lh: res[0],
                        mh: res[1],
                        sh: res[2],
                    };
                }
            }
        }

        HPE {
            lh: (0.7409792 * lms.l) + (0.2180250 * lms.m) + (0.0410058 * lms.s),
            mh: (0.2853532 * lms.l) + (0.6242014 * lms.m) + (0.0904454 * lms.s),
            sh: (-0.0096280 * lms.l) - (0.0056980 * lms.m) + (1.0153260 * lms.s),
        }
    }
}

impl<T: Into<sRGB>> From<T> for HPE {
    fn from(rgb: T) -> HPE {
        HPE::from(&LMS::from(&XYZ::from(&LinearRGB::from(&rgb.into()))))
    }
}

/// The CIECAM02 JCh (Lightness, Chroma, Hue) color space, derived from LMS.
#[derive(Default, Debug, Copy, Clone)]
pub struct JCh {
    pub J: f32,
    pub C: f32,
    pub H: f32,
    pub h: f32,
    pub Q: f32,
    pub M: f32,
    pub s: f32,
}

impl From<&LMS> for JCh {
    fn from(lms: &LMS) -> JCh {
        let [lc, mc, sc, _] = transform_cones([lms.l, lms.m, lms.s, 0.0]);

        let hpe_transforms = HPE::from(&LMS {
            l: lc,
            m: mc,
            s: sc,
        });

        let [lpa, mpa, spa, _] = nonlinear_adaptation(
            [hpe_transforms.lh, hpe_transforms.mh, hpe_transforms.sh, 0.0],
            VC::fl,
        );

        let ca = lpa - ((12.0 * mpa) / 11.0) + (spa / 11.0);
        let cb = (1.0 / 9.0) * (lpa + mpa - 2.0 * spa);

        let mut result_color = JCh::default();

        result_color.h = (180.0 / PI) * cb.atan2(ca);
        if result_color.h < 0.0 {
            result_color.h += 360.0;
        }

        let H = match result_color.h {
            h if h < 20.14 => {
                let temp = ((h + 122.47) / 1.2) + ((20.14 - h) / 0.8);
                300.0 + (100.0 * ((h + 122.47) / 1.2)) / temp
            }
            h if h < 90.0 => {
                let temp = ((h - 20.14) / 0.8) + ((90.0 - h) / 0.7);
                (100.0 * ((h - 20.14) / 0.8)) / temp
            }

            h if h < 164.25 => {
                let temp = ((h - 90.0) / 0.7) + ((164.25 - h) / 1.0);
                100.0 + ((100.0 * ((h - 90.0) / 0.7)) / temp)
            }
            h if h < 237.53 => {
                let temp = ((h - 164.25) / 1.0) + ((237.53 - h) / 1.2);
                200.0 + ((100.0 * ((h - 164.25) / 1.0)) / temp)
            }
            h => {
                let temp = ((h - 237.53) / 1.2) + ((360.0 - h + 20.14) / 0.8);
                300.0 + ((100.0 * ((h - 237.53) / 1.2)) / temp)
            }
        };

        result_color.H = H;

        let a = (2.0 * lpa + mpa + 0.05 * spa - 0.305) * VC::nbb;
        result_color.J = 100.0 * (a / VC::achromatic_response_to_white).powf(VC::c * VC::z);

        let et = 0.25 * (((result_color.h * PI) / 180.0 + 2.0).cos() + 3.8);
        let t = (50000.0 / 13.0) * VC::nc * VC::ncb * et * (ca.powi(2) + cb.powi(2)).sqrt()
            / (lpa + mpa + (21.0 / 20.0) * spa);

        result_color.C = t.powf(0.9f32)
            * (result_color.J / 100.0).sqrt()
            * (1.64 - 0.29f32.powf(VC::n)).powf(0.73f32);

        result_color.Q = (4.0 / VC::c)
            * (result_color.J / 100.0).sqrt()
            * (VC::achromatic_response_to_white + 4.0f32)
            * VC::fl.powf(0.25f32);

        result_color.M = result_color.C * VC::fl.powf(0.25f32);

        result_color.s = 100.0 * (result_color.M / result_color.Q).sqrt();

        result_color
    }
}

impl<T: Into<sRGB>> From<T> for JCh {
    fn from(rgb: T) -> JCh {
        JCh::from(&LMS::from(&XYZ::from(&LinearRGB::from(&rgb.into()))))
    }
}

/// the JabSpace defines constants for transformation from JCh space into JabSpace. Used for type-checking comparisons between Jab colors.
pub trait JabSpace {
    const k_l: f32;
    const c1: f32;
    const c2: f32;
}

/// The CAM02 Jab color appearance model.
/// It can be transformed from JCh space into an approximately perceptually uniform space (UCS), or into a space optimized for either LCD (Large Color Differences) or SCD (Small Color Differences).
/// Subsequent calculations of color difference must be between colors within the same space (UCS/LCD/SCD).
#[derive(Default, Debug, Copy, Clone)]
pub struct Jab<S: JabSpace> {
    pub J: f32,
    pub a: f32,
    pub b: f32,
    space: PhantomData<S>,
}

impl<S: JabSpace> From<&JCh> for Jab<S> {
    fn from(cam02: &JCh) -> Jab<S> {
        let j_prime = ((1.0 + 100.0 * S::c1) * cam02.J) / (1.0 + S::c1 * cam02.J) / S::k_l;

        let m_prime = (1.0 / S::c2) * (1.0 + S::c2 * cam02.M).ln();

        Jab {
            J: j_prime,
            a: m_prime * ((PI / 180.0) * cam02.h).cos(),
            b: m_prime * ((PI / 180.0) * cam02.h).sin(),
            space: PhantomData,
        }
    }
}

impl<T: Into<sRGB>, S: JabSpace> From<T> for Jab<S> {
    fn from(rgb: T) -> Jab<S> {
        Jab::<S>::from(&JCh::from(&LMS::from(&XYZ::from(&LinearRGB::from(
            &rgb.into(),
        )))))
    }
}

impl<S: JabSpace> From<[f32; 3]> for Jab<S> {
    fn from(jab: [f32; 3]) -> Jab<S> {
        Jab {
            J: jab[0],
            a: jab[1],
            b: jab[2],
            space: PhantomData,
        }
    }
}

impl<S: JabSpace> From<(f32, f32, f32)> for Jab<S> {
    fn from(jab: (f32, f32, f32)) -> Jab<S> {
        Jab {
            J: jab.0,
            a: jab.1,
            b: jab.2,
            space: PhantomData,
        }
    }
}

impl<S: JabSpace> Jab<S> {
    pub fn squared_difference(&self, other: &Jab<S>) -> f32 {
        let diff_j = (self.J - other.J).abs();
        let diff_a = (self.a - other.a).abs();
        let diff_b = (self.b - other.b).abs();

        (diff_j / S::k_l).powi(2) + diff_a.powi(2) + diff_b.powi(2)
    }
}

#[cfg(test)]
mod tests {
    use crate::{consts::UCS, JCh, Jab};

    macro_rules! float_eq {
        ($lhs:expr, $rhs:expr) => {
            assert_eq!(format!("{:.2}", $lhs), $rhs)
        };
    }

    // based on https://github.com/connorgr/d3-cam02/blob/master/test/cam02-test.js,
    #[test]
    fn jch_channels() {
        float_eq!(JCh::from([0, 0, 0]).J, "0.00");
        float_eq!(JCh::from([50, 50, 50]).J, "14.92");
        float_eq!(JCh::from([100, 100, 100]).J, "32.16");
        float_eq!(JCh::from([150, 150, 150]).J, "52.09");
        float_eq!(JCh::from([200, 200, 200]).J, "74.02");
        float_eq!(JCh::from([250, 250, 250]).J, "97.57");
        float_eq!(JCh::from([255, 255, 255]).J, "100.00");

        let red = JCh::from([255, 0, 0]);
        float_eq!(red.J, "46.93");
        float_eq!(red.C, "111.30");
        float_eq!(red.h, "32.15");
    }

    #[test]
    fn jab_channels() {
        float_eq!(Jab::<UCS>::from([0, 0, 0]).J, "0.00");
        float_eq!(Jab::<UCS>::from([50, 50, 50]).J, "22.96");
        float_eq!(Jab::<UCS>::from([150, 150, 150]).J, "64.89");
        let white = Jab::<UCS>::from([255, 255, 255]);
        float_eq!(white.J, "100.00");
        float_eq!(white.a, "-1.91");
        float_eq!(white.b, "-1.15");
        let red = Jab::<UCS>::from([255, 0, 0]);
        float_eq!(red.J, "60.05");
        float_eq!(red.a, "38.69");
        float_eq!(red.b, "24.32");
        let blue = Jab::<UCS>::from([0, 0, 255]);
        float_eq!(blue.J, "31.22");
        float_eq!(blue.a, "-8.38");
        float_eq!(blue.b, "-39.16");
    }
}
