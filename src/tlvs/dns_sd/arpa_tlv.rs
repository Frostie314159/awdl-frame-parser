use bin_utils::*;

use crate::{
    common::AWDLDnsName,
    tlvs::{impl_tlv_conversion, TLVType},
};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, PartialEq, Eq)]
/// A TLV containing the hostname of the peer. Used for reverse DNS.
pub struct ArpaTLV {
    /// The actual arpa data.
    pub arpa: AWDLDnsName,
}
#[cfg(feature = "read")]
impl Read for ArpaTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let flags = data.next().ok_or(ParserError::TooLittleData(1))?; // Always 0x03.
        if flags != 0x03 {
            return Err(ParserError::InvalidMagic);
        }
        let arpa = AWDLDnsName::from_bytes(data)?;
        Ok(Self { arpa })
    }
}
#[cfg(feature = "write")]
impl Write for ArpaTLV {
    fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        core::iter::once(0x03).chain(self.arpa.to_bytes()).collect()
    }
}
impl_tlv_conversion!(false, ArpaTLV, TLVType::Arpa, 3);
#[cfg(test)]
#[test]
fn test_arpa_tlv() {
    use crate::common::AWDLDnsCompression;
    use alloc::{borrow::ToOwned, vec};

    let bytes = include_bytes!("../../../test_bins/arpa_tlv.bin")[3..].to_vec();

    let arpa_tlv = ArpaTLV::from_bytes(&mut bytes.iter().copied()).unwrap();
    assert_eq!(
        arpa_tlv,
        ArpaTLV {
            arpa: AWDLDnsName {
                labels: vec!["simon-framework".into()],
                domain: AWDLDnsCompression::Local
            }
        }
    );

    assert_eq!(arpa_tlv.to_bytes(), bytes.as_slice().to_owned());
}
