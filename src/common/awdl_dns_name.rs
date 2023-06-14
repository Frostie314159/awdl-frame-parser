use bin_utils::*;

use alloc::{borrow::Cow, vec::Vec};

use super::{awdl_dns_compression::AWDLDnsCompression, awdl_str::AWDLStr};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, PartialEq, Eq)]
/// A hostname combined with the [domain](AWDLDnsCompression).
pub struct AWDLDnsName<'a> {
    /// The labels of the peer.
    pub labels: Vec<AWDLStr<'a>>,

    /// The domain in [compressed form](AWDLDnsCompression).
    pub domain: AWDLDnsCompression,
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
        if self.labels.len() == 1 {
            let binding = self.labels[0].to_bytes();
            let labels = binding.iter().copied();
            let domain = <AWDLDnsCompression as Into<u16>>::into(self.domain).to_be_bytes();
            labels.chain(domain.iter().copied()).collect()
        } else {
            let labels = self.labels.iter().flat_map(|x| x.to_bytes().to_vec());
            let domain = <AWDLDnsCompression as Into<u16>>::into(self.domain).to_be_bytes();
            labels.chain(domain.iter().copied()).collect()
        }
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
