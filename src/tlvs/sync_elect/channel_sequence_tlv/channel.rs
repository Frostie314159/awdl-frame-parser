use bin_utils::*;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
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

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Simple { channel: u8 },
    Legacy { flags: u8, channel: u8 },
    OpClass { channel: u8, opclass: u8 },
}
impl Channel {
    pub fn channel_encoding(&self) -> ChannelEncoding {
        match self {
            Self::Simple { .. } => ChannelEncoding::Simple,
            Self::Legacy { .. } => ChannelEncoding::Legacy,
            Self::OpClass { .. } => ChannelEncoding::OpClass,
        }
    }
    pub fn channel(&self) -> u8 {
        match self {
            Self::Simple { channel } => *channel,
            Self::Legacy { channel, .. } => *channel,
            Self::OpClass { channel, .. } => *channel,
        }
    }
}
