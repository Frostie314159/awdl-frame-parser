use alloc::vec::Vec;
use mac_parser::MACAddress;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Pwrite,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
/// This describes the structure of the AWDL mesh.
/// The contained mac address are in descending order,
/// with the first one being the mesh master and the other ones being sync masters.
pub struct SyncTreeTLV {
    /// The MACs.
    pub tree: Vec<MACAddress>,
}
impl MeasureWith<()> for SyncTreeTLV {
    fn measure_with(&self, _ctx: &()) -> usize {
        self.tree.len() * 6
    }
}
impl<'a> TryFromCtx<'a> for SyncTreeTLV {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let tree = from
            .array_chunks::<6>()
            .copied()
            .map(MACAddress::new)
            .collect();
        Ok((Self { tree }, from.len() / 6))
    }
}
impl TryIntoCtx for SyncTreeTLV {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        for address in self.tree {
            buf.gwrite(address.as_slice(), &mut offset)?;
        }
        Ok(offset)
    }
}

#[cfg(test)]
#[test]
fn test_sync_tree_tlv() {
    use alloc::vec;
    use mac_parser::ZERO;
    use scroll::Pread;

    let bytes = &include_bytes!("../../../test_bins/sync_tree_tlv.bin")[3..];

    let sync_tree_tlv = bytes.pread::<SyncTreeTLV>(0).unwrap();

    assert_eq!(
        sync_tree_tlv,
        SyncTreeTLV {
            tree: vec![[0xbe, 0x70, 0xf3, 0x17, 0x21, 0xf2].into(), ZERO]
        }
    );

    let mut buf = vec![0x00; sync_tree_tlv.measure_with(&())];
    buf.pwrite(sync_tree_tlv, 0).unwrap();
    assert_eq!(buf, bytes);
}
