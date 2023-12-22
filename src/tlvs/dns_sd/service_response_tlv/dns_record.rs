use core::iter::repeat;

use alloc::vec::Vec;
use macro_bits::serializable_enum;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Pread, Pwrite, NETWORK,
};

use crate::common::{AWDLDnsName, AWDLStr};

serializable_enum! {
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
    pub enum AWDLDnsRecordType: u8 {
        #[default]
        PTR => 0xc,
        TXT => 0x10,
        SRV => 0x21
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A DNS record as encoded by AWDL.
pub enum AWDLDnsRecord<'a> {
    /// Pointer
    PTR {
        domain_name: AWDLDnsName<'a>,
    },
    /// Text
    TXT {
        txt_record: Vec<AWDLStr<'a>>,
    },
    /// Service
    SRV {
        priority: u16,
        weight: u16,
        port: u16,
        target: AWDLDnsName<'a>,
    },
    UnknownRecord {
        record_type: u8,
        body: &'a [u8],
    },
}
impl<'a> AWDLDnsRecord<'a> {
    #[inline]
    /// Returns the [record type](AWDLDnsRecordType).
    pub const fn record_type(&self) -> AWDLDnsRecordType {
        match self {
            AWDLDnsRecord::PTR { .. } => AWDLDnsRecordType::PTR,
            AWDLDnsRecord::TXT { .. } => AWDLDnsRecordType::TXT,
            AWDLDnsRecord::SRV { .. } => AWDLDnsRecordType::SRV,
            AWDLDnsRecord::UnknownRecord { record_type, .. } => {
                AWDLDnsRecordType::Unknown(*record_type)
            }
        }
    }
}
impl<'a> MeasureWith<()> for AWDLDnsRecord<'a> {
    fn measure_with(&self, ctx: &()) -> usize {
        (match self {
            AWDLDnsRecord::PTR { domain_name } => domain_name.measure_with(ctx),
            AWDLDnsRecord::TXT { txt_record } => txt_record.iter().map(|x| x.size_in_bytes()).sum(),
            AWDLDnsRecord::SRV { target, .. } => target.measure_with(ctx) + 6,
            AWDLDnsRecord::UnknownRecord { body, .. } => body.len(),
        }) + 1
    }
}
impl<'a> TryFromCtx<'a> for AWDLDnsRecord<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        let record_type = AWDLDnsRecordType::from_representation(from.gread(&mut offset)?);
        offset += 4; // Skip length and unknown, because it's irrelevant for us.
        Ok((
            match record_type {
                AWDLDnsRecordType::PTR => Self::PTR {
                    domain_name: from.gread(&mut offset)?,
                },
                AWDLDnsRecordType::TXT => Self::TXT {
                    txt_record: repeat(())
                        .map_while(|_| from.gread(&mut offset).ok())
                        .collect(),
                },
                AWDLDnsRecordType::SRV => Self::SRV {
                    priority: from.gread_with(&mut offset, NETWORK)?,
                    weight: from.gread_with(&mut offset, NETWORK)?,
                    port: from.gread_with(&mut offset, NETWORK)?,
                    target: from.gread(&mut offset)?,
                },
                AWDLDnsRecordType::Unknown(record_type) => Self::UnknownRecord {
                    record_type,
                    body: &from[offset..],
                },
            },
            offset,
        ))
    }
}
impl<'a> TryIntoCtx for AWDLDnsRecord<'a> {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        buf.gwrite(self.record_type().to_representation(), &mut offset)?;
        offset += 4;
        // Length will be inserted at the end, avoiding an allocation. Offset will be 1.
        match self {
            AWDLDnsRecord::PTR { domain_name } => {
                let _ = buf.gwrite(domain_name, &mut offset)?;
            }
            AWDLDnsRecord::TXT { txt_record } => {
                for record in txt_record {
                    buf.gwrite(record, &mut offset)?;
                }
            }
            AWDLDnsRecord::SRV {
                priority,
                weight,
                port,
                target,
            } => {
                buf.gwrite_with(priority, &mut offset, NETWORK)?;
                buf.gwrite_with(weight, &mut offset, NETWORK)?;
                buf.gwrite_with(port, &mut offset, NETWORK)?;
                buf.gwrite(target, &mut offset)?;
            }
            AWDLDnsRecord::UnknownRecord { body, .. } => {
                buf.gwrite(body, &mut offset)?;
            }
        };
        buf.pwrite(offset as u16 - 5, 1)?; // Length
        Ok(offset)
    }
}
