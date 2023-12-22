#[cfg(feature = "debug")]
use core::fmt::Debug;
use core::{
    fmt::{Display, Write},
    iter::repeat,
};

use alloc::vec::Vec;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Pread, Pwrite, NETWORK,
};

use super::{awdl_dns_compression::AWDLDnsCompression, awdl_str::AWDLStr};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
/// A hostname combined with the [domain](AWDLDnsCompression).
pub struct AWDLDnsName<'a> {
    /// The labels of the peer.
    pub labels: Vec<AWDLStr<'a>>,

    /// The domain in [compressed form](AWDLDnsCompression).
    pub domain: AWDLDnsCompression,
}
impl<'a> MeasureWith<()> for AWDLDnsName<'a> {
    fn measure_with(&self, _ctx: &()) -> usize {
        self.labels
            .iter()
            .map(AWDLStr::size_in_bytes)
            .sum::<usize>()
            + 2
    }
}
impl<'a> TryFromCtx<'a> for AWDLDnsName<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        let labels = from.get(..from.len() - 2).ok_or(scroll::Error::BadInput {
            size: 0x00,
            msg: "Input too short.",
        })?;
        let labels = repeat(())
            .map_while(|_| labels.gread::<AWDLStr<'_>>(&mut offset).ok())
            .collect();
        let domain =
            AWDLDnsCompression::from_representation(from.gread_with(&mut offset, NETWORK)?);
        Ok((Self { labels, domain }, offset))
    }
}
impl<'a> TryIntoCtx for AWDLDnsName<'a> {
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
impl Display for AWDLDnsName<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for label in self.labels.iter() {
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
    let dns_name = bytes.pread::<AWDLDnsName<'_>>(0).unwrap();
    assert_eq!(
        dns_name,
        AWDLDnsName {
            labels: vec!["awdl".into(), "awdl".into()],
            domain: AWDLDnsCompression::Local
        }
    );
    let mut buf = [0x00; 12];
    buf.pwrite(dns_name, 0).unwrap();
    assert_eq!(bytes, buf);
}
