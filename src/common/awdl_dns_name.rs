use core::{
    fmt::{Display, Write},
    iter::repeat,
};
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Pread, Pwrite, NETWORK,
};

use crate::tlvs::RawAWDLTLV;

use super::{awdl_dns_compression::AWDLDnsCompression, awdl_str::AWDLStr};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ReadLabelIterator<'a> {
    bytes: &'a [u8],
    offset: usize,
}
impl<'a> ReadLabelIterator<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }
}
impl MeasureWith<()> for ReadLabelIterator<'_> {
    fn measure_with(&self, _ctx: &()) -> usize {
        self.bytes.len()
    }
}
impl<'a> Iterator for ReadLabelIterator<'a> {
    type Item = AWDLStr<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.bytes.gread(&mut self.offset).ok()
    }
}
impl ExactSizeIterator for ReadLabelIterator<'_> {
    fn len(&self) -> usize {
        repeat(())
            .scan(0usize, |offset, _| {
                self.bytes.gread::<RawAWDLTLV>(offset).ok()
            })
            .count()
    }
}

#[derive(Clone, Debug, Default, Hash)]
/// A hostname combined with the [domain](AWDLDnsCompression).
pub struct AWDLDnsName<'a, I = ReadLabelIterator<'a>>
where
    I: IntoIterator<Item = AWDLStr<'a>>,
{
    /// The labels of the peer.
    pub labels: I,

    /// The domain in [compressed form](AWDLDnsCompression).
    pub domain: AWDLDnsCompression,
}
impl<'a, I: IntoIterator<Item = AWDLStr<'a>> + Copy> Copy for AWDLDnsName<'a, I> {}
impl<'a, I: IntoIterator<Item = AWDLStr<'a>> + Clone> Eq for AWDLDnsName<'a, I> {}
impl<'a, LhsIterator, RhsIterator> PartialEq<AWDLDnsName<'a, RhsIterator>>
    for AWDLDnsName<'a, LhsIterator>
where
    LhsIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    RhsIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
{
    fn eq(&self, other: &AWDLDnsName<'a, RhsIterator>) -> bool {
        self.labels.clone().into_iter().eq(other.labels.clone())
    }
}

impl<'a, I> MeasureWith<()> for AWDLDnsName<'a, I>
where
    I: IntoIterator<Item = AWDLStr<'a>> + Clone,
{
    fn measure_with(&self, ctx: &()) -> usize {
        self.labels
            .clone()
            .into_iter()
            .map(|label| label.measure_with(ctx))
            .sum::<usize>()
            + 2
    }
}
impl<'a> TryFromCtx<'a> for AWDLDnsName<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        let label_bytes = from.gread_with(&mut offset, from.len() - 2)?;
        let domain =
            AWDLDnsCompression::from_representation(from.gread_with(&mut offset, NETWORK)?);
        Ok((
            Self {
                labels: ReadLabelIterator::new(label_bytes),
                domain,
            },
            offset,
        ))
    }
}
impl<'a, I: IntoIterator<Item = AWDLStr<'a>>> TryIntoCtx for AWDLDnsName<'a, I> {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        // Using for loop, because of ? operator.
        for x in self.labels {
            buf.gwrite(x, &mut offset)?;
        }
        buf.gwrite_with(self.domain.to_representation(), &mut offset, NETWORK)?;
        Ok(offset)
    }
}
impl<'a, I> Display for AWDLDnsName<'a, I>
where
    I: IntoIterator<Item = AWDLStr<'a>> + Clone,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for label in self.labels.clone() {
            f.write_str(&label)?;
            f.write_char('.')?;
        }
        f.write_str(self.domain.to_static_string())
    }
}
#[cfg(test)]
#[test]
fn test_dns_name() {
    use alloc::vec;
    let bytes = [
        0x04, b'a', b'w', b'd', b'l', 0x04, b'a', b'w', b'd', b'l', 0xc0, 0x0c,
    ]
    .as_slice();
    let dns_name = bytes.pread::<AWDLDnsName>(0).unwrap();
    assert_eq!(
        dns_name,
        AWDLDnsName {
            labels: ["awdl".into(), "awdl".into()],
            domain: AWDLDnsCompression::Local
        }
    );
    let mut buf = vec![0x00; dns_name.measure_with(&())];
    buf.pwrite(dns_name, 0).unwrap();
    assert_eq!(bytes, buf);
}
