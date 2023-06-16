pub mod dns_record;

#[cfg(feature = "read")]
use crate::tlvs::FromTLVError;
use crate::{
    common::awdl_dns_name::AWDLDnsName,
    tlvs::{TLVType, AWDLTLV},
};

use dns_record::AWDLDnsRecord;

use bin_utils::*;

#[cfg(feature = "write")]
use alloc::borrow::Cow;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub struct ServiceResponseTLV<'a> {
    pub name: AWDLDnsName<'a>,

    pub record: AWDLDnsRecord<'a>,
}
#[cfg(feature = "read")]
impl Read for ServiceResponseTLV<'_> {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let length = u16::from_le_bytes(data.next_chunk::<2>().unwrap());
        let name = AWDLDnsName::from_bytes(&mut data.take(length as usize - 1))?;
        let record = AWDLDnsRecord::from_bytes(data)?;
        Ok(Self { name, record })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for ServiceResponseTLV<'a> {
    fn to_bytes(&self) -> Cow<'a, [u8]> {
        let name = self.name.to_bytes();
        let name_length = (name.len() as u16 + 1).to_le_bytes();
        let record = self.record.to_bytes();
        name_length
            .iter()
            .chain(name.iter().chain(record.iter()))
            .copied()
            .collect()
    }
}
#[cfg(feature = "write")]
impl<'a> From<ServiceResponseTLV<'a>> for AWDLTLV<'a> {
    fn from(value: ServiceResponseTLV<'a>) -> Self {
        Self {
            tlv_type: TLVType::ServiceResponse,
            tlv_data: value.to_bytes(),
        }
    }
}
#[cfg(feature = "read")]
impl<'a> TryFrom<AWDLTLV<'a>> for ServiceResponseTLV<'a> {
    type Error = FromTLVError;
    fn try_from(value: AWDLTLV<'a>) -> Result<Self, Self::Error> {
        if value.tlv_data.len() < 7 {
            return Err(FromTLVError::IncorrectTlvLength);
        }
        if value.tlv_type != TLVType::ServiceResponse {
            return Err(FromTLVError::IncorrectTlvType);
        }
        Self::from_bytes(&mut value.tlv_data.iter().copied()).map_err(FromTLVError::ParserError)
    }
}
#[cfg(test)]
mod service_response_tests {
    use alloc::{borrow::Cow, vec};

    use bin_utils::*;

    use crate::{
        common::{awdl_dns_compression::AWDLDnsCompression, awdl_dns_name::AWDLDnsName},
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
