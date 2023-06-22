use bin_utils::*;
use try_take::try_take;

#[cfg(feature = "write")]
use alloc::borrow::Cow;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum SmPwSave {
    #[default]
    Static,
    Dynamic,
    Disabled,
    Unknown(u8),
}
enum_to_int! {
    u8,
    SmPwSave,

    0x00,
    SmPwSave::Static,
    0x01,
    SmPwSave::Dynamic,
    0x03,
    SmPwSave::Disabled
}
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum RxSpatialStreams {
    #[default]
    Zero,
    One,
    Two,
    Three,

    Unknown(u8),
}
enum_to_int! {
    u8,
    RxSpatialStreams,

    0x00,
    RxSpatialStreams::Zero,
    0x01,
    RxSpatialStreams::One,
    0x02,
    RxSpatialStreams::Two,
    0x03,
    RxSpatialStreams::Three
}
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum MAXAmsduLength {
    /// 3839 Bytes
    #[default]
    Small,
    /// 7935 Bytes
    Large,
}
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct HTCapabilitiesInfo {
    pub ldpc_coding_capability: bool,
    pub support_channel_width: bool,
    pub sm_power_save: SmPwSave,
    pub green_field: bool,
    pub short_gi_20mhz: bool,
    pub short_gi_40mhz: bool,
    pub tx_stbc: bool,
    pub rx_stbc: RxSpatialStreams,
    pub delayed_block_ack: bool,
    pub max_a_msdu_length: MAXAmsduLength,
    pub dsss_40mhz: bool,
    pub psmp: bool,
    pub forty_mhz_intolerant: bool,
    pub txop_protection_support: bool,
}
#[cfg(feature = "read")]
impl From<u16> for HTCapabilitiesInfo {
    fn from(value: u16) -> Self {
        Self {
            ldpc_coding_capability: value & 0x1 != 0x0,
            support_channel_width: value & 0x2 != 0x0,
            sm_power_save: (((value & 0b0000_0000_0000_1100) >> 2) as u8).into(),
            green_field: value & 0x10 != 0x0,
            short_gi_20mhz: value & 0x20 != 0x0,
            short_gi_40mhz: value & 0x40 != 0x0,
            tx_stbc: value & 0x80 != 0x0,
            rx_stbc: (((value & 0x300) >> 8) as u8).into(),
            delayed_block_ack: value & 0x400 != 0x0,
            max_a_msdu_length: if value & 0x800 != 0x0 {
                MAXAmsduLength::Large
            } else {
                MAXAmsduLength::Small
            },
            dsss_40mhz: value & 0x1000 != 0x0,
            psmp: value & 0x2000 != 0x0,
            forty_mhz_intolerant: value & 0x4000 != 0x0,
            txop_protection_support: value & 0x8000 != 0x0,
        }
    }
}
#[cfg(feature = "write")]
impl From<HTCapabilitiesInfo> for u16 {
    fn from(value: HTCapabilitiesInfo) -> u16 {
        let mut flags = 0u16;
        if value.ldpc_coding_capability {
            flags |= 0x1;
        }
        if value.support_channel_width {
            flags |= 0x2;
        }
        if value.sm_power_save != SmPwSave::Static {
            flags |= (<SmPwSave as Into<u8>>::into(value.sm_power_save) << 2) as u16;
        }
        if value.green_field {
            flags |= 0x10;
        }
        if value.short_gi_20mhz {
            flags |= 0x20;
        }
        if value.short_gi_40mhz {
            flags |= 0x40;
        }
        if value.tx_stbc {
            flags |= 0x80;
        }
        if value.rx_stbc != RxSpatialStreams::Zero {
            flags |= ((<RxSpatialStreams as Into<u8>>::into(value.rx_stbc) as u16) << 8) as u16;
        }
        if value.delayed_block_ack {
            flags |= 0x400;
        }
        if value.max_a_msdu_length == MAXAmsduLength::Large {
            flags |= 0x800;
        }
        if value.dsss_40mhz {
            flags |= 0x1000;
        }
        if value.psmp {
            flags |= 0x2000;
        }
        if value.forty_mhz_intolerant {
            flags |= 0x4000;
        }
        if value.txop_protection_support {
            flags |= 0x8000;
        }

        flags
    }
}

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
            max_a_mpdu_length: ((value & 0x3) as u8).into(),
            mpdu_density: (((value & 0x1C) >> 2) as u8).into(),
        }
    }
}
#[cfg(feature = "write")]
impl From<AMpduParameters> for u8 {
    fn from(value: AMpduParameters) -> u8 {
        let mut parameters = 0;

        parameters |= <MAXAMpduLength as Into<u8>>::into(value.max_a_mpdu_length);
        parameters |= <MpduDensity as Into<u8>>::into(value.mpdu_density) << 2;

        parameters
    }
}
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct HTCapabilitiesTLV {
    pub ht_capabilities_info: HTCapabilitiesInfo,
    pub a_mpdu_parameters: AMpduParameters,
    pub rx_spatial_stream_count: u8,
}
#[cfg(feature = "read")]
impl Read for HTCapabilitiesTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let _ = data.next_chunk::<2>();
        let mut fixed_data = try_take(data, 3).map_err(ParserError::TooLittleData)?;
        let ht_capabilities_info =
            HTCapabilitiesInfo::from(u16::from_le_bytes(fixed_data.next_chunk().unwrap()));
        let a_mpdu_parameters = AMpduParameters::from(fixed_data.next().unwrap());
        let rx_spatial_stream_count = data
            .len()
            .checked_sub(2)
            .ok_or(ParserError::TooLittleData(2))? as u8;

        Ok(Self {
            ht_capabilities_info,
            a_mpdu_parameters,
            rx_spatial_stream_count,
        })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for HTCapabilitiesTLV {
    fn to_bytes(&self) -> Cow<'a, [u8]> {
        let mut header = [0x00; 5];
        header[2..4].copy_from_slice(
            <HTCapabilitiesInfo as Into<u16>>::into(self.ht_capabilities_info)
                .to_le_bytes()
                .as_slice(),
        );
        header[4] = <AMpduParameters as Into<u8>>::into(self.a_mpdu_parameters);
        header
            .into_iter()
            .chain((0..self.rx_spatial_stream_count).map(|_| 0xff))
            .chain([0x00; 2].into_iter())
            .collect()
    }
}

#[cfg(test)]
#[test]
fn test_ht_capabilities() {
    use alloc::borrow::ToOwned;

    let bytes = include_bytes!("../../../test_bins/ht_capabilities_tlv.bin")[3..].to_vec();
    assert_eq!(
        HTCapabilitiesInfo::from(0x1u16 | 0xCu16),
        HTCapabilitiesInfo {
            ldpc_coding_capability: true,
            sm_power_save: SmPwSave::Disabled,
            ..Default::default()
        }
    );
    let ht_capabilities_tlv = HTCapabilitiesTLV::from_bytes(&mut bytes.iter().copied()).unwrap();
    assert_eq!(
        ht_capabilities_tlv,
        HTCapabilitiesTLV {
            ht_capabilities_info: HTCapabilitiesInfo {
                ldpc_coding_capability: true,
                support_channel_width: true,
                sm_power_save: SmPwSave::Disabled,
                short_gi_20mhz: true,
                short_gi_40mhz: true,
                rx_stbc: RxSpatialStreams::One,
                ..Default::default()
            },
            a_mpdu_parameters: AMpduParameters {
                max_a_mpdu_length: MAXAMpduLength::VeryLarge,
                mpdu_density: MpduDensity::Sixteen,
                ..Default::default()
            },
            rx_spatial_stream_count: 2
        }
    );
    assert_eq!(ht_capabilities_tlv.to_bytes(), bytes.as_slice().to_owned());
}
