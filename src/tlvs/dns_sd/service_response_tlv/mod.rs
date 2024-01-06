pub mod dns_record;

use crate::common::{AWDLDnsName, AWDLStr, ReadLabelIterator};

use dns_record::AWDLDnsRecord;

use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

#[derive(Clone, Debug, Hash)]
/// This TLV contains data about services offered by the peer.
pub struct ServiceResponseTLV<'a, I = ReadLabelIterator<'a>>
where
    I: IntoIterator<Item = AWDLStr<'a>>,
{
    /// The fullname of the service.
    pub name: AWDLDnsName<'a, I>,

    /// The DNS record contained in this response.
    pub record: AWDLDnsRecord<'a, I>,
}
impl<'a, I: IntoIterator<Item = AWDLStr<'a>> + Copy> Copy for ServiceResponseTLV<'a, I> {}
impl<'a, I: IntoIterator<Item = AWDLStr<'a>> + Clone> Eq for ServiceResponseTLV<'a, I> {}
impl<'a, LhsIterator, RhsIterator> PartialEq<ServiceResponseTLV<'a, RhsIterator>>
    for ServiceResponseTLV<'a, LhsIterator>
where
    LhsIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    RhsIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
{
    fn eq(&self, other: &ServiceResponseTLV<'a, RhsIterator>) -> bool {
        self.name == other.name && self.record == other.record
    }
}
impl<'a, I> MeasureWith<()> for ServiceResponseTLV<'a, I>
where
    I: IntoIterator<Item = AWDLStr<'a>> + Clone,
{
    fn measure_with(&self, ctx: &()) -> usize {
        6 + self.name.measure_with(ctx) + self.record.measure_with(ctx)
    }
}
impl<'a> TryFromCtx<'a> for ServiceResponseTLV<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;

        let length = from.gread_with::<u16>(&mut offset, Endian::Little)? as usize;
        let name = from
            .gread_with::<&'a [u8]>(&mut offset, length - 1)?
            .pread(0)?;
        let record = from.gread(&mut offset)?;
        Ok((Self { name, record }, offset))
    }
}
impl<'a, I> TryIntoCtx for ServiceResponseTLV<'a, I>
where
    I: IntoIterator<Item = AWDLStr<'a>> + Clone,
{
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        buf.gwrite_with::<u16>(
            self.name.measure_with(&()) as u16 + 1,
            &mut offset,
            Endian::Little,
        )?;
        buf.gwrite(self.name, &mut offset)?;
        buf.gwrite(self.record, &mut offset)?;
        Ok(offset)
    }
}
//impl_tlv_conversion!(false, ServiceResponseTLV, TLVType::ServiceResponse, 9);
#[cfg(test)]
mod service_response_tests {
    use alloc::vec;
    use scroll::{ctx::MeasureWith, Pread, Pwrite};

    use crate::{
        common::{AWDLDnsCompression, AWDLDnsName, ReadLabelIterator},
        tlvs::dns_sd::{dns_record::AWDLDnsRecord, ServiceResponseTLV},
    };

    #[test]
    fn test_service_response_tlv_ptr() {
        let bytes = &include_bytes!("../../../../test_bins/service_response_tlv_ptr.bin")[3..];

        let service_response_tlv = bytes
            .pread::<ServiceResponseTLV<ReadLabelIterator>>(0)
            .unwrap();

        assert_eq!(
            service_response_tlv,
            ServiceResponseTLV {
                name: AWDLDnsName {
                    labels: vec!["_airplay-p2p".into()],
                    domain: AWDLDnsCompression::TcpLocal,
                    ..Default::default()
                },
                record: AWDLDnsRecord::PTR {
                    domain_name: AWDLDnsName {
                        labels: vec!["34FD6A0C9A42@1.021".into()],
                        domain: AWDLDnsCompression::Null,
                        ..Default::default()
                    }
                }
            }
        );
        let mut buf = vec![0x00; service_response_tlv.measure_with(&())];
        buf.as_mut_slice().pwrite(service_response_tlv, 0).unwrap();
        assert_eq!(buf, bytes);
    }
    #[test]
    fn test_service_response_tlv_srv() {
        let bytes = &include_bytes!("../../../../test_bins/service_response_tlv_srv.bin")[3..];

        let service_response_tlv = bytes
            .pread::<ServiceResponseTLV<ReadLabelIterator>>(0)
            .unwrap();

        assert_eq!(
            service_response_tlv,
            ServiceResponseTLV {
                name: AWDLDnsName {
                    labels: vec!["34fd6a0c9a42@1.021".into(), "_airplay-p2p".into()],
                    domain: AWDLDnsCompression::TcpLocal,
                    ..Default::default()
                },
                record: AWDLDnsRecord::SRV {
                    priority: 0,
                    weight: 0,
                    port: 7000,
                    target: AWDLDnsName {
                        labels: vec!["dcc83dc2-fae7-4043-8c7a-a8b6bf49eaad".into()],
                        domain: AWDLDnsCompression::Local,
                        ..Default::default()
                    }
                }
            }
        );
        let mut buf = vec![0x00; service_response_tlv.measure_with(&())];
        buf.as_mut_slice().pwrite(service_response_tlv, 0).unwrap();
        assert_eq!(buf, bytes);
    }
    #[test]
    fn test_service_response_tlv_txt() {
        let bytes = &include_bytes!("../../../../test_bins/service_response_tlv_txt.bin")[3..];

        let service_response_tlv = bytes
            .pread::<ServiceResponseTLV<ReadLabelIterator>>(0)
            .unwrap();

        assert_eq!(
            service_response_tlv,
            ServiceResponseTLV {
                name: AWDLDnsName {
                    labels: vec!["6dba48462242".into()],
                    domain: AWDLDnsCompression::AirDropTcpLocal,
                    ..Default::default()
                },
                record: AWDLDnsRecord::TXT {
                    txt_record: alloc::vec!["flags=999".into()]
                }
            }
        );
        let mut buf = vec![0x00; service_response_tlv.measure_with(&())];
        buf.as_mut_slice().pwrite(service_response_tlv, 0).unwrap();
        assert_eq!(buf, bytes);
    }
}
