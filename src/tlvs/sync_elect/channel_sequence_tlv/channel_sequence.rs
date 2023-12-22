use core::fmt::Debug;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Pread, Pwrite,
};

use super::channel::*;

#[derive(Clone, PartialEq, Eq)]
/// The different types of channel sequences.
pub enum ChannelSequence {
    /// This en'codes just the channel.
    Simple([u8; 16]),
    /// This encodes channel flags and the channel it self.
    Legacy([(LegacyFlags, u8); 16]),
    /// This encodes first the channel and then the channels opclass.
    OpClass([(u8, u8); 16]),
}
impl ChannelSequence {
    #[inline]
    /// Returns the channel encoding of the channel sequence.
    pub const fn channel_encoding(&self) -> ChannelEncoding {
        match self {
            Self::Simple(_) => ChannelEncoding::Simple,
            Self::Legacy(_) => ChannelEncoding::Legacy,
            Self::OpClass(_) => ChannelEncoding::OpClass,
        }
    }
    #[inline]
    /// Generates a repeating channel sequence with the argument.
    pub const fn fixed_channel_sequence(channel: Channel) -> Self {
        match channel {
            Channel::Simple { channel } => ChannelSequence::Simple([channel; 16]),
            Channel::Legacy { flags, channel } => ChannelSequence::Legacy([(flags, channel); 16]),
            Channel::OpClass { channel, opclass } => {
                ChannelSequence::OpClass([(channel, opclass); 16])
            }
        }
    }
}
impl Default for ChannelSequence {
    fn default() -> Self {
        ChannelSequence::Simple(Default::default())
    }
}
impl Debug for ChannelSequence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ChannelSequence::Simple(channels) => f.debug_list().entries(channels.iter()).finish(),
            ChannelSequence::Legacy(channels) => f
                .debug_list()
                .entries(channels.iter().map(|(_, channel)| channel))
                .finish(),
            ChannelSequence::OpClass(channels) => f
                .debug_list()
                .entries(channels.iter().map(|(channel, _)| channel))
                .finish(),
        }
    }
}
impl MeasureWith<()> for ChannelSequence {
    fn measure_with(&self, _ctx: &()) -> usize {
        16 * match self {
            ChannelSequence::Legacy(_) | ChannelSequence::OpClass(_) => 2,
            _ => 1,
        }
    }
}
impl<'a> TryFromCtx<'a, ChannelEncoding> for ChannelSequence {
    type Error = scroll::Error;
    fn try_from_ctx(
        from: &'a [u8],
        encoding: ChannelEncoding,
    ) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        Ok((
            match encoding {
                ChannelEncoding::Simple => {
                    ChannelSequence::Simple(from.gread::<[u8; 16]>(&mut offset)?)
                }
                ChannelEncoding::Legacy => ChannelSequence::Legacy({
                    let mut array = [(LegacyFlags::default(), 0); 16];
                    for (i, bytes) in from
                        .gread::<[u8; 32]>(&mut offset)?
                        .as_chunks::<2>()
                        .0
                        .iter()
                        .enumerate()
                    {
                        array[i] = (LegacyFlags::from_representation(bytes[0]), bytes[1]);
                    }
                    array
                }),
                ChannelEncoding::OpClass => ChannelSequence::OpClass({
                    let mut array = [(0, 0); 16];
                    for (i, bytes) in from
                        .gread::<[u8; 32]>(&mut offset)?
                        .as_chunks::<2>()
                        .0
                        .iter()
                        .enumerate()
                    {
                        array[i] = (bytes[0], bytes[1]);
                    }
                    array
                }),
                ChannelEncoding::Unknown(_) => {
                    return Err(scroll::Error::BadInput {
                        size: offset,
                        msg: "Unknown encoding.",
                    })
                }
            },
            offset,
        ))
    }
}
impl TryIntoCtx for ChannelSequence {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        match self {
            ChannelSequence::Simple(channels) => buf.pwrite::<&[u8]>(channels.as_ref(), 0),
            ChannelSequence::Legacy(channels) => {
                let mut offset = 0;
                for (flags, channel) in channels.iter() {
                    buf.gwrite(flags.to_representation(), &mut offset)?;
                    buf.gwrite(channel, &mut offset)?;
                }
                Ok(offset)
            }
            ChannelSequence::OpClass(channels) => {
                let mut offset = 0;
                for (channel, opclass) in channels.iter() {
                    buf.gwrite(channel, &mut offset)?;
                    buf.gwrite(opclass, &mut offset)?;
                }
                Ok(offset)
            }
        }
    }
}
