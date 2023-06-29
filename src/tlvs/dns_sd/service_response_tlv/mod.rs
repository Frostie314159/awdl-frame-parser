pub mod dns_record;

use crate::{common::AWDLDnsName, impl_tlv_conversion, tlvs::TLVType};

use dns_record::AWDLDnsRecord;

use bin_utils::*;

#[cfg(feature = "write")]
use alloc::borrow::Cow;
use try_take::try_take;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
/// This TLV contains data about services offered by the peer.
pub struct ServiceResponseTLV<'a> {
    /// The fullname of the service.
    pub name: AWDLDnsName<'a>,

    /// The DNS record contained in this response.
    pub record: AWDLDnsRecord<'a>,
}
#[cfg(feature = "read")]
impl Read for ServiceResponseTLV<'_> {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let length = u16::from_le_bytes(
            try_take(data, 2)
                .map_err(ParserError::TooLittleData)?
                .next_chunk()
                .unwrap(),
        );
        let name = AWDLDnsName::from_bytes(
            &mut try_take(
                data,
                length
                    .checked_sub(1)
                    .ok_or(ParserError::ValueNotUnderstood)? as usize,
            )
            .map_err(ParserError::TooLittleData)?,
        )?;
        let record = AWDLDnsRecord::from_bytes(data)?;
        Ok(Self { name, record })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for ServiceResponseTLV<'a> {
    fn to_bytes(&self) -> Cow<'a, [u8]> {
        let name_length = (self.name.len() as u16 + 1).to_le_bytes();
        let record = self.record.to_bytes();
        name_length
            .into_iter()
            .chain(self.name.iter())
            .chain(record.iter().copied())
            .collect()
    }
}
impl_tlv_conversion!(false, ServiceResponseTLV<'a>, TLVType::ServiceResponse, 9);
#[cfg(test)]
mod service_response_tests {
    use alloc::{borrow::Cow, vec};

    use bin_utils::*;

    use crate::{
        common::{AWDLDnsCompression, AWDLDnsName},
        tlvs::dns_sd::{dns_record::AWDLDnsRecord, ServiceResponseTLV},
    };

    #[test]
    fn test_service_response_tlv_ptr() {
        let bytes =
            include_bytes!("../../../../test_bins/service_response_tlv_ptr.bin")[3..].to_vec();

        let service_response_tlv =
            ServiceResponseTLV::from_bytes(&mut bytes.iter().copied()).unwrap();

        assert_eq!(
            service_response_tlv,
            ServiceResponseTLV {
                name: AWDLDnsName {
                    labels: vec!["_airplay-p2p".into()],
                    domain: AWDLDnsCompression::TcpLocal,
                },
                record: AWDLDnsRecord::PTR {
                    domain_name: AWDLDnsName {
                        labels: vec!["34FD6A0C9A42@1.021".into()],
                        domain: AWDLDnsCompression::Null,
                    }
                }
            }
        );

        assert_eq!(
            service_response_tlv.to_bytes(),
            <&[u8] as Into<Cow<[u8]>>>::into(bytes.as_slice())
        );
    }
    #[test]
    fn test_service_response_tlv_srv() {
        let bytes =
            include_bytes!("../../../../test_bins/service_response_tlv_srv.bin")[3..].to_vec();

        let service_response_tlv =
            ServiceResponseTLV::from_bytes(&mut bytes.iter().copied()).unwrap();

        assert_eq!(
            service_response_tlv,
            ServiceResponseTLV {
                name: AWDLDnsName {
                    labels: vec!["34fd6a0c9a42@1.021".into(), "_airplay-p2p".into()],
                    domain: AWDLDnsCompression::TcpLocal,
                },
                record: AWDLDnsRecord::SRV {
                    priority: 0,
                    weight: 0,
                    port: 7000,
                    target: AWDLDnsName {
                        labels: vec!["dcc83dc2-fae7-4043-8c7a-a8b6bf49eaad".into()],
                        domain: AWDLDnsCompression::Local,
                    }
                }
            }
        );
        assert_eq!(
            service_response_tlv.to_bytes(),
            <&[u8] as Into<Cow<[u8]>>>::into(bytes.as_slice())
        );
    }
    #[test]
    fn test_service_response_tlv_txt() {
        let bytes =
            include_bytes!("../../../../test_bins/service_response_tlv_txt.bin")[3..].to_vec();

        let service_response_tlv =
            ServiceResponseTLV::from_bytes(&mut bytes.iter().copied()).unwrap();

        assert_eq!(
            service_response_tlv,
            ServiceResponseTLV {
                name: AWDLDnsName {
                    labels: vec!["6dba48462242".into()],
                    domain: AWDLDnsCompression::AirDropTcpLocal,
                },
                record: AWDLDnsRecord::TXT {
                    txt_record: alloc::vec!["flags=999".into()]
                }
            }
        );
        assert_eq!(
            service_response_tlv.to_bytes(),
            <&[u8] as Into<Cow<[u8]>>>::into(bytes.as_slice())
        );
    }
}
