pub mod channel;
pub mod channel_sequence;

use core::num::NonZeroU8;

use channel::*;
use channel_sequence::*;

use bin_utils::*;
#[cfg(feature = "read")]
use try_take::try_take;

use crate::tlvs::{impl_tlv_conversion, TLVType};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub struct ChannelSequenceTLV {
    /// The channel encoding.
    pub channel_encoding: ChannelEncoding,

    /// The amount of AWs spent on one channel.
    pub step_count: NonZeroU8,

    /// The channels.
    pub channel_sequence: ChannelSequence,
}
#[cfg(feature = "read")]
impl Read for ChannelSequenceTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let mut header = try_take(data, 9).map_err(ParserError::TooLittleData)?;

        let _channel_count = header
            .next()
            .unwrap()
            .checked_add(1)
            .ok_or(ParserError::ValueNotUnderstood)?; // Don't ask.
        let channel_encoding = header.next().unwrap().into();
        let _duplicate_count = header.next().unwrap();
        let step_count = NonZeroU8::new(
            header
                .next()
                .unwrap()
                .checked_add(1)
                .ok_or(ParserError::ValueNotUnderstood)?,
        )
        .unwrap();
        let _fill_channels = u16::from_le_bytes(header.next_chunk().unwrap());

        let channel_sequence = ChannelSequence::from_bytes(data, &channel_encoding)?;

        Ok(Self {
            channel_encoding,
            step_count,
            channel_sequence,
        })
    }
}
#[cfg(feature = "write")]
impl Write for ChannelSequenceTLV {
    fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        let binding = [
            0x0f,
            self.channel_encoding.into(),
            0x00,
            self.step_count.get() - 1,
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
impl_tlv_conversion!(false, ChannelSequenceTLV, TLVType::ChannelSequence, 9);

#[cfg(test)]
#[test]
fn test_channel_sequence_tlv() {
    use crate::tlvs::AWDLTLV;

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
            step_count: NonZeroU8::new(4).unwrap(),
            channel_sequence: ChannelSequence::fixed_channel_sequence(Channel::OpClass {
                channel: 0x6,
                opclass: 0x51
            }),
        }
    );

    assert_eq!(channel_sequence_tlv.to_bytes(), &bytes[3..]);
}
