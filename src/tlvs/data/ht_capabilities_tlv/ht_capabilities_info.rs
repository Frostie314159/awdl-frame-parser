use bin_utils::*;

#[cfg(feature = "read")]
use crate::common::check_bit;
#[cfg(feature = "write")]
use crate::common::set_bit;

use crate::common::bit;

pub const FLAG_HT_LDPC_CODING_CAPABILITY: u16 = bit!(0);
pub const FLAG_SUPPORT_CHANNEL_WIDTH: u16 = bit!(1);
pub const FLAG_SM_POWER_SAVE: u16 = bit!(2, 3);
pub const FLAG_GREEN_FIELD: u16 = bit!(4);
pub const FLAG_SHORT_GI_20MHZ: u16 = bit!(5);
pub const FLAG_SHORT_GI_40MHZ: u16 = bit!(6);
pub const FLAG_TX_STBC: u16 = bit!(7);
pub const FLAG_RX_STBC: u16 = bit!(8, 9);
pub const FLAG_DELAYED_BLOCK_ACK: u16 = bit!(10);
pub const FLAG_MAX_A_MSDU_LENGTH: u16 = bit!(11);
pub const FLAG_DSSS_40MHZ: u16 = bit!(12);
pub const FLAG_PSMP: u16 = bit!(13);
pub const FLAG_40MHZ_INTOLERANT: u16 = bit!(14);
pub const FLAG_TXOP_PROTECTION_SUPPORT: u16 = bit!(15);

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
            ldpc_coding_capability: check_bit!(value, FLAG_HT_LDPC_CODING_CAPABILITY),
            support_channel_width: check_bit!(value, FLAG_SUPPORT_CHANNEL_WIDTH),
            green_field: check_bit!(value, FLAG_GREEN_FIELD),
            short_gi_20mhz: check_bit!(value, FLAG_SHORT_GI_20MHZ),
            short_gi_40mhz: check_bit!(value, FLAG_SHORT_GI_40MHZ),
            tx_stbc: check_bit!(value, FLAG_TX_STBC),
            delayed_block_ack: check_bit!(value, FLAG_DELAYED_BLOCK_ACK),
            dsss_40mhz: check_bit!(value, FLAG_DSSS_40MHZ),
            psmp: check_bit!(value, FLAG_PSMP),
            forty_mhz_intolerant: check_bit!(value, FLAG_40MHZ_INTOLERANT),
            txop_protection_support: check_bit!(value, FLAG_TXOP_PROTECTION_SUPPORT),
            max_a_msdu_length: if check_bit!(value, FLAG_MAX_A_MSDU_LENGTH) {
                MAXAmsduLength::Large
            } else {
                MAXAmsduLength::Small
            },
            rx_stbc: (((value & FLAG_RX_STBC) >> 8) as u8).into(),
            sm_power_save: (((value & FLAG_SM_POWER_SAVE) >> 2) as u8).into(),
        }
    }
}
#[cfg(feature = "write")]
impl From<HTCapabilitiesInfo> for u16 {
    fn from(value: HTCapabilitiesInfo) -> u16 {
        let mut flags = 0u16;

        set_bit!(
            flags,
            FLAG_HT_LDPC_CODING_CAPABILITY,
            value.ldpc_coding_capability
        );
        set_bit!(
            flags,
            FLAG_SUPPORT_CHANNEL_WIDTH,
            value.support_channel_width
        );
        set_bit!(
            flags,
            (<SmPwSave as Into<u8>>::into(value.sm_power_save) << 2) as u16,
            value.sm_power_save != SmPwSave::Static
        );
        set_bit!(flags, FLAG_GREEN_FIELD, value.green_field);
        set_bit!(flags, FLAG_SHORT_GI_20MHZ, value.short_gi_20mhz);
        set_bit!(flags, FLAG_SHORT_GI_40MHZ, value.short_gi_40mhz);
        set_bit!(flags, FLAG_TX_STBC, value.tx_stbc);
        set_bit!(
            flags,
            (<RxSpatialStreams as Into<u8>>::into(value.rx_stbc) as u16) << 8,
            value.rx_stbc != RxSpatialStreams::Zero
        );
        set_bit!(flags, FLAG_DELAYED_BLOCK_ACK, value.delayed_block_ack);
        set_bit!(
            flags,
            FLAG_MAX_A_MSDU_LENGTH,
            value.max_a_msdu_length == MAXAmsduLength::Large
        );
        set_bit!(flags, FLAG_DSSS_40MHZ, value.dsss_40mhz);
        set_bit!(flags, FLAG_PSMP, value.psmp);
        set_bit!(flags, FLAG_40MHZ_INTOLERANT, value.forty_mhz_intolerant);
        set_bit!(
            flags,
            FLAG_TXOP_PROTECTION_SUPPORT,
            value.txop_protection_support
        );
        flags
    }
}
