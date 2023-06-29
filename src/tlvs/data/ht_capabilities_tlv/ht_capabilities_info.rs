use bin_utils::*;

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
            flags |= (<RxSpatialStreams as Into<u8>>::into(value.rx_stbc) as u16) << 8;
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
