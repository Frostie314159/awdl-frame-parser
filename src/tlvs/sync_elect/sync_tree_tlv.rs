use bin_utils::*;

use alloc::vec::Vec;

use crate::tlvs::{TLVType, TLV};

#[cfg(feature = "read")]
use crate::tlvs::FromTLVError;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub struct SyncTreeTLV {
    pub tree: Vec<[u8; 6]>,
}
#[cfg(feature = "read")]
impl Read for SyncTreeTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        Ok(Self {
            tree: data.array_chunks::<6>().collect(),
        })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for SyncTreeTLV {
    fn to_bytes(&self) -> alloc::borrow::Cow<'a, [u8]> {
        match self.tree.len() {
            0 => [0x00; 12].as_slice().into(),
            1 => [[0x00; 6]]
                .iter()
                .chain(self.tree.iter())
                .copied()
                .flatten()
                .collect(),
            _ => self.tree.iter().flatten().copied().collect(),
        }
    }
}
#[cfg(feature = "read")]
impl<'a> TryFrom<TLV<'a>> for SyncTreeTLV {
    type Error = FromTLVError;
    fn try_from(value: TLV<'a>) -> Result<Self, Self::Error> {
        if value.tlv_type != TLVType::SynchronizationTree {
            return Err(FromTLVError::IncorrectTlvType);
        }
        SyncTreeTLV::from_bytes(&mut value.tlv_data.iter().copied())
            .map_err(FromTLVError::ParserError)
    }
}
#[cfg(feature = "write")]
impl From<SyncTreeTLV> for TLV<'_> {
    fn from(value: SyncTreeTLV) -> Self {
        TLV {
            tlv_type: TLVType::SynchronizationTree,
            tlv_data: value.to_bytes(),
        }
    }
}
#[cfg(test)]
#[test]
fn test_sync_tree_tlv() {
    let bytes = include_bytes!("../../../test_bins/sync_tree_tlv.bin");

    let tlv = TLV::from_bytes(&mut bytes.iter().copied()).unwrap();

    let sync_tree_tlv = SyncTreeTLV::try_from(tlv.clone()).unwrap();
    assert_eq!(tlv, <SyncTreeTLV as Into<TLV>>::into(sync_tree_tlv.clone()));

    assert_eq!(
        sync_tree_tlv,
        SyncTreeTLV {
            tree: alloc::vec![[0xbe, 0x70, 0xf3, 0x17, 0x21, 0xf2], [0x00; 6]]
        }
    );

    assert_eq!(sync_tree_tlv.to_bytes(), &bytes[3..]);
}
