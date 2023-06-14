use bin_utils::*;

use crate::{impl_tlv_conversion_fixed, tlvs::TLVType};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
/// A TLV describing the election parameters of a peer.
pub struct ElectionParametersTLV {
    /// Unknown
    pub flags: u8,

    /// Unknown
    pub id: u16,

    /// Distance to the mesh master
    pub distance_to_master: u8,

    /// Address of the master
    pub master_address: [u8; 6],

    /// Self metric of the master
    pub master_metric: u32,

    /// Own self metric
    pub self_metric: u32,
}
#[cfg(feature = "read")]
impl ReadFixed<21> for ElectionParametersTLV {
    fn from_bytes(data: &[u8; 21]) -> Result<Self, ParserError> {
        let mut data = data.iter().copied();
        let flags = data.next().unwrap();
        let id = u16::from_le_bytes(data.next_chunk().unwrap()); // In reality this is always zero.
        let distance_to_master = data.next().unwrap();
        let _ = data.next();
        let master_address = data.next_chunk::<6>().unwrap();
        let master_metric = u32::from_le_bytes(data.next_chunk().unwrap());
        let self_metric = u32::from_le_bytes(data.next_chunk().unwrap());
        Ok(Self {
            flags,
            id,
            distance_to_master,
            master_address,
            master_metric,
            self_metric,
        })
    }
}
#[cfg(feature = "write")]
impl WriteFixed<21> for ElectionParametersTLV {
    fn to_bytes(&self) -> [u8; 21] {
        let mut bytes = [0x00; 21];
        bytes[0] = self.flags;
        bytes[1..3].copy_from_slice(&self.id.to_le_bytes());
        bytes[3] = self.distance_to_master;
        bytes[5..11].copy_from_slice(&self.master_address);
        bytes[11..15].copy_from_slice(&self.master_metric.to_le_bytes());
        bytes[15..19].copy_from_slice(&self.self_metric.to_le_bytes());
        bytes
    }
}
impl_tlv_conversion_fixed!(ElectionParametersTLV, TLVType::ElectionParameters, 21);

#[cfg(test)]
#[test]
fn test_election_parameters_tlv() {
    use crate::tlvs::TLV;

    let bytes = include_bytes!("../../../test_bins/election_parameters_tlv.bin");

    let tlv = TLV::from_bytes(&mut bytes.iter().copied()).unwrap();

    let election_parameters_tlv = ElectionParametersTLV::try_from(tlv.clone()).unwrap();
    assert_eq!(
        tlv,
        <ElectionParametersTLV as Into<TLV>>::into(election_parameters_tlv.clone())
    );

    assert_eq!(
        election_parameters_tlv,
        ElectionParametersTLV {
            flags: 0x00,
            id: 0x00,
            distance_to_master: 0x02,
            master_address: [0x3a, 0xb4, 0x08, 0x6e, 0x66, 0x3d],
            master_metric: 541,
            self_metric: 60
        }
    );

    assert_eq!(election_parameters_tlv.to_bytes(), bytes[3..]);
}
