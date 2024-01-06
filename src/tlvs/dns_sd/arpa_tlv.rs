use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Pread, Pwrite,
};

use crate::common::{AWDLDnsName, AWDLStr, ReadLabelIterator};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
/// A TLV containing the hostname of the peer. Used for reverse DNS.
pub struct ArpaTLV<'a, I>
where
    I: IntoIterator<Item = AWDLStr<'a>> + Clone,
    <I as IntoIterator>::IntoIter: Clone,
{
    /// The actual arpa data.
    pub arpa: AWDLDnsName<I>,
}
impl<'a, I> MeasureWith<()> for ArpaTLV<'a, I>
where
    I: IntoIterator<Item = AWDLStr<'a>> + Clone,
    <I as IntoIterator>::IntoIter: Clone,
{
    fn measure_with(&self, ctx: &()) -> usize {
        self.arpa.measure_with(ctx) + 1
    }
}
impl<'a> TryFromCtx<'a> for ArpaTLV<'a, ReadLabelIterator<'a>> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        offset += 1; // Skip flags.
        let arpa = from.gread(&mut offset)?;
        Ok((Self { arpa }, offset))
    }
}
impl<'a, I> TryIntoCtx for ArpaTLV<'a, I>
where
    I: IntoIterator<Item = AWDLStr<'a>> + Clone,
    <I as IntoIterator>::IntoIter: Clone,
{
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        buf.gwrite(0x03u8, &mut offset)?;
        buf.gwrite(self.arpa, &mut offset)?;

        Ok(offset)
    }
}
//impl_tlv_conversion!(false, ArpaTLV, TLVType::Arpa, 3);
#[cfg(test)]
#[test]
fn test_arpa_tlv() {
    use crate::common::AWDLDnsCompression;
    use alloc::{vec, vec::Vec};
    use scroll::{Pread, Pwrite};

    let bytes = &include_bytes!("../../../test_bins/arpa_tlv.bin")[3..];

    let arpa_tlv = bytes.pread::<ArpaTLV<ReadLabelIterator>>(0).unwrap();
    assert_eq!(arpa_tlv.arpa.domain, AWDLDnsCompression::Local);
    assert_eq!(
        arpa_tlv.arpa.labels.collect::<Vec<_>>(),
        vec!["simon-framework".into()]
    );
    let mut buf = vec![0x00; arpa_tlv.measure_with(&())];
    buf.as_mut_slice()
        .pwrite::<ArpaTLV<ReadLabelIterator>>(arpa_tlv, 0)
        .unwrap();
    assert_eq!(buf, bytes);
}
