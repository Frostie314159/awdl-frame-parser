pub mod channel;
pub mod channel_sequence;

pub use channel::*;
pub use channel_sequence::*;

use bin_utils::*;

use crate::tlvs::{TLVType, AWDLTLV};

#[cfg(feature = "read")]
use crate::tlvs::FromTLVError;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub struct ChannelSequenceTLV {
    /// The channel encoding.
    pub channel_encoding: ChannelEncoding,

    /// The amount of AWs spent on one channel.
    pub step_count: u8,

    /// The channels.
    pub channel_sequence: ChannelSequence,
}
#[cfg(feature = "read")]
impl Read for ChannelSequenceTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        if data.len() < 9 {
            return Err(ParserError::TooLittleData(9 - data.len()));
        }

        let _channel_count = data.next().unwrap() + 1; // Don't ask.
        let channel_encoding = data.next().unwrap().into();
        let _duplicate_count = data.next().unwrap();
        let step_count = data.next().unwrap() + 1;
        let _fill_channels = u16::from_le_bytes(data.next_chunk().unwrap());

        let channel_sequence = ChannelSequence::from_bytes(data, &channel_encoding).unwrap();
        let _ = data.next_chunk::<3>(); // Discard padding.
        Ok(Self {
            channel_encoding,
            step_count,
            channel_sequence,
        })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for ChannelSequenceTLV {
    fn to_bytes(&self) -> alloc::borrow::Cow<'a, [u8]> {
        let binding = [
            0x0f,
            self.channel_encoding.into(),
            0x00,
            self.step_count - 1,
            0xff,
            0xff,
        ];
        let header = binding.iter();
        let binding = self.channel_sequence.to_bytes();
        let channel_sequence = binding.iter();
        let padding = [0; 3].iter();
        header
            .chain(channel_sequence.chain(padding))
            .copied()
            .collect()
    }
}
#[cfg(feature = "write")]
impl From<ChannelSequenceTLV> for AWDLTLV<'_> {
    fn from(value: ChannelSequenceTLV) -> Self {
        Self {
            tlv_type: TLVType::ChannelSequence,
            tlv_data: value.to_bytes(),
        }
    }
}
#[cfg(feature = "read")]
impl TryFrom<AWDLTLV<'_>> for ChannelSequenceTLV {
    type Error = FromTLVError;
    fn try_from(value: AWDLTLV) -> Result<Self, Self::Error> {
        if value.tlv_data.len() < 9 {
            return Err(FromTLVError::IncorrectTlvLength);
        }
        if value.tlv_type != TLVType::ChannelSequence {
            return Err(FromTLVError::IncorrectTlvType);
        }
        Self::from_bytes(&mut value.tlv_data.iter().copied()).map_err(FromTLVError::ParserError)
    }
}
#[cfg(test)]
#[test]
fn test_channel_sequence_tlv() {
    let bytes = include_bytes!("../../../../test_bins/channel_sequence_tlv.bin");

    let tlv = AWDLTLV::from_bytes(&mut bytes.iter().copied()).unwrap();

    let channel_sequence_tlv = ChannelSequenceTLV::try_from(tlv.clone()).unwrap();
    assert_eq!(
        tlv,
        <ChannelSequenceTLV as Into<AWDLTLV>>::into(channel_sequence_tlv.clone())
    );

    assert_eq!(
        channel_sequence_tlv,
        ChannelSequenceTLV {
            channel_encoding: ChannelEncoding::OpClass,
            step_count: 4,
            channel_sequence: ChannelSequence::fixed_channel_sequence(Channel::OpClass {
                channel: 0x6,
                opclass: 0x51
            }),
        }
    );

    assert_eq!(channel_sequence_tlv.to_bytes(), &bytes[3..]);
}
