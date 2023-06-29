use bin_utils::*;

use crate::tlvs::TLVType;
use crate::{common::AWDLDnsName, impl_tlv_conversion};
#[cfg(feature = "write")]
use alloc::borrow::Cow;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, PartialEq, Eq)]
/// A TLV containing the hostname of the peer. Used for reverse DNS.
pub struct ArpaTLV<'a> {
    /// A currently unknown flags header.
    pub flags: u8,

    /// The actual arpa data.
    pub arpa: AWDLDnsName<'a>,
}
#[cfg(feature = "read")]
impl<'a> Read for ArpaTLV<'a> {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let flags = data.next().ok_or(ParserError::TooLittleData(1))?;
        let arpa = AWDLDnsName::from_bytes(data)?;
        Ok(Self { flags, arpa })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for ArpaTLV<'a> {
    fn to_bytes(&self) -> Cow<'a, [u8]> {
        core::iter::once(self.flags)
            .chain(self.arpa.to_bytes().iter().copied())
            .collect()
    }
}
impl_tlv_conversion!(false, ArpaTLV<'a>, TLVType::Arpa, 3);
#[cfg(test)]
#[test]
fn test_arpa_tlv() {
    use crate::common::awdl_dns_compression::AWDLDnsCompression;
    use alloc::{borrow::ToOwned, vec};

    let bytes = include_bytes!("../../../test_bins/arpa_tlv.bin")[3..].to_vec();

    let arpa_tlv = ArpaTLV::from_bytes(&mut bytes.iter().copied()).unwrap();
    assert_eq!(
        arpa_tlv,
        ArpaTLV {
            flags: 0x03,
            arpa: AWDLDnsName {
                labels: vec!["simon-framework".into()],
                domain: AWDLDnsCompression::Local
            }
        }
    );

    assert_eq!(arpa_tlv.to_bytes(), bytes.as_slice().to_owned());
}
