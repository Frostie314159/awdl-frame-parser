#[cfg(feature = "dns_sd_tlvs")]
pub mod dns_sd;
#[cfg(feature = "sync_elect_tlvs")]
pub mod sync_elect;
#[cfg(feature = "version_tlv")]
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
        impl From<$ntype> for $crate::tlvs::AWDLTLV<'_> {
            fn from(value: $ntype) -> Self {
                use alloc::borrow::ToOwned;
                Self {
                    tlv_type: $tlv_type,
                    tlv_data: alloc::borrow::Cow::Owned(value.to_bytes().as_slice().to_owned()),
                }
            }
        }

        #[cfg(feature = "read")]
        impl TryFrom<$crate::tlvs::AWDLTLV<'_>> for $ntype {
            type Error = $crate::tlvs::FromTLVError;
            fn try_from(value: $crate::tlvs::AWDLTLV<'_>) -> Result<Self, Self::Error> {
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