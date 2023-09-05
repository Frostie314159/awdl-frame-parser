use bin_utils::*;

use alloc::vec::Vec;
#[cfg(feature = "read")]
use try_take::try_take;

use crate::common::{AWDLDnsName, AWDLStr};

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
/// A DNS record as encoded by AWDL.
pub enum AWDLDnsRecord {
    /// Service
    SRV {
        priority: u16,
        weight: u16,
        port: u16,
        target: AWDLDnsName,
    },
    /// Pointer
    PTR { domain_name: AWDLDnsName },
    /// Text
    TXT { txt_record: Vec<AWDLStr> },
}
impl AWDLDnsRecord {
    #[inline]
    /// Returns the [record type](AWDLDnsRecordType).
    pub const fn record_type(&self) -> AWDLDnsRecordType {
        match self {
            AWDLDnsRecord::PTR { .. } => AWDLDnsRecordType::PTR,
            AWDLDnsRecord::SRV { .. } => AWDLDnsRecordType::SRV,
            AWDLDnsRecord::TXT { .. } => AWDLDnsRecordType::TXT,
        }
    }
}
#[cfg(feature = "read")]
impl Read for AWDLDnsRecord {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let mut header = try_take(data, 5).map_err(ParserError::TooLittleData)?;
        let record_type = header.next().unwrap().into();
        let length = u16::from_le_bytes(header.next_chunk().unwrap());
        let _ = header.next_chunk::<2>();
        let mut data = try_take(data, length as usize).map_err(ParserError::TooLittleData)?;
        Ok(match record_type {
            AWDLDnsRecordType::PTR => AWDLDnsRecord::PTR {
                domain_name: AWDLDnsName::from_bytes(&mut data)?,
            },
            AWDLDnsRecordType::SRV => {
                let mut header = try_take(&mut data, 6).map_err(ParserError::TooLittleData)?;
                AWDLDnsRecord::SRV {
                    priority: u16::from_be_bytes(header.next_chunk().unwrap()),
                    weight: u16::from_be_bytes(header.next_chunk().unwrap()),
                    port: u16::from_be_bytes(header.next_chunk().unwrap()),
                    target: AWDLDnsName::from_bytes(&mut data)?,
                }
            }
            AWDLDnsRecordType::TXT => Self::TXT {
                txt_record: (0..)
                    .map_while(|_| AWDLStr::from_bytes(&mut data).ok())
                    .collect(),
            },
            AWDLDnsRecordType::Unknown(_) => return Err(ParserError::ValueNotUnderstood),
        })
    }
}
#[cfg(feature = "write")]
impl Write for AWDLDnsRecord {
    fn to_bytes(&self) -> alloc::vec::Vec<u8> {
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
                let mut static_bytes = [0x00; 6];
                static_bytes[0..2].copy_from_slice(&priority.to_be_bytes());
                static_bytes[2..4].copy_from_slice(&weight.to_be_bytes());
                static_bytes[4..6].copy_from_slice(&port.to_be_bytes());

                static_bytes.into_iter().chain(target.iter()).collect()
            }
            AWDLDnsRecord::TXT { txt_record } => {
                txt_record.iter().flat_map(AWDLStr::iter).collect()
            }
        };
        header[1..3].copy_from_slice(&(bytes.len() as u16).to_le_bytes());
        header.into_iter().chain(bytes).collect()
    }
}
