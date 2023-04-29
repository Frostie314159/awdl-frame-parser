use deku::prelude::*;

#[cfg(all(not(feature = "std"), feature = "write"))]
use alloc::vec::Vec;
#[cfg(all(not(feature = "std"), feature = "read"))]
use alloc::format;

#[cfg_attr(feature = "read", derive(DekuRead))]
#[cfg_attr(feature = "write", derive(DekuWrite))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
#[deku(type = "u8")]
pub enum ChannelEncoding {
    /// Simple channel encoding.
    #[deku(id = "0")]
    Simple,

    /// Legacy channel encoding.
    #[deku(id = "1")]
    Legacy,

    /// Operating class channel encoding.
    #[deku(id = "3")]
    OpClass,

    #[deku(id_pat = "_")]
    Unknown(u8),
}
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    Simple(u8),
    Legacy(u8, u8),
    OpClass(u8, u8),
}
#[cfg(feature = "read")]
impl<'a> DekuRead<'a, &ChannelEncoding> for Channel {
    fn read(
        input: &'a deku::bitvec::BitSlice<u8, deku::bitvec::Msb0>,
        ctx: &ChannelEncoding,
    ) -> Result<(&'a deku::bitvec::BitSlice<u8, deku::bitvec::Msb0>, Self), DekuError>
    where
        Self: Sized,
    {
        match ctx {
            ChannelEncoding::Simple => {
                let (rest, channel_number) = u8::read(input, ())?;
                Ok((rest, Self::Simple(channel_number)))
            }
            ChannelEncoding::Legacy => {
                let (rest, (flags, channel_number)) = <(u8, u8)>::read(input, ())?;
                Ok((rest, Self::Legacy(flags, channel_number)))
            }
            ChannelEncoding::OpClass => {
                let (rest, (channel_number, operating_class)) = <(u8, u8)>::read(input, ())?;
                Ok((rest, Self::Legacy(channel_number, operating_class)))
            }
            ChannelEncoding::Unknown(ce) => Err(DekuError::Parse(format!(
                "Cannot parse unknown channel encoding {ce}"
            ))),
        }
    }
}
#[cfg(feature = "write")]
impl DekuWrite for Channel {
    fn write(
        &self,
        output: &mut deku::bitvec::BitVec<u8, deku::bitvec::Msb0>,
        ctx: (),
    ) -> Result<(), DekuError> {
        match self {
            Channel::Simple(channel_number) => channel_number.write(output, ctx),
            Channel::Legacy(flags, channel_number) => {
                flags.write(output, ctx)?;
                channel_number.write(output, ctx)?;
                Ok(())
            }
            Channel::OpClass(channel_number, operating_class) => {
                channel_number.write(output, ctx)?;
                operating_class.write(output, ctx)?;
                Ok(())
            }
        }
    }
}
