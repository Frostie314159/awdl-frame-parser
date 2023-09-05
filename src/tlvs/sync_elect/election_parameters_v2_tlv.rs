use bin_utils::*;
use mac_parser::MACAddress;

use crate::tlvs::{impl_tlv_conversion, TLVType};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
/// Another TLV describing the election parameters of the peer.
pub struct ElectionParametersV2TLV {
    /// MAC address of the master
    pub master_address: MACAddress,

    /// MAC address of the peer this peer is syncing to
    pub sync_address: MACAddress,

    /// Counter value of the master
    pub master_counter: u32,

    /// Distance to the current master
    pub distance_to_master: u32,

    /// Metric of the master
    pub master_metric: u32,

    /// Self metric of the peer
    pub self_metric: u32,

    /// Self counter of the peer
    pub self_counter: u32,
}
#[cfg(feature = "read")]
impl ReadFixed<40> for ElectionParametersV2TLV {
    fn from_bytes(data: &[u8; 40]) -> Result<Self, ParserError> {
        let mut data = data.iter().copied();
        let master_address = MACAddress::from_bytes(&data.next_chunk().unwrap()).unwrap(); // Infallible
        let sync_address = MACAddress::from_bytes(&data.next_chunk().unwrap()).unwrap(); // Infallible
        let master_counter = u32::from_le_bytes(data.next_chunk().unwrap());
        let distance_to_master = u32::from_le_bytes(data.next_chunk().unwrap());
        let master_metric = u32::from_le_bytes(data.next_chunk().unwrap());
        let self_metric = u32::from_le_bytes(data.next_chunk().unwrap());
        let _ = data.next_chunk::<8>();
        let self_counter = u32::from_le_bytes(data.next_chunk().unwrap());

        Ok(Self {
            master_address,
            sync_address,
            master_counter,
            distance_to_master,
            master_metric,
            self_metric,
            self_counter,
        })
    }
}
#[cfg(feature = "write")]
impl WriteFixed<40> for ElectionParametersV2TLV {
    fn to_bytes(&self) -> [u8; 40] {
        let mut bytes = [0x00; 40];
        bytes[0..6].copy_from_slice(&self.master_address.to_bytes());
        bytes[6..12].copy_from_slice(&self.sync_address.to_bytes());
        bytes[12..16].copy_from_slice(&self.master_counter.to_le_bytes());
        bytes[16..20].copy_from_slice(&self.distance_to_master.to_le_bytes());
        bytes[20..24].copy_from_slice(&self.master_metric.to_le_bytes());
        bytes[24..28].copy_from_slice(&self.self_metric.to_le_bytes());
        bytes[36..40].copy_from_slice(&self.self_counter.to_le_bytes());

        bytes
    }
}
impl_tlv_conversion!(
    true,
    ElectionParametersV2TLV,
    TLVType::ElectionParametersV2,
    40
);
#[cfg(test)]
#[test]
fn test_election_parameters_v2_tlv() {
    use crate::tlvs::{AWDLTLV, TLV};

    let bytes = include_bytes!("../../../test_bins/election_parameters_v2_tlv.bin");

    let tlv = TLV::from_bytes(&mut bytes.iter().copied()).unwrap();

    let election_parameters_v2_tlv = ElectionParametersV2TLV::try_from(tlv.clone()).unwrap();
    assert_eq!(
        tlv,
        <ElectionParametersV2TLV as Into<AWDLTLV>>::into(election_parameters_v2_tlv.clone())
    );

    assert_eq!(
        election_parameters_v2_tlv,
        ElectionParametersV2TLV {
            master_address: [0xce, 0x21, 0x1f, 0x62, 0x21, 0x22].into(),
            sync_address: [0xce, 0x21, 0x1f, 0x62, 0x21, 0x22].into(),
            master_counter: 960,
            distance_to_master: 1,
            master_metric: 650,
            self_metric: 650,
            self_counter: 30,
        }
    );

    assert_eq!(election_parameters_v2_tlv.to_bytes(), bytes[3..]);
}
