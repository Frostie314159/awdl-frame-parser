use bin_utils::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// This enum contains the three different types of channel encodings.
pub enum ChannelEncoding {
    /// Simple channel encoding.
    Simple,

    /// Legacy channel encoding.
    Legacy,

    /// Operating class channel encoding.
    OpClass,

    Unknown(u8),
}
impl ChannelEncoding {
    #[inline]
    /// Returns the size in bytes of one channel with the encoding.
    pub const fn size(&self) -> u8 {
        match self {
            Self::Simple => 1,
            _ => 2,
        }
    }
}
enum_to_int! {
    u8,
    ChannelEncoding,

    0x00,
    ChannelEncoding::Simple,
    0x01,
    ChannelEncoding::Legacy,
    0x03,
    ChannelEncoding::OpClass
}

pub type ChannelSequenceInternal<T> = [T; 16];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SupportChannel {
    Lower,

    Upper,

    #[default]
    Primary,

    Unknown(u8),
}
enum_to_int! {
    u8,
    SupportChannel,

    0x01,
    SupportChannel::Lower,
    0x02,
    SupportChannel::Upper,
    0x03,
    SupportChannel::Primary
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// These are the channel bandwiths supported by AWDL.
pub enum ChannelBandwidth {
    #[default]
    /// 20MHz
    TwentyMHz,

    /// 40MHz
    FourtyMHz,

    Unknown(u8),
}
enum_to_int! {
    u8,
    ChannelBandwidth,

    0x01,
    ChannelBandwidth::TwentyMHz,
    0x03,
    ChannelBandwidth::FourtyMHz
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// This is the band on which the channel lies.
/// This could potentially be expanded to the 6GHz spectrum as well.
pub enum Band {
    #[default]
    /// 2.4GHz
    TwoPointFourGHz,
    /// 5GHz
    FiveGHz,

    Unknown(u8),
}
enum_to_int! {
    u8,
    Band,

    0x01,
    Band::FiveGHz,
    0x02,
    Band::TwoPointFourGHz
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// The Flags for the legacy channel encoding.
pub struct LegacyFlags {
    pub support_channel: SupportChannel,
    pub channel_bandwidth: ChannelBandwidth,
    pub band: Band,
}
impl From<u8> for LegacyFlags {
    fn from(value: u8) -> Self {
        Self {
            support_channel: (value & 3).into(),
            channel_bandwidth: ((value & 12) >> 2).into(),
            band: ((value & 48) >> 4).into(),
        }
    }
}
impl From<LegacyFlags> for u8 {
    fn from(value: LegacyFlags) -> Self {
        <SupportChannel as Into<u8>>::into(value.support_channel)
            | (<ChannelBandwidth as Into<u8>>::into(value.channel_bandwidth) << 2)
            | (<Band as Into<u8>>::into(value.band) << 4)
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
/// This enum contains a named channel.
pub enum Channel {
    Simple { channel: u8 },
    Legacy { flags: LegacyFlags, channel: u8 },
    OpClass { channel: u8, opclass: u8 },
}
impl Channel {
    #[inline]
    /// This returns the channel encoding of the channel.
    pub const fn channel_encoding(&self) -> ChannelEncoding {
        match self {
            Self::Simple { .. } => ChannelEncoding::Simple,
            Self::Legacy { .. } => ChannelEncoding::Legacy,
            Self::OpClass { .. } => ChannelEncoding::OpClass,
        }
    }
    #[inline]
    /// This returns the channel independent of the encoding.
    /// **NOTE**: for a [legacy](Channel::Legacy) this returns a corrected channel, as if the [support channel](SupportChannel) was set to primary.
    pub const fn channel(&self) -> u8 {
        match self {
            Self::Simple { channel } => *channel,
            Self::Legacy { flags, channel } => match flags.support_channel {
                SupportChannel::Lower => *channel + 2,
                SupportChannel::Upper => *channel - 2,
                SupportChannel::Primary => *channel,
                _ => *channel,
            },
            Self::OpClass { channel, .. } => *channel,
        }
    }
}
