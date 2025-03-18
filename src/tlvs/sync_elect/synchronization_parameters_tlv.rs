use mac_parser::MACAddress;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

use crate::tlvs::{AWDLTLVType, AwdlTlv};

use super::ChannelSequenceTLV;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
/// The synchronization parameters of the peer.
pub struct SynchronizationParametersTLV {
    pub next_channel: u8,
    pub tx_counter: u16,
    pub master_channel: u8,
    pub guard_time: u8,
    pub aw_period: u16,
    pub af_period: u16,
    pub awdl_flags: u16, // Don't ask, don't know either.
    pub aw_ext_length: u16,
    pub aw_common_length: u16,
    pub remaining_aw_length: u16,
    pub min_ext_count: u8,
    pub max_multicast_ext_count: u8,
    pub max_unicast_ext_count: u8,
    pub max_af_ext_count: u8,
    pub master_address: MACAddress,
    pub presence_mode: u8,
    pub aw_seq_number: u16,
    pub ap_beacon_alignment_delta: u16,
    /// This isn't actually a TLV, but contains the functionality we need.
    pub channel_sequence: ChannelSequenceTLV,
}
impl AwdlTlv for SynchronizationParametersTLV {
    const TLV_TYPE: AWDLTLVType = AWDLTLVType::SynchronizationParameters;
}
impl MeasureWith<()> for SynchronizationParametersTLV {
    fn measure_with(&self, ctx: &()) -> usize {
        32 + self.channel_sequence.measure_with(ctx)
    }
}
impl<'a> TryFromCtx<'a> for SynchronizationParametersTLV {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;

        let next_channel = from.gread_with(&mut offset, Endian::Little)?;
        let tx_counter = from.gread_with(&mut offset, Endian::Little)?;
        let master_channel = from.gread_with(&mut offset, Endian::Little)?;
        let guard_time = from.gread_with(&mut offset, Endian::Little)?;
        let aw_period = from.gread_with(&mut offset, Endian::Little)?;
        let af_period = from.gread_with(&mut offset, Endian::Little)?;
        let awdl_flags = from.gread_with(&mut offset, Endian::Little)?;
        let aw_ext_length = from.gread_with(&mut offset, Endian::Little)?;
        let aw_common_length = from.gread_with(&mut offset, Endian::Little)?;
        let remaining_aw_length = from.gread_with(&mut offset, Endian::Little)?;
        let min_ext_count = from.gread_with(&mut offset, Endian::Little)?;
        let max_multicast_ext_count = from.gread_with(&mut offset, Endian::Little)?;
        let max_unicast_ext_count = from.gread_with(&mut offset, Endian::Little)?;
        let max_af_ext_count = from.gread_with(&mut offset, Endian::Little)?;
        let master_address = from.gread(&mut offset)?;
        let presence_mode = from.gread_with(&mut offset, Endian::Little)?;
        offset += 1;
        let aw_seq_number = from.gread_with(&mut offset, Endian::Little)?;
        let ap_beacon_alignment_delta = from.gread_with(&mut offset, Endian::Little)?;
        let channel_sequence = from.gread(&mut offset)?;
        Ok((
            Self {
                next_channel,
                tx_counter,
                master_channel,
                guard_time,
                aw_period,
                af_period,
                awdl_flags,
                aw_ext_length,
                aw_common_length,
                remaining_aw_length,
                min_ext_count,
                max_multicast_ext_count,
                max_unicast_ext_count,
                max_af_ext_count,
                master_address,
                presence_mode,
                aw_seq_number,
                ap_beacon_alignment_delta,
                channel_sequence,
            },
            offset,
        ))
    }
}
impl TryIntoCtx for SynchronizationParametersTLV {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;

        buf.gwrite_with(self.next_channel, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.tx_counter, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.master_channel, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.guard_time, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.aw_period, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.af_period, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.awdl_flags, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.aw_ext_length, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.aw_common_length, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.remaining_aw_length, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.min_ext_count, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.max_multicast_ext_count, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.max_unicast_ext_count, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.max_af_ext_count, &mut offset, Endian::Little)?;
        buf.gwrite(self.master_address, &mut offset)?;
        buf.gwrite_with(self.presence_mode, &mut offset, Endian::Little)?;
        offset += 1;
        buf.gwrite_with(self.aw_seq_number, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.ap_beacon_alignment_delta, &mut offset, Endian::Little)?;
        buf.gwrite(self.channel_sequence, &mut offset)?;

        offset -= 1;

        Ok(offset)
    }
}
#[cfg(test)]
#[test]
fn test_sync_parameters_tlv() {
    use core::num::NonZeroU8;

    use alloc::vec;

    use crate::tlvs::sync_elect::{
        channel::{Band, ChannelBandwidth, LegacyFlags, SupportChannel},
        channel_sequence::ChannelSequence,
    };

    let bytes = &include_bytes!("../../../test_bins/sync_parameters_tlv.bin")[3..];

    let sync_parameters_tlv = bytes.pread::<SynchronizationParametersTLV>(0).unwrap();
    assert_eq!(
        sync_parameters_tlv,
        SynchronizationParametersTLV {
            next_channel: 44,
            tx_counter: 49,
            master_channel: 6,
            guard_time: 0,
            aw_period: 16,
            af_period: 110,
            awdl_flags: 0x1800,
            aw_ext_length: 16,
            aw_common_length: 16,
            remaining_aw_length: 1,
            min_ext_count: 3,
            max_multicast_ext_count: 3,
            max_unicast_ext_count: 3,
            max_af_ext_count: 3,
            master_address: [0xce, 0x21, 0x1f, 0x62, 0x21, 0x22].into(),
            presence_mode: 4,
            aw_seq_number: 1988,
            ap_beacon_alignment_delta: 1986,
            channel_sequence: ChannelSequenceTLV {
                step_count: NonZeroU8::new(4).unwrap(),
                channel_sequence: ChannelSequence::Legacy([
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Primary,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::TwoPointFourGHz
                        },
                        8
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Primary,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::TwoPointFourGHz
                        },
                        8
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Primary,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::TwoPointFourGHz
                        },
                        8
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Primary,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::TwoPointFourGHz
                        },
                        8
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        38
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        38
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        38
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::EightyMHz,
                            band: Band::FiveGHz
                        },
                        38
                    ),
                ])
            }
        }
    );
    let mut buf = vec![0x00; sync_parameters_tlv.measure_with(&())];
    buf.as_mut_slice().pwrite(sync_parameters_tlv, 0).unwrap();
    assert_eq!(buf, bytes);
}
