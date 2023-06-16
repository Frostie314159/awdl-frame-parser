use bin_utils::*;

#[cfg(feature = "write")]
use alloc::borrow::Cow;
use alloc::vec::Vec;

use crate::common::{awdl_dns_name::AWDLDnsName, awdl_str::AWDLStr};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AWDLDnsRecordType {
    PTR,
    TXT,
    SRV,
    Unknown(u8),
}
enum_to_int! {
    u8,
    AWDLDnsRecordType,

    12,
    AWDLDnsRecordType::PTR,
    16,
    AWDLDnsRecordType::TXT,
    33,
    AWDLDnsRecordType::SRV
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub enum AWDLDnsRecord<'a> {
    SRV {
        priority: u16,
        weight: u16,
        port: u16,
        target: AWDLDnsName<'a>,
    },
    PTR {
        domain_name: AWDLDnsName<'a>,
    },
    TXT {
        txt_record: Vec<AWDLStr<'a>>,
    },
}
impl AWDLDnsRecord<'_> {
    pub fn record_type(&self) -> AWDLDnsRecordType {
        match self {
            AWDLDnsRecord::PTR { .. } => AWDLDnsRecordType::PTR,
            AWDLDnsRecord::SRV { .. } => AWDLDnsRecordType::SRV,
            AWDLDnsRecord::TXT { .. } => AWDLDnsRecordType::TXT,
        }
    }
}
#[cfg(feature = "read")]
impl Read for AWDLDnsRecord<'_> {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let mut header = data.take(5);
        let record_type = header.next().unwrap().into();
        let length = u16::from_le_bytes(header.next_chunk().unwrap());
        let _ = header.next_chunk::<2>();

        if data.len() < length as usize {
            return Err(ParserError::HeaderIncomplete(length as usize - data.len()));
        }
        Ok(match record_type {
            AWDLDnsRecordType::PTR => AWDLDnsRecord::PTR {
                domain_name: AWDLDnsName::from_bytes(data)?,
            },
            AWDLDnsRecordType::SRV => AWDLDnsRecord::SRV {
                priority: u16::from_be_bytes(data.next_chunk().unwrap()),
                weight: u16::from_be_bytes(data.next_chunk().unwrap()),
                port: u16::from_be_bytes(data.next_chunk().unwrap()),
                target: AWDLDnsName::from_bytes(data)?,
            },
            AWDLDnsRecordType::TXT => Self::TXT {
                txt_record: (0..)
                    .map_while(|_| AWDLStr::from_bytes(data).ok())
                    .collect(),
            },
            AWDLDnsRecordType::Unknown(_) => return Err(ParserError::ValueNotUnderstood),
        })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for AWDLDnsRecord<'a> {
    fn to_bytes(&self) -> Cow<'a, [u8]> {
        let mut header = [0x00; 5];
        header[0] = self.record_type().into();

        let bytes = match self {
            AWDLDnsRecord::PTR { domain_name } => domain_name.to_bytes(),
            AWDLDnsRecord::SRV {
                priority,
                weight,
                port,
                target,
            } => {
                let target = target.to_bytes();

                let mut static_bytes = [0x00; 6];
                static_bytes[0..2].copy_from_slice(&priority.to_be_bytes());
                static_bytes[2..4].copy_from_slice(&weight.to_be_bytes());
                static_bytes[4..6].copy_from_slice(&port.to_be_bytes());

                static_bytes.into_iter().chain(target.iter().copied()).collect()
            }
            AWDLDnsRecord::TXT { txt_record } => txt_record
                .iter()
                .map(AWDLStr::to_bytes)
                .collect::<Vec<Cow<[u8]>>>()
                .concat()
                .into(),
        };
        header[1..3].copy_from_slice(&(bytes.len() as u16).to_le_bytes());
        header.into_iter().chain(bytes.iter().copied()).collect()
    }
}