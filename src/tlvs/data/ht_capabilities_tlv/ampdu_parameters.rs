use bin_utils::*;

#[cfg(feature = "read")]
use crate::common::bit;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MAXAMpduLength {
    /// 8kb
    #[default]
    Small,
    /// 16kb
    Medium,
    /// 32kb
    Large,
    /// 64kb
    VeryLarge,

    Unknown(u8),
}
enum_to_int! {
    u8,
    MAXAMpduLength,

    0x0,
    MAXAMpduLength::Small,
    0x1,
    MAXAMpduLength::Medium,
    0x2,
    MAXAMpduLength::Large,
    0x3,
    MAXAMpduLength::VeryLarge
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MpduDensity {
    #[default]
    NoRestriction,
    Quarter,
    Half,
    One,
    Two,
    Four,
    Eight,
    Sixteen,

    Unknown(u8),
}
enum_to_int! {
    u8,
    MpduDensity,

    0x0,
    MpduDensity::NoRestriction,
    0x1,
    MpduDensity::Quarter,
    0x2,
    MpduDensity::Half,
    0x3,
    MpduDensity::One,
    0x4,
    MpduDensity::Two,
    0x5,
    MpduDensity::Four,
    0x6,
    MpduDensity::Eight,
    0x7,
    MpduDensity::Sixteen
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct AMpduParameters {
    pub max_a_mpdu_length: MAXAMpduLength,
    pub mpdu_density: MpduDensity,
}
#[cfg(feature = "read")]
impl From<u8> for AMpduParameters {
    fn from(value: u8) -> Self {
        Self {
            max_a_mpdu_length: (value & (bit!(0, 1))).into(),
            mpdu_density: ((value & bit!(2, 3, 4)) >> 2).into(),
        }
    }
}
#[cfg(feature = "write")]
impl From<AMpduParameters> for u8 {
    fn from(value: AMpduParameters) -> u8 {
        <MAXAMpduLength as Into<u8>>::into(value.max_a_mpdu_length)
            | <MpduDensity as Into<u8>>::into(value.mpdu_density) << 2
    }
}
