#[cfg(feature = "write")]
use alloc::borrow::ToOwned;
use bin_utils::*;

use super::channel::*;

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
    pub fn fixed_channel_sequence(channel: Channel) -> Self {
        match channel {
            Channel::Simple { channel } => ChannelSequence::Simple([channel; 16]),
            Channel::Legacy { flags, channel } => ChannelSequence::Legacy([(flags, channel); 16]),
            Channel::OpClass { channel, opclass } => {
                ChannelSequence::OpClass([(channel, opclass); 16])
            }
        }
    }
}
#[cfg(feature = "read")]
impl ReadCtx<&ChannelEncoding> for ChannelSequence {
    fn from_bytes(
        data: &mut impl ExactSizeIterator<Item = u8>,
        ctx: &ChannelEncoding,
    ) -> Result<Self, ParserError> {
        let channel_sequence_bytes_length = 16 * ctx.size() as usize;
        let mut data = data.take(channel_sequence_bytes_length);
        if data.len() < channel_sequence_bytes_length {
            return Err(ParserError::TooLittleData(
                channel_sequence_bytes_length - data.len(),
            ));
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
            ChannelEncoding::Unknown(_) => return Err(ParserError::ValueNotUnderstood),
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
