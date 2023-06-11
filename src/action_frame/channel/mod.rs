

use crate::enum_to_int;
#[cfg(feature = "read")]
use crate::parser::{ParserError, ReadCtx};
#[cfg(feature = "write")]
use {
    crate::parser::Write,
    alloc::borrow::{ToOwned},
};

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
}
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChannelSequence {
    /// Channel
    Simple(ChannelSequenceInternal<u8>),
    /// Flags, Channel
    Legacy(ChannelSequenceInternal<(u8, u8)>),
    /// Channel, OpClass
    OpClass(ChannelSequenceInternal<(u8, u8)>),
}
impl ChannelSequence {
    pub fn channel_encoding(&self) -> ChannelEncoding {
        match self {
            Self::Simple(_) => ChannelEncoding::Simple,
            Self::Legacy(_) => ChannelEncoding::Legacy,
            Self::OpClass(_) => ChannelEncoding::OpClass,
        }
    }
}
#[cfg(feature = "read")]
impl ReadCtx<&ChannelEncoding> for ChannelSequence {
    fn from_bytes(
        data: &mut impl ExactSizeIterator<Item = u8>,
        ctx: &ChannelEncoding,
    ) -> Result<Self, crate::parser::ParserError> {
        let channel_sequence_bytes_length = 16 * ctx.size() as usize;
        let mut data = data.take(channel_sequence_bytes_length);
        if data.len() < channel_sequence_bytes_length {
            return Err(ParserError::TooLittleData(channel_sequence_bytes_length - data.len()));
        }
        Ok(match ctx {
            ChannelEncoding::Simple => Self::Simple(data.next_chunk().unwrap()),
            ChannelEncoding::Legacy => Self::Legacy(
                data.array_chunks::<2>()
                    .map(|x| (x[0], x[1]))
                    .next_chunk()
                    .unwrap(),
            ),
            ChannelEncoding::OpClass => Self::OpClass(
                data.array_chunks::<2>()
                    .map(|x| (x[0], x[1]))
                    .next_chunk()
                    .unwrap(),
            ),
            ChannelEncoding::Unknown(_) => return Err(ParserError::ValueNotUnderstood)
        })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for ChannelSequence {
    fn to_bytes(&self) -> alloc::borrow::Cow<'a, [u8]> {
        match self {
            ChannelSequence::Simple(chan_seq) => chan_seq.as_slice().to_owned().into(),
            ChannelSequence::Legacy(chan_seq) | ChannelSequence::OpClass(chan_seq) => {
                chan_seq.iter().copied().flat_map(|(x, y)| [x, y]).collect()
            }
        }
    }
}
pub fn fixed_channel_sequence(channel: Channel) -> ChannelSequence {
    match channel {
        Channel::Simple { channel } => ChannelSequence::Simple([channel; 16]),
        Channel::Legacy { flags, channel } => ChannelSequence::Legacy([(flags, channel); 16]),
        Channel::OpClass { channel, opclass } => ChannelSequence::OpClass([(channel, opclass); 16])
    }
}
