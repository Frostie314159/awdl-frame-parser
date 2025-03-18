use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Pread, Pwrite,
};

use crate::{
    common::{AWDLDnsName, AWDLStr, ReadLabelIterator},
    tlvs::{AWDLTLVType, AwdlTlv},
};

#[derive(Clone, Copy, Debug, Default, Hash)]
/// A TLV containing the hostname of the peer. Used for reverse DNS.
pub struct ArpaTLV<I> {
    /// The actual arpa data.
    pub arpa: AWDLDnsName<I>,
}
impl<I> AwdlTlv for ArpaTLV<I> {
    const TLV_TYPE: AWDLTLVType = AWDLTLVType::Arpa;
}
impl<'a, I: IntoIterator<Item = AWDLStr<'a>> + Clone> Eq for ArpaTLV<I> {}
impl<'a, LhsIterator, RhsIterator> PartialEq<ArpaTLV<RhsIterator>> for ArpaTLV<LhsIterator>
where
    LhsIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    RhsIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
{
    fn eq(&self, other: &ArpaTLV<RhsIterator>) -> bool {
        self.arpa == other.arpa
    }
}
impl<'a, I> MeasureWith<()> for ArpaTLV<I>
where
    I: IntoIterator<Item = AWDLStr<'a>> + Clone,
{
    fn measure_with(&self, ctx: &()) -> usize {
        self.arpa.measure_with(ctx) + 1
    }
}
impl<'a> TryFromCtx<'a> for ArpaTLV<ReadLabelIterator<'a>> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        offset += 1; // Skip flags.
        let arpa = from.gread(&mut offset)?;
        Ok((Self { arpa }, offset))
    }
}
impl<'a, I> TryIntoCtx for ArpaTLV<I>
where
    I: IntoIterator<Item = AWDLStr<'a>>,
{
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        buf.gwrite(0x03u8, &mut offset)?;
        buf.gwrite(self.arpa, &mut offset)?;

        Ok(offset)
    }
}
/// The default arpa tlv returned by reading.
pub type DefaultArpaTLV<'a> = ArpaTLV<ReadLabelIterator<'a>>;
#[cfg(test)]
#[test]
fn test_arpa_tlv() {
    use crate::common::AWDLDnsCompression;
    use alloc::vec;
    use scroll::{Pread, Pwrite};

    let bytes = &include_bytes!("../../../test_bins/arpa_tlv.bin")[3..];

    let arpa_tlv = bytes.pread::<ArpaTLV<ReadLabelIterator>>(0).unwrap();
    assert_eq!(
        arpa_tlv,
        ArpaTLV {
            arpa: AWDLDnsName {
                labels: ["simon-framework".into()],
                domain: AWDLDnsCompression::Local
            }
        }
    );
    let mut buf = vec![0x00; arpa_tlv.measure_with(&())];
    buf.as_mut_slice()
        .pwrite::<ArpaTLV<ReadLabelIterator>>(arpa_tlv, 0)
        .unwrap();
    assert_eq!(buf, bytes);
}
