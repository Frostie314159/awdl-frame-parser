use bin_utils::*;

use alloc::vec::Vec;
use mac_parser::MACAddress;

#[cfg(feature = "read")]
use crate::tlvs::FromTLVError;
use crate::tlvs::{TLVType, AWDLTLV};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub struct SyncTreeTLV {
    pub tree: Vec<MACAddress>,
}
#[cfg(feature = "read")]
impl Read for SyncTreeTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        Ok(Self {
            tree: data
                .array_chunks::<6>()
                .map(|x| MACAddress::from_bytes(&x).unwrap())
                .collect(),
        })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for SyncTreeTLV {
    fn to_bytes(&self) -> alloc::borrow::Cow<'a, [u8]> {
        self.tree.iter().flat_map(MACAddress::to_bytes).collect()
    }
}
#[cfg(feature = "read")]
impl<'a> TryFrom<AWDLTLV<'a>> for SyncTreeTLV {
    type Error = FromTLVError;
    fn try_from(value: AWDLTLV<'a>) -> Result<Self, Self::Error> {
        if value.tlv_type != TLVType::SynchronizationTree {
            return Err(FromTLVError::IncorrectTlvType);
        }
        SyncTreeTLV::from_bytes(&mut value.tlv_data.iter().copied())
            .map_err(FromTLVError::ParserError)
    }
}
#[cfg(feature = "write")]
impl From<SyncTreeTLV> for AWDLTLV<'_> {
    fn from(value: SyncTreeTLV) -> Self {
        AWDLTLV {
            tlv_type: TLVType::SynchronizationTree,
            tlv_data: value.to_bytes(),
        }
    }
}
#[cfg(test)]
#[test]
fn test_sync_tree_tlv() {
    let bytes = include_bytes!("../../../test_bins/sync_tree_tlv.bin");

    let tlv = AWDLTLV::from_bytes(&mut bytes.iter().copied()).unwrap();

    let sync_tree_tlv = SyncTreeTLV::try_from(tlv.clone()).unwrap();
    assert_eq!(
        tlv,
        <SyncTreeTLV as Into<AWDLTLV>>::into(sync_tree_tlv.clone())
    );

    assert_eq!(
        sync_tree_tlv,
        SyncTreeTLV {
            tree: alloc::vec![
                [0xbe, 0x70, 0xf3, 0x17, 0x21, 0xf2].into(),
                [0x00; 6].into()
            ]
        }
    );

    assert_eq!(sync_tree_tlv.to_bytes(), &bytes[3..]);
}
