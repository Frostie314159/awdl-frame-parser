#[cfg(feature = "write")]
use bin_utils::*;
use mac_parser::MACAddress;
use try_take::try_take;

#[cfg(feature = "read")]
use crate::tlvs::FromTLVError;
use crate::tlvs::{TLVType, AWDLTLV};

use super::ChannelSequenceTLV;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
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
impl Read for SynchronizationParametersTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let mut fixed_data = try_take(data, 0x21).map_err(ParserError::TooLittleData)?;
        Ok(Self {
            next_channel: fixed_data.next().unwrap(),
            tx_counter: u16::from_le_bytes(fixed_data.next_chunk().unwrap()),
            master_channel: fixed_data.next().unwrap(),
            guard_time: fixed_data.next().unwrap(),
            aw_period: u16::from_le_bytes(fixed_data.next_chunk().unwrap()),
            af_period: u16::from_le_bytes(fixed_data.next_chunk().unwrap()),
            awdl_flags: u16::from_le_bytes(fixed_data.next_chunk().unwrap()),
            aw_ext_length: u16::from_le_bytes(fixed_data.next_chunk().unwrap()),
            aw_common_length: u16::from_le_bytes(fixed_data.next_chunk().unwrap()),
            remaining_aw_length: u16::from_le_bytes(fixed_data.next_chunk().unwrap()),
            min_ext_count: fixed_data.next().unwrap(),
            max_multicast_ext_count: fixed_data.next().unwrap(),
            max_unicast_ext_count: fixed_data.next().unwrap(),
            max_af_ext_count: fixed_data.next().unwrap(),
            master_address: MACAddress::from_bytes(&fixed_data.next_chunk().unwrap())?,
            presence_mode: fixed_data.next().unwrap(),
            aw_seq_number: {
                let _ = fixed_data.next(); // padding
                u16::from_le_bytes(fixed_data.next_chunk().unwrap())
            },
            ap_beacon_alignment_delta: u16::from_le_bytes(fixed_data.next_chunk().unwrap()),
            channel_sequence: ChannelSequenceTLV::from_bytes(data)?,
        })
    }
}
impl<'a> Write<'a> for SynchronizationParametersTLV {
    fn to_bytes(&self) -> alloc::borrow::Cow<'a, [u8]> {
        let mut data = [0; 33];

        data[0] = self.next_channel;
        data[1..3].copy_from_slice(&self.tx_counter.to_le_bytes());
        data[3] = self.master_channel;
        data[4] = self.guard_time;
        data[5..7].copy_from_slice(&self.aw_period.to_le_bytes());
        data[7..9].copy_from_slice(&self.af_period.to_le_bytes());
        data[9..11].copy_from_slice(&self.awdl_flags.to_le_bytes());
        data[11..13].copy_from_slice(&self.aw_ext_length.to_le_bytes());
        data[13..15].copy_from_slice(&self.aw_common_length.to_le_bytes());
        data[15..17].copy_from_slice(&self.remaining_aw_length.to_le_bytes());
        data[17] = self.min_ext_count;
        data[18] = self.max_multicast_ext_count;
        data[19] = self.max_unicast_ext_count;
        data[20] = self.max_af_ext_count;
        data[21..27].copy_from_slice(&self.master_address.to_bytes());
        data[27] = self.presence_mode;
        data[29..31].copy_from_slice(&self.aw_seq_number.to_le_bytes());
        data[31..33].copy_from_slice(&self.ap_beacon_alignment_delta.to_le_bytes());

        data.iter()
            .chain(self.channel_sequence.to_bytes().iter())
            .copied()
            .collect()
    }
}
#[cfg(feature = "read")]
impl TryFrom<AWDLTLV<'_>> for SynchronizationParametersTLV {
    type Error = FromTLVError;
    fn try_from(value: AWDLTLV<'_>) -> Result<Self, Self::Error> {
        if value.tlv_type != TLVType::SynchronizationParameters {
            return Err(FromTLVError::IncorrectTlvType);
        }
        if value.tlv_data.len() != 0x21 {
            return Err(FromTLVError::IncorrectTlvLength);
        }
        SynchronizationParametersTLV::from_bytes(&mut value.tlv_data.iter().copied())
            .map_err(FromTLVError::ParserError)
    }
}
#[cfg(feature = "write")]
impl From<SynchronizationParametersTLV> for AWDLTLV<'_> {
    fn from(value: SynchronizationParametersTLV) -> Self {
        Self {
            tlv_type: TLVType::SynchronizationParameters,
            tlv_data: value.to_bytes().to_vec().into(),
        }
    }
}
#[cfg(test)]
#[test]
fn test_sync_parameters_tlv() {
    use core::num::NonZeroU8;

    use crate::tlvs::sync_elect::{
        channel::{Band, ChannelBandwidth, ChannelEncoding, LegacyFlags, SupportChannel},
        channel_sequence::ChannelSequence,
    };

    let bytes = include_bytes!("../../../test_bins/sync_parameters_tlv.bin")[3..].to_vec();

    let sync_parameters_tlv =
        SynchronizationParametersTLV::from_bytes(&mut bytes.iter().copied()).unwrap();
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
                channel_encoding: ChannelEncoding::Legacy,
                step_count: NonZeroU8::new(4).unwrap(),
                channel_sequence: ChannelSequence::Legacy([
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Primary,
                            channel_bandwidth: ChannelBandwidth::Unknown(2),
                            band: Band::TwoPointFourGHz
                        },
                        8
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Primary,
                            channel_bandwidth: ChannelBandwidth::Unknown(2),
                            band: Band::TwoPointFourGHz
                        },
                        8
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Primary,
                            channel_bandwidth: ChannelBandwidth::Unknown(2),
                            band: Band::TwoPointFourGHz
                        },
                        8
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Primary,
                            channel_bandwidth: ChannelBandwidth::Unknown(2),
                            band: Band::TwoPointFourGHz
                        },
                        8
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        46
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        38
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        38
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        38
                    ),
                    (
                        LegacyFlags {
                            support_channel: SupportChannel::Lower,
                            channel_bandwidth: ChannelBandwidth::FourtyMHz,
                            band: Band::FiveGHz
                        },
                        38
                    ),
                ])
            }
        }
    );
    assert_eq!(sync_parameters_tlv.to_bytes()[..33], bytes[..33]);
}
