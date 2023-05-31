use crate::enum_to_int;

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
    pub fn size(&self) -> u8 {
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

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Simple(u8),
    Legacy(u8, u8),
    OpClass(u8, u8),
}
impl From<Channel> for ChannelEncoding {
    fn from(value: Channel) -> Self {
        match value {
            Channel::Simple(_) => ChannelEncoding::Simple,
            Channel::Legacy(_, _) => ChannelEncoding::Legacy,
            Channel::OpClass(_, _) => ChannelEncoding::OpClass,
        }
    }
}
#[cfg(feature = "read")]
impl crate::parser::ReadCtx<&ChannelEncoding> for Channel {
    type Error = crate::parser::ParserError;

    fn from_bytes(
        data: &mut impl ExactSizeIterator<Item = u8>,
        ctx: &ChannelEncoding,
    ) -> Result<Self, Self::Error> {
        use crate::parser::ParserError;
        Ok(match ctx {
            ChannelEncoding::Simple => {
                Self::Simple(data.next().ok_or(ParserError::TooLittleData(1))?)
            }
            ChannelEncoding::Legacy => {
                if data.len() < 2 {
                    return Err(ParserError::TooLittleData(2 - data.len()));
                }
                Self::Legacy(data.next().unwrap(), data.next().unwrap())
            }
            ChannelEncoding::OpClass => {
                if data.len() < 2 {
                    return Err(ParserError::TooLittleData(2 - data.len()));
                }

                Self::OpClass(data.next().unwrap(), data.next().unwrap())
            }
            ChannelEncoding::Unknown(_) => return Err(ParserError::ValueNotUnderstood),
        })
    }
}
#[cfg(feature = "write")]
impl<'a> crate::parser::Write<'a> for Channel {
    fn to_bytes(&self) -> alloc::borrow::Cow<'a, [u8]> {
        use alloc::borrow::ToOwned;
        alloc::borrow::Cow::Owned(match self {
            Channel::Simple(channel) => [*channel].as_slice().to_owned(),
            Channel::Legacy(flags, channel) => [*flags, *channel].as_slice().to_owned(),
            Channel::OpClass(channel, op_class) => [*channel, *op_class].as_slice().to_owned(),
        })
    }
}

#[cfg(feature = "fixed_chan_seq")]
pub type ChannelSequence = [Channel; 16];
#[cfg(not(feature = "fixed_chan_seq"))]
pub type ChannelSequence = alloc::vec::Vec<Channel>;

#[cfg(feature = "read")]
impl crate::parser::ReadCtx<(&u8, &ChannelEncoding)> for ChannelSequence {
    type Error = crate::parser::ParserError;

    #[cfg(feature = "fixed_chan_seq")]
    fn from_bytes(
        data: &mut impl ExactSizeIterator<Item = u8>,
        ctx: (&u8, &ChannelEncoding),
    ) -> Result<Self, Self::Error> {
        use crate::parser::ParserError;

        fn parse_2byte_channel<F: Fn(u8, u8) -> Channel>(data: &mut impl ExactSizeIterator<Item = u8>, f: F) -> [Channel; 16] {
            let mut channels = [f(0x00, 0x00); 16];
            let bytes = data.next_chunk::<32>().unwrap();
            (0..16)
                .for_each(|i| channels[i] = f(bytes[2 * i], bytes[2 * i + 1]));
            channels
        }

        match ctx.1 {
            ChannelEncoding::Simple if data.len() < 16 => {
                return Err(ParserError::TooLittleData(16 - data.len()))
            }
            _ if data.len() < 32 => return Err(ParserError::TooLittleData(32 - data.len())),
            _ => {}
        };
        Ok(match ctx.1 {
            ChannelEncoding::Simple => data.next_chunk::<16>().unwrap().map(Channel::Simple),
            ChannelEncoding::Legacy => parse_2byte_channel(data, Channel::Legacy),
            ChannelEncoding::OpClass => parse_2byte_channel(data, Channel::OpClass),
            _ => return Err(ParserError::ValueNotUnderstood),
        })
    }
    #[cfg(not(feature = "fixed_chan_seq"))]
    fn from_bytes(
        data: &mut impl ExactSizeIterator<Item = u8>,
        ctx: (&u8, &ChannelEncoding),
    ) -> Result<Self, Self::Error> {
        use crate::parser::ParserError;

        let expected_length = ctx.0 * ctx.1.size();
        if data.len() < expected_length.into() {
            return Err(ParserError::TooLittleData(
                expected_length as usize - data.len(),
            ));
        }
        Ok(alloc::vec![Channel::Simple(0x00); *ctx.0 as usize]
            .iter()
            .map(|_| Channel::from_bytes(data, ctx.1).unwrap())
            .collect())
    }
}
#[cfg(feature = "write")]
impl<'a> crate::parser::Write<'a> for ChannelSequence {
    #[cfg(feature = "fixed_chan_seq")]
    fn to_bytes(&self) -> alloc::borrow::Cow<'a, [u8]> {
        use alloc::borrow::ToOwned;

        match self[0].into() {
            ChannelEncoding::Simple => self.map(|x| x.to_bytes()[0]).as_slice().to_owned().into(),
            _ => {
                let mut bytes = [0x00; 32];
                (0..16).for_each(|i| {
                    let channel_bytes = self[i].to_bytes();
                    bytes[2 * i] = channel_bytes[0];
                    bytes[2 * i + 1] = channel_bytes[1];
                });
                bytes.as_slice().to_owned().into()
            }
        }
    }
    #[cfg(not(feature = "fixed_chan_seq"))]
    fn to_bytes(&self) -> alloc::borrow::Cow<'a, [u8]> {
        use alloc::borrow::ToOwned;

        match self[0].into() {
            ChannelEncoding::Simple => self.map(|x| x.to_bytes()[0]).as_slice().to_owned().into(),
            _ => self
                .iter()
                .copied()
                .flat_map(|x| <&[u8] as TryInto<[u8; 2]>>::try_into(&x.to_bytes()).unwrap())
                .collect(),
        }
    }
}
pub fn fixed_channel_sequence(channel: Channel) -> ChannelSequence {
    #[cfg(feature = "fixed_chan_seq")]
    return [channel; 16];

    #[cfg(not(feature = "fixed_chan_seq"))]
    return alloc::vec![channel; 16];
}
