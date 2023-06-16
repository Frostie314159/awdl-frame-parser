#[cfg(feature = "write")]
use alloc::borrow::{Cow, ToOwned};
use bin_utils::*;
use mac_parser::MACAddress;

#[cfg(feature = "read")]
use crate::tlvs::FromTLVError;
use crate::tlvs::{TLVType, AWDLTLV};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SynchronizationParametersTLV {
    next_channel: u8,
    tx_counter: u16,
    master_channel: u8,
    guard_time: u8,
    aw_period: u16,
    af_period: u16,
    awdl_flags: u16, // Don't ask, don't know either.
    aw_ext_length: u16,
    aw_common_length: u16,
    remaining_aw_length: u16,
    min_ext_count: u8,
    max_multicast_ext_count: u8,
    max_unicast_ext_count: u8,
    max_af_ext_count: u8,
    master_address: MACAddress,
    presence_mode: u8,
    aw_seq_number: u16,
    ap_beacon_alignment_delta: u16,
    // The channel sequence is omitted, because it's wrong anyways.
}
impl Read for SynchronizationParametersTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let mut data = data.take(0x21);
        if data.len() < 0x21 {
            return Err(ParserError::TooLittleData(0x21 - data.len()));
        }
        Ok(Self {
            next_channel: data.next().unwrap(),
            tx_counter: u16::from_le_bytes(data.next_chunk().unwrap()),
            master_channel: data.next().unwrap(),
            guard_time: data.next().unwrap(),
            aw_period: u16::from_le_bytes(data.next_chunk().unwrap()),
            af_period: u16::from_le_bytes(data.next_chunk().unwrap()),
            awdl_flags: u16::from_le_bytes(data.next_chunk().unwrap()),
            aw_ext_length: u16::from_le_bytes(data.next_chunk().unwrap()),
            aw_common_length: u16::from_le_bytes(data.next_chunk().unwrap()),
            remaining_aw_length: u16::from_le_bytes(data.next_chunk().unwrap()),
            min_ext_count: data.next().unwrap(),
            max_multicast_ext_count: data.next().unwrap(),
            max_unicast_ext_count: data.next().unwrap(),
            max_af_ext_count: data.next().unwrap(),
            master_address: MACAddress::from_bytes(&data.next_chunk().unwrap())?,
            presence_mode: data.next().unwrap(),
            aw_seq_number: {
                let _ = data.next(); // padding
                u16::from_le_bytes(data.next_chunk().unwrap())
            },
            ap_beacon_alignment_delta: u16::from_le_bytes(data.next_chunk().unwrap()),
        })
    }
}
impl WriteFixed<73> for SynchronizationParametersTLV {
    fn to_bytes(&self) -> [u8; 73] {
        let mut data = [0; 73];

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

        // Channel Sequence
        data[33] = 0x0f;
        data[34] = 0x01;
        data[36] = 0x03;
        data[37..39].copy_from_slice(&[0xff; 2]);

        data
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
            tlv_data: Cow::Owned(value.to_bytes().as_slice().to_owned()),
        }
    }
}
#[cfg(test)]
#[test]
fn test_sync_parameters_tlv() {
    let bytes = include_bytes!("../../../test_bins/sync_parameters_tlv.bin")[3..].to_vec();

    let sync_parameters_tlv =
        SynchronizationParametersTLV::from_bytes(&mut bytes.iter().copied()).unwrap();
    assert_eq!(
        sync_parameters_tlv,
        SynchronizationParametersTLV {
            next_channel: 44,
            tx_counter: 38,
            master_channel: 44,
            guard_time: 0,
            aw_period: 16,
            af_period: 110,
            awdl_flags: 0x1800,
            aw_ext_length: 16,
            aw_common_length: 16,
            remaining_aw_length: 0,
            min_ext_count: 3,
            max_multicast_ext_count: 3,
            max_unicast_ext_count: 3,
            max_af_ext_count: 3,
            master_address: [0xce, 0x21, 0x1f, 0x62, 0x21, 0x22].into(),
            presence_mode: 4,
            aw_seq_number: 2025,
            ap_beacon_alignment_delta: 2022
        }
    );
    assert_eq!(sync_parameters_tlv.to_bytes()[..33], bytes[..33]);
}
