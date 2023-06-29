#[cfg(feature = "data_tlvs")]
#[doc(cfg(feature = "data_tlvs"))]
/// TLVs regarding the data path.
pub mod data;
#[cfg(feature = "dns_sd_tlvs")]
#[doc(cfg(feature = "dns_sd_tlvs"))]
/// TLVs containing data about dns services.
pub mod dns_sd;
#[cfg(feature = "sync_elect_tlvs")]
#[doc(cfg(feature = "sync_elect_tlvs"))]
/// TLVs about the synchronization and election state of the peer.
pub mod sync_elect;
#[cfg(feature = "version_tlv")]
#[doc(cfg(feature = "version_tlv"))]
pub mod version;

use bin_utils::*;
use tlv_rs::TLV;

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
pub type AWDLTLV<'a> = TLV<'a, TLVType>;
#[cfg(feature = "read")]
#[derive(Debug)]
/// Errors that can occur while converting an AWDLTLV to a specific type.
pub enum FromTLVError {
    IncorrectTlvType,
    IncorrectTlvLength,
    NoData,
    ParserError(ParserError),
}
#[macro_export]
#[doc(hidden)]
macro_rules! impl_tlv_conversion {
    (true, $ntype:ty, $tlv_type:expr, $tlv_length:expr) => {
        #[cfg(feature = "write")]
        impl From<$ntype> for $crate::tlvs::AWDLTLV<'_> {
            fn from(value: $ntype) -> Self {
                Self {
                    tlv_type: $tlv_type,
                    tlv_data: value.to_bytes().to_vec().into(),
                }
            }
        }
        #[cfg(feature = "read")]
        impl TryFrom<$crate::tlvs::AWDLTLV<'_>> for $ntype {
            type Error = $crate::tlvs::FromTLVError;
            fn try_from(value: $crate::tlvs::AWDLTLV<'_>) -> Result<Self, Self::Error> {
                if value.tlv_data.len() != $tlv_length {
                    return Err($crate::tlvs::FromTLVError::IncorrectTlvLength);
                }
                if value.tlv_type != $tlv_type {
                    return Err($crate::tlvs::FromTLVError::IncorrectTlvType);
                }
                Self::from_bytes(&value.tlv_data.into_iter().copied().next_chunk().unwrap())
                    .map_err($crate::tlvs::FromTLVError::ParserError)
            }
        }
    };
    (false, $ntype:ident <$lt:lifetime>, $tlv_type:expr, $tlv_length:expr) => {
        #[cfg(feature = "write")]
        impl<'a> From<$ntype<$lt>> for $crate::tlvs::AWDLTLV<'a> {
            fn from(value: $ntype) -> Self {
                Self {
                    tlv_type: $tlv_type,
                    tlv_data: value.to_bytes().to_vec().into(),
                }
            }
        }
        #[cfg(feature = "read")]
        impl<'a> TryFrom<$crate::tlvs::AWDLTLV<'a>> for $ntype<$lt> {
            type Error = $crate::tlvs::FromTLVError;
            fn try_from(value: $crate::tlvs::AWDLTLV<'a>) -> Result<Self, Self::Error> {
                if value.tlv_data.len() < $tlv_length {
                    return Err($crate::tlvs::FromTLVError::IncorrectTlvLength);
                }
                if value.tlv_type != $tlv_type {
                    return Err($crate::tlvs::FromTLVError::IncorrectTlvType);
                }
                Self::from_bytes(&mut value.tlv_data.into_iter().copied())
                    .map_err($crate::tlvs::FromTLVError::ParserError)
            }
        }
    };
    (false, $ntype:ident, $tlv_type:expr, 0) => {
        #[cfg(feature = "write")]
        impl<'a> From<$ntype> for $crate::tlvs::AWDLTLV<'a> {
            fn from(value: $ntype) -> Self {
                Self {
                    tlv_type: $tlv_type,
                    tlv_data: value.to_bytes().to_vec().into(),
                }
            }
        }
        #[cfg(feature = "read")]
        impl<'a> TryFrom<$crate::tlvs::AWDLTLV<'a>> for $ntype {
            type Error = $crate::tlvs::FromTLVError;
            fn try_from(value: $crate::tlvs::AWDLTLV<'a>) -> Result<Self, Self::Error> {
                if value.tlv_type != $tlv_type {
                    return Err($crate::tlvs::FromTLVError::IncorrectTlvType);
                }
                Self::from_bytes(&mut value.tlv_data.into_iter().copied())
                    .map_err($crate::tlvs::FromTLVError::ParserError)
            }
        }
    };
    (false, $ntype:ident, $tlv_type:expr, $tlv_length:expr) => {
        #[cfg(feature = "write")]
        impl<'a> From<$ntype> for $crate::tlvs::AWDLTLV<'a> {
            fn from(value: $ntype) -> Self {
                Self {
                    tlv_type: $tlv_type,
                    tlv_data: value.to_bytes().to_vec().into(),
                }
            }
        }
        #[cfg(feature = "read")]
        impl<'a> TryFrom<$crate::tlvs::AWDLTLV<'a>> for $ntype {
            type Error = $crate::tlvs::FromTLVError;
            fn try_from(value: $crate::tlvs::AWDLTLV<'a>) -> Result<Self, Self::Error> {
                if value.tlv_data.len() < $tlv_length {
                    return Err($crate::tlvs::FromTLVError::IncorrectTlvLength);
                }
                if value.tlv_type != $tlv_type {
                    return Err($crate::tlvs::FromTLVError::IncorrectTlvType);
                }
                Self::from_bytes(&mut value.tlv_data.into_iter().copied())
                    .map_err($crate::tlvs::FromTLVError::ParserError)
            }
        }
    };
}
