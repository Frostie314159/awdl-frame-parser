use bin_utils::*;

use crate::common::awdl_dns_name::AWDLDnsName;
#[cfg(feature = "read")]
use crate::tlvs::FromTLVError;
use crate::tlvs::{TLVType, AWDLTLV};
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
        if data.len() < 4 {
            return Err(ParserError::TooLittleData(data.len() - 4));
        }
        let flags = data.next().unwrap();
        let arpa = AWDLDnsName::from_bytes(data)?;
        Ok(Self { flags, arpa })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for ArpaTLV<'a> {
    fn to_bytes(&self) -> Cow<'a, [u8]> {
        [self.flags]
            .iter()
            .chain(self.arpa.to_bytes().iter())
            .copied()
            .collect()
    }
}
#[cfg(feature = "write")]
impl<'a> From<ArpaTLV<'a>> for AWDLTLV<'a> {
    fn from(value: ArpaTLV<'a>) -> Self {
        Self {
            tlv_type: TLVType::Arpa,
            tlv_data: value.to_bytes(),
        }
    }
}
#[cfg(feature = "read")]
impl<'a> TryFrom<AWDLTLV<'a>> for ArpaTLV<'a> {
    type Error = FromTLVError;
    fn try_from(value: AWDLTLV<'a>) -> Result<Self, Self::Error> {
        if value.tlv_data.len() < 4 {
            return Err(FromTLVError::IncorrectTlvLength);
        }
        if value.tlv_type != TLVType::Arpa {
            return Err(FromTLVError::IncorrectTlvType);
        }
        Self::from_bytes(&mut value.tlv_data.iter().copied()).map_err(FromTLVError::ParserError)
    }
}
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
