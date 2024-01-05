use core::fmt::Display;

use mac_parser::MACAddress;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Pread, Pwrite,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReadMACIterator<'a> {
    pub bytes: &'a [u8],
    pub offset: usize,
}
impl<'a> ReadMACIterator<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }
}
impl Iterator for ReadMACIterator<'_> {
    type Item = MACAddress;
    fn next(&mut self) -> Option<Self::Item> {
        self.bytes.gread(&mut self.offset).ok()
    }
}
impl ExactSizeIterator for ReadMACIterator<'_> {
    fn len(&self) -> usize {
        self.bytes.len() / 6
    }
}
impl Display for ReadMACIterator<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(*self).finish()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// This describes the structure of the AWDL mesh.
/// The contained mac address are in descending order,
/// with the first one being the mesh master and the other ones being sync masters.
pub struct SyncTreeTLV<MACIterator> {
    /// The MACs.
    pub tree: MACIterator,
}
impl<MACIterator> MeasureWith<()> for SyncTreeTLV<MACIterator>
where
    MACIterator: IntoIterator<Item = MACAddress> + ExactSizeIterator,
    <MACIterator as IntoIterator>::IntoIter: Clone,
{
    fn measure_with(&self, _ctx: &()) -> usize {
        self.tree.len() * 6
    }
}
impl<'a> TryFromCtx<'a> for SyncTreeTLV<ReadMACIterator<'a>> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        Ok((
            Self {
                tree: ReadMACIterator::new(from),
            },
            from.len() / 6,
        ))
    }
}
impl<MACIterator> TryIntoCtx for SyncTreeTLV<MACIterator>
where
    MACIterator: IntoIterator<Item = MACAddress>,
    <MACIterator as IntoIterator>::IntoIter: Clone,
{
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
    use alloc::{vec, vec::Vec};
    use mac_parser::ZERO;
    use scroll::Pread;

    let bytes = &include_bytes!("../../../test_bins/sync_tree_tlv.bin")[3..];

    let sync_tree_tlv = bytes.pread::<SyncTreeTLV<_>>(0).unwrap();

    assert_eq!(
        sync_tree_tlv.tree.collect::<Vec<_>>(),
        vec![MACAddress::new([0xbe, 0x70, 0xf3, 0x17, 0x21, 0xf2]), ZERO]
    );

    let mut buf = vec![0x00; sync_tree_tlv.measure_with(&())];
    buf.pwrite(sync_tree_tlv, 0).unwrap();
    assert_eq!(buf, bytes);
}
