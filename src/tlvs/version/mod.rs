use crate::{
    common::AWDLVersion,
    tlvs::{impl_tlv_conversion, TLVType},
};
use bin_utils::*;
use macro_bits::serializable_enum;

serializable_enum! {
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Default, Clone, Copy, PartialEq, Eq)]
    /// The device class of the peer.
    pub enum AWDLDeviceClass: u8 {
        /// A macOS X device.
        #[default]
        MacOS => 0x1,

        /// A iOS device.
        IOS => 0x2,

        /// A watchOS device.
        WatchOS => 0x4,

        /// A tvOS device.
        TVOS => 0x8
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
/// A TLV containing the actual version of the AWDL protocol.
pub struct VersionTLV {
    ///  The AWDL protocol version.
    pub version: AWDLVersion,

    /// The device class.
    pub device_class: AWDLDeviceClass,
}
impl_tlv_conversion!(true, VersionTLV, TLVType::Version, 2);

#[cfg(feature = "read")]
impl ReadFixed<2> for VersionTLV {
    fn from_bytes(data: &[u8; 2]) -> Result<Self, ParserError> {
        Ok(Self {
            version: AWDLVersion::from(data[0]),
            device_class: data[1].into(),
        })
    }
}
#[cfg(feature = "write")]
impl WriteFixed<2> for VersionTLV {
    fn to_bytes(&self) -> [u8; 2] {
        [self.version.into(), self.device_class.into()]
    }
}

#[cfg(test)]
#[test]
fn test_version_tlv() {
    let bytes = [0x3e, 0x01];

    let version_tlv = VersionTLV::from_bytes(&bytes).unwrap();

    assert_eq!(
        version_tlv,
        VersionTLV {
            version: AWDLVersion {
                major: 3,
                minor: 14
            },
            device_class: AWDLDeviceClass::MacOS,
        }
    );
    assert_eq!(version_tlv.to_bytes(), bytes);
}
