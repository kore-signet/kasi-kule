use crate::{
    consts::{self, VC},
    LinearRGB, LMS, XYZ,
};

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

#[target_feature(enable = "sse")]
pub unsafe fn sse_transform_cones(cones: [f32; 4]) -> [f32; 4] {
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

// slower than scalar
// #[target_feature(enable = "sse,sse2")]
// pub unsafe fn sse2_nonlinear_adaptation(cones: [f32; 4], fl: f32) -> [f32; 4] {
//     #[cfg(target_arch = "x86")]
//     use std::arch::x86::*;
//     #[cfg(target_arch = "x86_64")]
//     use std::arch::x86_64::*;

//     use crate::simd_math::*;

//     let cones_vec = _mm_load_ps(cones[..].as_ptr());

//     let p = if _mm_movemask_ps(_mm_cmpgt_ps(cones_vec, _mm_setzero_ps())) > 0 {
//         let fl = _mm_set1_ps(fl);
//         powf128_ps(
//             _mm_div_ps(_mm_mul_ps(fl, cones_vec), _mm_set1_ps(100.0)),
//             _mm_set1_ps(0.42),
//         )
//     } else {
//         cones_vec
//     };

//     let res = _mm_add_ps(
//         _mm_div_ps(
//             _mm_mul_ps(p, _mm_set1_ps(400.0)),
//             _mm_add_ps(p, _mm_set1_ps(27.13)),
//         ),
//         _mm_set1_ps(0.1),
//     );

//     std::mem::transmute::<__m128, [f32; 4]>(res)
// }

#[cfg(test)]
mod test {
    use super::*;
    use crate::{LinearRGB, HPE, LMS, XYZ};

    #[test]
    fn test_sse_xyz() {
        let color = [fastrand::u8(..), fastrand::u8(..), fastrand::u8(..)];
        let lrgb = LinearRGB::from(color);
        let scalar = XYZ::from(color);
        let sse_result = unsafe { sse_xyz(&lrgb) };

        assert_eq!(scalar.x, sse_result[0], "X channel does not match");
        assert_eq!(scalar.y, sse_result[1], "Y channel does not match");
        assert_eq!(scalar.z, sse_result[2], "Z channel does not match");
    }

    #[test]
    fn test_sse_lms() {
        let color = [fastrand::u8(..), fastrand::u8(..), fastrand::u8(..)];
        let xyz = XYZ::from(color);
        let scalar = LMS::from(&xyz);

        let sse_result = unsafe { sse_lms(&xyz) };

        assert_eq!(scalar.l, sse_result[0], "L channel does not match");
        assert_eq!(scalar.m, sse_result[1], "M channel does not match");
        assert_eq!(scalar.s, sse_result[2], "S channel does not match");
    }

    #[test]
    fn test_sse_hpe() {
        let color = [fastrand::u8(..), fastrand::u8(..), fastrand::u8(..)];
        let lms = LMS::from(color);
        let scalar = HPE::from(&lms);

        let sse_result = unsafe { sse_hpe(&lms) };

        assert_eq!(scalar.lh, sse_result[0], "Lh channel does not match");
        assert_eq!(scalar.mh, sse_result[1], "Mh channel does not match");
        assert_eq!(scalar.sh, sse_result[2], "Sh channel does not match");
    }

    #[test]
    fn test_sse_transform_cones() {
        let color = [fastrand::u8(..), fastrand::u8(..), fastrand::u8(..)];
        let lms = LMS::from(color);
        let scalar = crate::utils::transform_cones([lms.l, lms.m, lms.s, 0.0]);
        let sse_result = unsafe { sse_transform_cones([lms.l, lms.m, lms.s, 0.0]) };

        assert_eq!(scalar[0], sse_result[0], "L channel does not match");
        assert_eq!(scalar[1], sse_result[1], "M channel does not match");
        assert_eq!(scalar[2], sse_result[2], "S channel does not match");
    }

    // #[test]
    // fn test_sse2_nonlinear_adaptation() {
    //     use crate::consts::VC;
    //     use crate::utils::nonlinear_adaptation_scalar;
    //     use approx::assert_relative_eq;

    //     let color = [fastrand::u8(..), fastrand::u8(..), fastrand::u8(..)];
    //     let hpe_transforms = HPE::from(color);

    //     let (lpa, mpa, spa) = (
    //         nonlinear_adaptation_scalar(hpe_transforms.lh, VC::fl),
    //         nonlinear_adaptation_scalar(hpe_transforms.mh, VC::fl),
    //         nonlinear_adaptation_scalar(hpe_transforms.sh, VC::fl),
    //     );

    //     let sse_result = unsafe {
    //         sse2_nonlinear_adaptation(
    //             [hpe_transforms.lh, hpe_transforms.mh, hpe_transforms.sh, 0.0],
    //             VC::fl,
    //         )
    //     };

    //     assert_relative_eq!(lpa, sse_result[0], max_relative = 0.001);
    //     assert_relative_eq!(mpa, sse_result[1], max_relative = 0.001);
    //     assert_relative_eq!(spa, sse_result[2], max_relative = 0.001);
    // }
}
