use bin_utils::*;

use alloc::vec::Vec;
use mac_parser::MACAddress;

use crate::{impl_tlv_conversion, tlvs::TLVType};

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
impl_tlv_conversion!(false, SyncTreeTLV, TLVType::SynchronizationTree, 0);

#[cfg(test)]
#[test]
fn test_sync_tree_tlv() {
    use crate::tlvs::AWDLTLV;
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
