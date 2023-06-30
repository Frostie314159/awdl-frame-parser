#[cfg(feature = "debug")]
use core::fmt::Debug;

use bin_utils::*;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

#[cfg(feature = "write")]
use alloc::borrow::Cow;

use super::{awdl_dns_compression::AWDLDnsCompression, awdl_str::AWDLStr};

#[derive(Clone, Default, PartialEq, Eq)]
/// A hostname combined with the [domain](AWDLDnsCompression).
pub struct AWDLDnsName<'a> {
    /// The labels of the peer.
    pub labels: Vec<AWDLStr<'a>>,

    /// The domain in [compressed form](AWDLDnsCompression).
    pub domain: AWDLDnsCompression,
}
impl AWDLDnsName<'_> {
    #[inline]
    /// Turns the string into an Iterator over bytes without allocating.
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        self.labels
            .iter()
            .flat_map(AWDLStr::iter)
            .chain(<AWDLDnsCompression as Into<u16>>::into(self.domain).to_be_bytes())
    }
    #[inline]
    /// Returns the complete length in bytes.
    pub fn len(&self) -> usize {
        self.labels.iter().map(|x| x.total_len()).sum::<usize>() + 2
    }
    #[inline]
    /// Returns false if either the labels vector is empty or the only element in the vector is empty.
    pub fn is_empty(&self) -> bool {
        match self.labels.get(0) {
            Some(label) => label.is_empty(),
            None => true,
        }
    }
}
#[cfg(feature = "read")]
impl Read for AWDLDnsName<'_> {
    fn from_bytes<'a>(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        if data.len() < 3 {
            return Err(ParserError::TooLittleData(3 - data.len()));
        }
        let mut label_data = data.take(data.len() - 2);
        let labels = (0..)
            .map_while(|_| AWDLStr::from_bytes(&mut label_data).ok())
            .collect();
        let domain = u16::from_be_bytes(
            data.next_chunk()
                .map_err(|_| ParserError::TooLittleData(2))?,
        )
        .into();

        Ok(Self { labels, domain })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for AWDLDnsName<'a> {
    fn to_bytes(&self) -> Cow<'a, [u8]> {
        self.iter().collect()
    }
}
impl ToString for AWDLDnsName<'_> {
    fn to_string(&self) -> String {
        self.labels
            .iter()
            .fold(String::new(), |acc, x| acc + x + ".")
            + &self.domain.to_string()
    }
}
#[cfg(feature = "debug")]
impl Debug for AWDLDnsName<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_string())
    }
}
#[cfg(test)]
#[test]
fn test_dns_name() {
    use alloc::vec;
    let bytes: [u8; 7] = [0x04, 0x61, 0x77, 0x64, 0x6C, 0xc0, 0x0c];
    let dns_name = <AWDLDnsName as Read>::from_bytes(&mut bytes.iter().copied()).unwrap();
    assert_eq!(
        dns_name,
        AWDLDnsName {
            labels: vec!["awdl".into()],
            domain: AWDLDnsCompression::Local
        }
    );
    let dns_name_bytes = dns_name.to_bytes();
    assert_eq!(<&[u8] as Into<Cow<[u8]>>>::into(&bytes), dns_name_bytes);
    let bytes: [u8; 12] = [
        0x04, 0x61, 0x77, 0x64, 0x6C, 0x04, 0x61, 0x77, 0x64, 0x6C, 0xc0, 0x0c,
    ];
    let dns_name = AWDLDnsName::from_bytes(&mut bytes.iter().copied()).unwrap();
    assert_eq!(
        dns_name,
        AWDLDnsName {
            labels: vec!["awdl".into(), "awdl".into()],
            domain: AWDLDnsCompression::Local
        }
    );
    let dns_name_bytes = dns_name.to_bytes();
    assert_eq!(dns_name_bytes, Cow::Borrowed(bytes.as_slice()));
}
