use macro_bits::{bit, bitfield, serializable_enum};

serializable_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// This enum contains the three different types of channel encodings.
    pub enum ChannelEncoding : u8 {
        /// Simple channel encoding.
        Simple => 0x00,

        /// Legacy channel encoding.
        Legacy => 0x01,

        /// Operating class channel encoding.
        OpClass => 0x03
    }
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

serializable_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub enum SupportChannel : u8 {
        Lower => 0x01,

        Upper => 0x02,

        #[default]
        Primary => 0x03
    }
}
serializable_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    /// These are the channel bandwiths supported by AWDL.
    pub enum ChannelBandwidth : u8{
        #[default]
        /// 20MHz
        TwentyMHz => 0x01,

        /// 40MHz
        FourtyMHz => 0x03
    }
}
serializable_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    /// This is the band on which the channel lies.
    /// This could potentially be expanded to the 6GHz spectrum as well.
    pub enum Band : u8 {
        #[default]
        /// 2.4GHz
        TwoPointFourGHz => 0x02,
        /// 5GHz
        FiveGHz => 0x01
    }
}
bitfield! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    /// The Flags for the legacy channel encoding.
    pub struct LegacyFlags : u8 {
        pub support_channel: SupportChannel => bit!(0, 1),
        pub channel_bandwidth: ChannelBandwidth => bit!(2, 3),
        pub band: Band => bit!(4, 5)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
                SupportChannel::Lower => *channel - 2,
                SupportChannel::Upper => *channel + 2,
                SupportChannel::Primary => *channel,
                _ => *channel,
            },
            Self::OpClass { channel, .. } => *channel,
        }
    }
}
