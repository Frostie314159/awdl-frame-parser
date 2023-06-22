#[cfg(feature = "write")]
use bin_utils::*;
use try_take::try_take;

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
        let mut data =
            try_take(data, channel_sequence_bytes_length).map_err(ParserError::TooLittleData)?;
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
            ChannelSequence::Simple(chan_seq) => chan_seq.to_vec().into(),
            ChannelSequence::Legacy(chan_seq) | ChannelSequence::OpClass(chan_seq) => {
                chan_seq.iter().copied().flat_map(|(x, y)| [x, y]).collect()
            }
        }
    }
}
