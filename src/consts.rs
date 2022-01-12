#![allow(non_upper_case_globals)]
//! Constants for CAM02 and other CIE spaces.
use crate::{JabSpace, LMS, XYZ};

const unsafe fn float_from_bits(v: u32) -> f32 {
    std::mem::transmute::<u32, f32>(u32::from_be(v))
}

/// The standard D65 CIEXYZ illuminant.
pub const D65_XYZ: XYZ = XYZ {
    x: 95.047,
    y: 100.0,
    z: 108.883,
};

/// Transformation of the D65 CIEXYZ illuminant into CAM02 LMS
//     pub static ref D65_LMS: LMS = LMS::from(&D65_XYZ);
pub const D65_LMS: LMS = unsafe {
    LMS {
        l: float_from_bits(232504642),
        m: float_from_bits(118804290),
        s: float_from_bits(4268808514),
    }
};

/// CIECAM02 viewing conditions
pub mod VC {
    use super::{float_from_bits, D65_XYZ};
    use std::f32::consts::PI;

    pub const la: f32 = (64.0 / PI) / 5.0;
    pub const yb: f32 = 20.0;
    pub const f: f32 = 1.0;
    pub const c: f32 = 0.69;
    pub const nc: f32 = 1.0;
    pub const n: f32 = yb / D65_XYZ.y;
    pub const k: f32 = 1.0 / ((5.0 * la) + 1.0);

    /// 1.48 + n.sqrt();
    pub const z: f32 = unsafe { float_from_bits(4037998143) };
    /// (0.2 * k.powi(4) * (5.0 * la))
    pub const fl: f32 = unsafe { float_from_bits(3319237438) };
    /// 0.725 * (1.0 / n).powf(0.2);
    pub const nbb: f32 = unsafe { float_from_bits(4127817791) };
    /// 0.725 * (1.0 / n).powf(0.2);
    pub const ncb: f32 = unsafe { float_from_bits(4127817791) };
    /// f * (1.0 - (1.0 / 3.6) * ((-la - 42.0) / 92.0).exp());
    pub const d: f32 = unsafe { float_from_bits(1592218687) };
    pub const achromatic_response_to_white: f32 = unsafe { float_from_bits(3189820481) };

    // pub static ref z: f32 = 1.48 + n.sqrt();
    // pub static ref fl: f32 = (0.2 * k.powi(4) * (5.0 * la))
    //     + 0.1 * ((1.0 - k.powi(4)).powi(2)) * (5.0 * la).powf(1.0 / 3.0);
    // pub static ref nbb: f32 = 0.725 * (1.0 / n).powf(0.2);
    // pub static ref ncb: f32 = *nbb;
    // pub static ref d: f32 = f * (1.0 - (1.0 / 3.6) * ((-la - 42.0) / 92.0).exp());
    // pub static ref achromatic_response_to_white: f32 = {
    //     let lc = D65_LMS.l * (((D65_XYZ.y * *d) / D65_LMS.l) + (1.0 - *d));
    //     let mc = D65_LMS.m * (((D65_XYZ.y * *d) / D65_LMS.m) + (1.0 - *d));
    //     let sc = D65_LMS.s * (((D65_XYZ.y * *d) / D65_LMS.s) + (1.0 - *d));

    //     let hpe = HPE::from(&LMS {
    //         l: lc,
    //         m: mc,
    //         s: sc,
    //     });
    //     let lpa = nonlinear_adaptation(hpe.lh, *fl);
    //     let mpa = nonlinear_adaptation(hpe.mh, *fl);
    //     let spa = nonlinear_adaptation(hpe.sh, *fl);

    //     (2.0 * lpa + mpa + 0.05 * spa - 0.305) * *nbb
    // };
}

/// Jab transformation coefficients optimized for Large Color Differences.
pub struct LCD;
impl JabSpace for LCD {
    const k_l: f32 = 0.77;
    const c1: f32 = 0.007;
    const c2: f32 = 0.0053;
}

/// Jab transformation coefficients optimized for Short Color Differences.
pub struct SCD;
impl JabSpace for SCD {
    const k_l: f32 = 1.24;
    const c1: f32 = 0.007;
    const c2: f32 = 0.0363;
}

/// Jab transformations to create an approximately perceptually uniform color space.
pub struct UCS;
impl JabSpace for UCS {
    const k_l: f32 = 1.0;
    const c1: f32 = 0.007;
    const c2: f32 = 0.0228;
}

/// pre-generated lookup table for sRGB -> linear rgb conversion.
pub const sRGB_LOOKUP: [f32; 256] = unsafe {
    [
        float_from_bits(0),
        float_from_bits(3022167865),
        float_from_bits(3022135098),
        float_from_bits(246705722),
        float_from_bits(3022167866),
        float_from_bits(1642841658),
        float_from_bits(246738490),
        float_from_bits(1564347195),
        float_from_bits(3022135099),
        float_from_bits(185021243),
        float_from_bits(1642808891),
        float_from_bits(2370919227),
        float_from_bits(2381410363),
        float_from_bits(3336667963),
        float_from_bits(384208699),
        float_from_bits(4253523003),
        float_from_bits(3083446587),
        float_from_bits(1873655611),
        float_from_bits(1228260923),
        float_from_bits(1633932603),
        float_from_bits(3241796923),
        float_from_bits(1891300667),
        float_from_bits(3044213564),
        float_from_bits(1382091836),
        float_from_bits(61281596),
        float_from_bits(3192201020),
        float_from_bits(1799235900),
        float_from_bits(4153488188),
        float_from_bits(1229012540),
        float_from_bits(1129138492),
        float_from_bits(3350615100),
        float_from_bits(2977849404),
        float_from_bits(3752160316),
        float_from_bits(573405500),
        float_from_bits(2819261244),
        float_from_bits(2679081276),
        float_from_bits(3682963516),
        float_from_bits(3314390844),
        float_from_bits(3267862076),
        float_from_bits(859088444),
        float_from_bits(2111024444),
        float_from_bits(27702588),
        float_from_bits(548191548),
        float_from_bits(971425084),
        float_from_bits(2874461756),
        float_from_bits(3589789244),
        float_from_bits(282124092),
        float_from_bits(3118327868),
        float_from_bits(740422204),
        float_from_bits(3248159548),
        float_from_bits(1772421693),
        float_from_bits(3700950845),
        float_from_bits(3869707325),
        float_from_bits(2949779773),
        float_from_bits(1662588733),
        float_from_bits(779885629),
        float_from_bits(1023091005),
        float_from_bits(2996184893),
        float_from_bits(3142462781),
        float_from_bits(2149790525),
        float_from_bits(722811197),
        float_from_bits(3810737981),
        float_from_bits(3511633213),
        float_from_bits(479808317),
        float_from_bits(4026487101),
        float_from_bits(1787058237),
        float_from_bits(3072745277),
        float_from_bits(4276512061),
        float_from_bits(1690659901),
        float_from_bits(264401725),
        float_from_bits(702380605),
        float_from_bits(1728807229),
        float_from_bits(2511832125),
        float_from_bits(812877885),
        float_from_bits(1245613117),
        float_from_bits(4128804925),
        float_from_bits(1157862461),
        float_from_bits(1258133309),
        float_from_bits(369597501),
        float_from_bits(3139477565),
        float_from_bits(1263182909),
        float_from_bits(3615729725),
        float_from_bits(1892527165),
        float_from_bits(673689917),
        float_from_bits(311539005),
        float_from_bits(990624317),
        float_from_bits(3046489661),
        float_from_bits(2452669245),
        float_from_bits(3806054205),
        float_from_bits(3063401533),
        float_from_bits(526700861),
        float_from_bits(742512189),
        float_from_bits(4012825405),
        float_from_bits(1999495229),
        float_from_bits(3577537853),
        float_from_bits(425585213),
        float_from_bits(1385099069),
        float_from_bits(2446390333),
        float_from_bits(3894671933),
        float_from_bits(1636368189),
        float_from_bits(125764158),
        float_from_bits(2151810366),
        float_from_bits(2735605822),
        float_from_bits(2028145214),
        float_from_bits(96537918),
        float_from_bits(1420234814),
        float_from_bits(1804997438),
        float_from_bits(1385043518),
        float_from_bits(277813566),
        float_from_bits(2912427070),
        float_from_bits(816521022),
        float_from_bits(2747671102),
        float_from_bits(166405438),
        float_from_bits(1813522750),
        float_from_bits(3494784062),
        float_from_bits(1099837246),
        float_from_bits(3302371902),
        float_from_bits(1630025278),
        float_from_bits(511916350),
        float_from_bits(65485886),
        float_from_bits(374620222),
        float_from_bits(1607091006),
        float_from_bits(3830007614),
        float_from_bits(2933017150),
        float_from_bits(3261353534),
        float_from_bits(654332478),
        float_from_bits(3869529406),
        float_from_bits(55793982),
        float_from_bits(2332712254),
        float_from_bits(2093703230),
        float_from_bits(3767886910),
        float_from_bits(3211356222),
        float_from_bits(541551678),
        float_from_bits(204369982),
        float_from_bits(2216588350),
        float_from_bits(2417522750),
        float_from_bits(941390910),
        float_from_bits(3256123454),
        float_from_bits(1008763454),
        float_from_bits(2368767038),
        float_from_bits(3125118526),
        float_from_bits(3311372350),
        float_from_bits(2994637374),
        float_from_bits(2208468030),
        float_from_bits(1036750398),
        float_from_bits(3791163454),
        float_from_bits(1949012798),
        float_from_bits(4167210302),
        float_from_bits(1906284350),
        float_from_bits(3806370110),
        float_from_bits(1311218750),
        float_from_bits(3077742142),
        float_from_bits(583245886),
        float_from_bits(2451088190),
        float_from_bits(141796670),
        float_from_bits(2312283966),
        float_from_bits(406301246),
        float_from_bits(3080760382),
        float_from_bits(1762635582),
        float_from_bits(864268606),
        float_from_bits(419214142),
        float_from_bits(410695230),
        float_from_bits(939374654),
        float_from_bits(2038807358),
        float_from_bits(3776101950),
        float_from_bits(1906688062),
        float_from_bits(759022398),
        float_from_bits(400213310),
        float_from_bits(863815742),
        float_from_bits(2200161086),
        float_from_bits(198233406),
        float_from_bits(3447836734),
        float_from_bits(3426276158),
        float_from_bits(200660286),
        float_from_bits(2377570366),
        float_from_bits(1417534270),
        float_from_bits(1699339838),
        float_from_bits(3239764286),
        float_from_bits(1811014462),
        float_from_bits(1741547070),
        float_from_bits(3064916286),
        float_from_bits(1570106430),
        float_from_bits(1568796478),
        float_from_bits(3128095294),
        float_from_bits(1986655550),
        float_from_bits(2489710654),
        float_from_bits(392690494),
        float_from_bits(40828478),
        float_from_bits(767361087),
        float_from_bits(239469119),
        float_from_bits(2815492927),
        float_from_bits(4183754047),
        float_from_bits(116459071),
        float_from_bits(3481536575),
        float_from_bits(1444612671),
        float_from_bits(2612267839),
        float_from_bits(2689600831),
        float_from_bits(1743719999),
        float_from_bits(4052750399),
        float_from_bits(1043665471),
        float_from_bits(1373377343),
        float_from_bits(730207551),
        float_from_bits(3442612031),
        float_from_bits(971118655),
        float_from_bits(1871977023),
        float_from_bits(1900616767),
        float_from_bits(1090592319),
        float_from_bits(3753582399),
        float_from_bits(1316561215),
        float_from_bits(2386109247),
        float_from_bits(2700878911),
        float_from_bits(2260870719),
        float_from_bits(1099638847),
        float_from_bits(3545639487),
        float_from_bits(1025846335),
        float_from_bits(2163617087),
        float_from_bits(2664050495),
        float_from_bits(2543923519),
        float_from_bits(1820013375),
        float_from_bits(542651711),
        float_from_bits(3006740287),
        float_from_bits(639252799),
        float_from_bits(2097101375),
        float_from_bits(3051830335),
        float_from_bits(3536993855),
        float_from_bits(3569369151),
        float_from_bits(3199287871),
        float_from_bits(2426750015),
        float_from_bits(1268532799),
        float_from_bits(4053092415),
        float_from_bits(2190625343),
        float_from_bits(26364991),
        float_from_bits(1855213119),
        float_from_bits(3415822399),
        float_from_bits(413291071),
        float_from_bits(1470977087),
        float_from_bits(2327533119),
        float_from_bits(2966181951),
        float_from_bits(3454032447),
        float_from_bits(3774307391),
        float_from_bits(3960561471),
        float_from_bits(3996016959),
        float_from_bits(4014891839),
        float_from_bits(3916522815),
        float_from_bits(3818350399),
        float_from_bits(3602934079),
        float_from_bits(3454823231),
        float_from_bits(3206245951),
        float_from_bits(3092082751),
        float_from_bits(2944561727),
        float_from_bits(2931455039),
        float_from_bits(2935322175),
        float_from_bits(3107158335),
        float_from_bits(3329522495),
        float_from_bits(3770187071),
        float_from_bits(32831),
    ]
};
