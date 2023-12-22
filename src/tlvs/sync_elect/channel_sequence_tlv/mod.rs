pub mod channel;
pub mod channel_sequence;

use core::num::NonZeroU8;

use channel::*;
use channel_sequence::*;

use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Pread, Pwrite,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelSequenceTLV {
    /// The amount of AWs spent on one channel.
    pub step_count: NonZeroU8,

    /// The channels.
    pub channel_sequence: ChannelSequence,
}
impl Default for ChannelSequenceTLV {
    fn default() -> Self {
        ChannelSequenceTLV {
            step_count: NonZeroU8::new(3).unwrap(),
            channel_sequence: Default::default(),
        }
    }
}
impl MeasureWith<()> for ChannelSequenceTLV {
    fn measure_with(&self, ctx: &()) -> usize {
        9 + self.channel_sequence.measure_with(ctx)
    }
}
impl<'a> TryFromCtx<'a> for ChannelSequenceTLV {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;

        let channel_count = from.gread::<u8>(&mut offset)? + 1;
        if channel_count != 16 {
            return Err(scroll::Error::BadInput {
                size: offset,
                msg: "Channel sequence length wasn't 16.",
            });
        }
        let channel_encoding = ChannelEncoding::from_representation(from.gread(&mut offset)?);
        offset += 1; // Skip duplicate count
        let step_count = NonZeroU8::new(from.gread::<u8>(&mut offset)?.checked_add(1).ok_or(
            scroll::Error::BadInput {
                size: offset,
                msg: "step_count caused overflow",
            },
        )?)
        .unwrap();
        offset += 2;
        let channel_sequence = from.gread_with(&mut offset, channel_encoding)?;

        Ok((
            Self {
                step_count,
                channel_sequence,
            },
            offset,
        ))
    }
}
impl TryIntoCtx for ChannelSequenceTLV {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;

        buf.gwrite(16u8 - 1, &mut offset)?;
        buf.gwrite(
            self.channel_sequence.channel_encoding().to_representation(),
            &mut offset,
        )?;
        offset += 1;
        buf.gwrite(self.step_count.get() - 1, &mut offset)?;
        buf.gwrite(0xffffu16, &mut offset)?;
        buf.gwrite(self.channel_sequence, &mut offset)?;
        offset += 3;
        Ok(offset)
    }
}
#[cfg(test)]
#[test]
fn test_channel_sequence_tlv() {
    use alloc::vec;

    let bytes = &include_bytes!("../../../../test_bins/channel_sequence_tlv.bin")[3..];

    let channel_sequence_tlv = bytes.pread::<ChannelSequenceTLV>(0).unwrap();
    assert_eq!(
        channel_sequence_tlv,
        ChannelSequenceTLV {
            step_count: NonZeroU8::new(4).unwrap(),
            channel_sequence: ChannelSequence::fixed_channel_sequence(Channel::OpClass {
                channel: 0x6,
                opclass: 0x51
            },),
        }
    );
    let mut buf = vec![0x00; channel_sequence_tlv.measure_with(&())];
    buf.as_mut_slice().pwrite(channel_sequence_tlv, 0).unwrap();
    assert_eq!(buf, bytes);
}
