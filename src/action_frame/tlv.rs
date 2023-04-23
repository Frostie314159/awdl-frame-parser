#[cfg(feature = "read")]
use self::{arpa::ArpaTLV, version::VersionTLV};
use deku::prelude::*;

#[cfg(all(not(feature = "std"), feature = "read"))]
use alloc::format;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg_attr(feature = "read", derive(DekuRead))]
#[cfg_attr(feature = "write", derive(DekuWrite))]
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
#[deku(type = "u8")]
/// The type of the TLV.
pub enum TLVType {
    /// The service parameters.
    #[deku(id = "0x02")]
    ServiceResponse,

    /// The synchronization parameters.
    #[deku(id = "0x04")]
    SynchronizationParameters,

    /// The election parameters.
    #[deku(id = "0x05")]
    ElectionParameters,

    /// The service parameters.
    #[deku(id = "0x06")]
    ServiceParameters,

    /// The HT capabilities.
    #[deku(id = "0x07")]
    HTCapabilities,

    /// The data path state.
    #[deku(id = "0x0C")]
    DataPathState,

    /// The hostname of the peer.
    #[deku(id = "0x10")]
    Arpa,

    /// The VHT capabilities.
    #[deku(id = "0x11")]
    VHTCapabilities,

    /// The channel sequence.
    #[deku(id = "0x12")]
    ChannelSequence,
    
    /// The synchronization tree.
    #[deku(id = "0x14")]
    SynchronizationTree,

    /// The actual version of the AWDL protocol, that's being used.
    #[deku(id = "0x15")]
    Version,

    /// The V2 Election Parameters.
    #[deku(id = "0x18")]
    ElectionParametersV2,

    /// Any TLV type that's unknown to the parser.
    #[deku(id_pat = "_")]
    Unknown(u8),
}

#[cfg(feature = "read")]
macro_rules! as_tlv {
    ($fn_name:ident, $type_name:ty) => {
        pub fn $fn_name(&self) -> Option<$type_name> {
            Some(<$type_name>::try_from(self.tlv_data.as_ref()).unwrap())
        }
    };
}

#[cfg_attr(feature = "read", derive(DekuRead))]
#[cfg_attr(feature = "write", derive(DekuWrite))]
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
/// A **T**ype **L**ength **V**alue structure.
pub struct TLV {
    /// The type.
    pub tlv_type: TLVType,

    /// The length.
    pub tlv_length: u16,

    /// The data contained within the TLV.
    #[deku(count = "tlv_length")]
    pub tlv_data: Vec<u8>,
}
#[cfg(feature = "read")]
impl TLV {
    as_tlv! {as_version, VersionTLV}
    as_tlv! {as_arpa, ArpaTLV}
}

pub mod version {
    use deku::prelude::*;

    use crate::action_frame::version::AWDLVersion;

    #[cfg(all(not(feature = "std"), feature = "read"))]
    use alloc::format;
    #[cfg(all(not(feature = "std"), feature = "write"))]
    use alloc::vec::Vec;

    #[cfg_attr(feature = "read", derive(DekuRead))]
    #[cfg_attr(feature = "write", derive(DekuWrite))]
    #[cfg_attr(feature = "std", derive(Debug))]
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    #[deku(type = "u8")]
    /// The device class of the peer.
    pub enum AWDLDeviceClass {
        /// A macOS X device.
        #[default]
        #[deku(id = "0x01")]
        MacOS,
        
        /// A iOS or watchOS device.
        #[deku(id = "0x02")]
        IOSWatchOS,

        /// A tvOS device.
        #[deku(id = "0x03")]
        TVOS,

        /// A device of unknown type.
        #[deku(id_pat = "_")]
        Unknown(u8),
    }

    #[cfg_attr(feature = "read", derive(DekuRead))]
    #[cfg_attr(feature = "write", derive(DekuWrite))]
    #[cfg_attr(feature = "std", derive(Debug))]
    #[derive(Clone, Copy, PartialEq, Eq)]
    /// A TLV containing the actual version of the AWDL protocol.
    pub struct VersionTLV {
        ///  The AWDL protocol version.
        pub version: AWDLVersion,

        /// The device class.
        pub device_class: AWDLDeviceClass,
    }
}
pub mod arpa {
    #[cfg(not(feature = "std"))]
    use alloc::string::String;
    use deku::{bitvec::Msb0, prelude::*};

    use crate::action_frame::{dns_compression::AWDLDnsCompression, util};
    #[cfg(all(not(feature = "std"), feature = "read"))]
    use alloc::format;
    #[cfg(all(not(feature = "std"), feature = "write"))]
    use alloc::vec::Vec;
    #[cfg(all(not(feature = "std"), feature = "read"))]
    use alloc::{format, string::ToString};

    #[cfg(feature = "read")]
    use deku::{bitvec::BitSlice, ctx::Endian};

    #[cfg(feature = "write")]
    use deku::bitvec::BitVec;

    #[cfg(feature = "read")]
    fn read_string(
        rest: &BitSlice<u8, Msb0>,
        len: usize,
    ) -> Result<(&BitSlice<u8, Msb0>, String), DekuError> {
        let (rest, string) = Vec::<u8>::read(&rest, (len.into(), Endian::Little))?;
        Ok((rest, String::from_utf8_lossy(string.as_ref()).to_string()))
    }
    #[cfg(feature = "write")]
    fn write_string(output: &mut BitVec<u8, Msb0>, string: &String) -> Result<(), DekuError> {
        string.as_bytes().write(output, ())
    }

    #[cfg_attr(feature = "read", derive(DekuRead))]
    #[cfg_attr(feature = "write", derive(DekuWrite))]
    #[cfg_attr(feature = "std", derive(Debug))]
    #[derive(Clone, PartialEq, Eq)]
    /// A hostname combined with the [domain](AWDLDnsCompression).
    pub struct Hostname {
        /// An unknown random prefix byte before the host.
        pub unknown: u8,

        #[deku(
            reader = "read_string(deku::rest, (deku::rest.len() / 8) - 2)",
            writer = "write_string(deku::output, &self.host)"
        )]
        /// The hostname of the peer.
        pub host: String,

        /// The domain in [compressed form](AWDLDnsCompression).
        pub domain: AWDLDnsCompression,
    }

    #[cfg_attr(feature = "read", derive(DekuRead))]
    #[cfg_attr(feature = "write", derive(DekuWrite))]
    #[cfg_attr(feature = "std", derive(Debug))]
    #[derive(Clone, PartialEq, Eq)]
    /// A TLV containing the hostname of the peer. Used for reverse DNS.
    pub struct ArpaTLV {
        /// A currently unknown flags header.
        pub flags: u8,

        /// The actual arpa data.
        pub arpa: Hostname,
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    use alloc::vec;

    use deku::DekuContainerWrite;

    use crate::action_frame::tlv::{TLVType, TLV};

    use super::{version::VersionTLV, ArpaTLV};

    macro_rules! test_tlv {
        ($type_name:ty, $tlv_type:expr, $test_name:ident, $bytes:expr) => {
    #[test]
            fn $test_name() {
                let bytes = $bytes;
                let tlv = <$type_name>::try_from(bytes.as_ref()).unwrap();
                assert_eq!(tlv.to_bytes().unwrap(), bytes);
                assert_eq!(
                    <$type_name as Into<TLV>>::into(tlv),
                    TLV {
                        tlv_type: $tlv_type,
                        tlv_length: bytes.len() as u16,
                        tlv_data: bytes,
    }
                );
    }
        };
    }

    test_tlv!(
        ArpaTLV,
        TLVType::Arpa,
        test_arpa,
        vec![0x03, 0x0f, 0x62, 0x6d, 0x2d, 0x63, 0x33, 0x33, 0x2F, 0x61, 0xc0, 0x0c,]
    );
    test_tlv!(VersionTLV, TLVType::Version, test_tlv, vec![0x10, 0x03]);
}
