use super::TLVType;
use crate::{common::awdl_version::AWDLVersion, impl_tlv_conversion_fixed};
use bin_utils::*;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Default, Clone, Copy, PartialEq, Eq)]
/// The device class of the peer.
pub enum AWDLDeviceClass {
    /// A macOS X device.
    #[default]
    MacOS,

    /// A iOS or watchOS device.
    IOSWatchOS,

    /// A tvOS device.
    TVOS,

    /// A device of unknown type.
    Unknown(u8),
}
enum_to_int! {
    u8,
    AWDLDeviceClass,

    0x01,
    AWDLDeviceClass::MacOS,
    0x02,
    AWDLDeviceClass::IOSWatchOS,
    0x08,
    AWDLDeviceClass::TVOS
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
impl_tlv_conversion_fixed!(VersionTLV, TLVType::Version, 2);

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
    use crate::tlvs::AWDLTLV;

    use super::TLV;

    let bytes = include_bytes!("../../../test_bins/version_tlv.bin");

    let tlv = TLV::from_bytes(&mut bytes.iter().copied()).unwrap();

    let version_tlv = VersionTLV::try_from(tlv.clone()).unwrap();
    assert_eq!(tlv, <VersionTLV as Into<AWDLTLV>>::into(version_tlv));

    assert_eq!(
        version_tlv,
        VersionTLV {
            version: AWDLVersion { major: 3, minor: 4 },
            device_class: AWDLDeviceClass::MacOS,
        }
    );
    assert_eq!(version_tlv.to_bytes(), bytes[3..]);
}
