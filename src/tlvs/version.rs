use crate::common::AWDLVersion;
use macro_bits::serializable_enum;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Pread, Pwrite,
};

use super::{AWDLTLVType, AwdlTlv};

serializable_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// A TLV containing the actual version of the AWDL protocol.
pub struct VersionTLV {
    ///  The AWDL protocol version.
    pub version: AWDLVersion,

    /// The device class.
    pub device_class: AWDLDeviceClass,
}
impl AwdlTlv for VersionTLV {
    const TLV_TYPE: AWDLTLVType = AWDLTLVType::Version;
}
impl MeasureWith<()> for VersionTLV {
    fn measure_with(&self, _ctx: &()) -> usize {
        2
    }
}
impl<'a> TryFromCtx<'a> for VersionTLV {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        let version = AWDLVersion::from_bits(from.gread(&mut offset)?);
        let device_class = AWDLDeviceClass::from_bits(from.gread(&mut offset)?);
        Ok((
            Self {
                version,
                device_class,
            },
            offset,
        ))
    }
}
impl TryIntoCtx for VersionTLV {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        buf.gwrite(self.version.into_bits(), &mut offset)?;
        buf.gwrite(self.device_class.into_bits(), &mut offset)?;
        Ok(offset)
    }
}
//impl_tlv_conversion!(true, VersionTLV, TLVType::Version, 2);

#[cfg(test)]
#[test]
fn test_version_tlv() {
    let bytes = [0x3e, 0x01];

    let version_tlv = bytes.pread::<VersionTLV>(0).unwrap();

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
    let mut buf = [0x00; 2];
    buf.pwrite(version_tlv, 0).unwrap();
    assert_eq!(buf, bytes);
}
