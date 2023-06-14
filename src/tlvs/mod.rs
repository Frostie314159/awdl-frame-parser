#[cfg(feature = "dns_sd_tlvs")]
pub mod dns_sd;
#[cfg(feature = "sync_elect_tlvs")]
pub mod sync_elect;
#[cfg(feature = "version_tlv")]
pub mod version;

use bin_utils::*;

use alloc::borrow::Cow;

#[cfg(feature = "read")]
use core::cmp::Ordering;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
/// The type of the TLV.
pub enum TLVType {
    /// The service parameters.
    ServiceResponse,

    /// The synchronization parameters.
    SynchronizationParameters,

    /// The election parameters.
    ElectionParameters,

    /// The service parameters.
    ServiceParameters,

    /// The HT capabilities.
    HTCapabilities,

    /// The data path state.
    DataPathState,

    /// The hostname of the peer.
    Arpa,

    /// The VHT capabilities.
    VHTCapabilities,

    /// The channel sequence.
    ChannelSequence,

    /// The synchronization tree.
    SynchronizationTree,

    /// The actual version of the AWDL protocol, that's being used.
    Version,

    /// The V2 Election Parameters.
    ElectionParametersV2,

    Unknown(u8),
}
enum_to_int! {
    u8,
    TLVType,

    0x02,
    TLVType::ServiceResponse,
    0x04,
    TLVType::SynchronizationParameters,
    0x05,
    TLVType::ElectionParameters,
    0x06,
    TLVType::ServiceParameters,
    0x07,
    TLVType::HTCapabilities,
    0x0C,
    TLVType::DataPathState,
    0x10,
    TLVType::Arpa,
    0x11,
    TLVType::VHTCapabilities,
    0x12,
    TLVType::ChannelSequence,
    0x14,
    TLVType::SynchronizationTree,
    0x15,
    TLVType::Version,
    0x18,
    TLVType::ElectionParametersV2
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
/// A **T**ype **L**ength **V**alue structure.
pub struct TLV<'a> {
    /// The type.
    pub tlv_type: TLVType,

    /// The data contained within the TLV.
    pub tlv_data: Cow<'a, [u8]>,
}
#[cfg(feature = "read")]
impl Read for TLV<'_> {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        if data.len() < 3 {
            return Err(ParserError::TooLittleData(3 - data.len()));
        }

        let tlv_type = data.next().unwrap().into();
        let tlv_length = u16::from_le_bytes(data.next_chunk().unwrap());
        let tlv_data = match data.len().cmp(&(tlv_length as usize)) {
            Ordering::Less => {
                return Err(ParserError::TooLittleData(tlv_length as usize - data.len()))
            }
            _ => Cow::Owned(data.take(tlv_length as usize).collect()),
        };

        Ok(Self { tlv_type, tlv_data })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for TLV<'a> {
    fn to_bytes(&self) -> Cow<'a, [u8]> {
        let tlv_length = self.tlv_data.len().to_le_bytes();
        let tlv_header = [self.tlv_type.into(), tlv_length[0], tlv_length[1]];
        tlv_header
            .into_iter()
            .chain(self.tlv_data.iter().copied())
            .collect()
    }
}
#[cfg(test)]
#[test]
fn test_tlv() {
    use alloc::borrow::ToOwned;
    let bytes = &[0x04, 0x05, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff];
    let tlv = TLV::from_bytes(&mut bytes.iter().copied()).unwrap();
    assert_eq!(
        tlv,
        TLV {
            tlv_type: TLVType::SynchronizationParameters,
            tlv_data: Cow::Owned([0xff; 5].as_slice().to_owned())
        }
    );
    assert_eq!(tlv.to_bytes(), bytes.as_slice().to_owned());
}
#[cfg(feature = "read")]
#[derive(Debug)]
pub enum FromTLVError {
    IncorrectTlvType,
    IncorrectTlvLength,
    NoData,
    ParserError(ParserError),
}
#[macro_export]
macro_rules! impl_tlv_conversion_fixed {
    ($ntype:ty, $tlv_type:expr, $tlv_length:expr) => {
        #[cfg(feature = "write")]
        impl From<$ntype> for $crate::tlvs::TLV<'_> {
            fn from(value: $ntype) -> Self {
                use alloc::borrow::ToOwned;
                Self {
                    tlv_type: $tlv_type,
                    tlv_data: alloc::borrow::Cow::Owned(value.to_bytes().as_slice().to_owned()),
                }
            }
        }

        #[cfg(feature = "read")]
        impl TryFrom<$crate::tlvs::TLV<'_>> for $ntype {
            type Error = $crate::tlvs::FromTLVError;
            fn try_from(value: $crate::tlvs::TLV<'_>) -> Result<Self, Self::Error> {
                if value.tlv_data.len() < $tlv_length {
                    return Err($crate::tlvs::FromTLVError::IncorrectTlvLength);
                }
                if value.tlv_type != $tlv_type {
                    return Err($crate::tlvs::FromTLVError::IncorrectTlvType);
                }
                Self::from_bytes(&value.tlv_data.iter().map(|x| *x).next_chunk().unwrap())
                    .map_err($crate::tlvs::FromTLVError::ParserError)
            }
        }
    };
}
