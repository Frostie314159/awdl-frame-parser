#[cfg(feature = "debug")]
use core::fmt::Debug;

use bin_utils::*;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use super::{awdl_dns_compression::AWDLDnsCompression, awdl_str::AWDLStr};

#[derive(Clone, Default, PartialEq, Eq)]
/// A hostname combined with the [domain](AWDLDnsCompression).
pub struct AWDLDnsName {
    /// The labels of the peer.
    pub labels: Vec<AWDLStr>,

    /// The domain in [compressed form](AWDLDnsCompression).
    pub domain: AWDLDnsCompression,
}
impl AWDLDnsName {
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
impl Read for AWDLDnsName {
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
impl Write for AWDLDnsName {
    fn to_bytes(&self) -> Vec<u8> {
        if self.labels.len() == 1 {
            let binding = self.labels[0].to_bytes();
            let labels = binding.iter().copied();
            let domain = <AWDLDnsCompression as Into<u16>>::into(self.domain).to_be_bytes();
            labels.chain(domain).collect()
        } else {
            let labels = self.labels.iter().flat_map(|x| x.to_bytes().to_vec());
            let domain = <AWDLDnsCompression as Into<u16>>::into(self.domain).to_be_bytes();
            labels.chain(domain).collect()
        }
    }
}
impl ToString for AWDLDnsName {
    fn to_string(&self) -> String {
        self.labels
            .iter()
            .fold(String::new(), |acc, x| acc + x + ".")
            + self.domain.to_string()
    }
}
#[cfg(feature = "debug")]
impl Debug for AWDLDnsName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_string())
    }
}
#[cfg(test)]
#[test]
fn test_dns_name() {
    use alloc::vec;
    let bytes = vec![0x04, 0x61, 0x77, 0x64, 0x6C, 0xc0, 0x0c];
    let dns_name = <AWDLDnsName as Read>::from_bytes(&mut bytes.iter().copied()).unwrap();
    assert_eq!(
        dns_name,
        AWDLDnsName {
            labels: vec!["awdl".into()],
            domain: AWDLDnsCompression::Local
        }
    );
    let dns_name_bytes = dns_name.to_bytes();
    assert_eq!(bytes, dns_name_bytes);
    let bytes = vec![
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
    assert_eq!(bytes, dns_name_bytes);
}
