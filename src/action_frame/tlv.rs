#[cfg(all(feature = "read", feature = "dns_sd_tlvs"))]
use self::{dns_sd::ArpaTLV, sync_elect::ChannelSequenceTLV};
#[cfg(all(feature = "read", feature = "version_tlv"))]
use self::version::VersionTLV;
use deku::prelude::*;

#[cfg(all(not(feature = "std"), feature = "read"))]
use alloc::format;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg_attr(feature = "read", derive(DekuRead))]
#[cfg_attr(feature = "write", derive(DekuWrite))]
#[cfg_attr(feature = "debug", derive(Debug))]
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
macro_rules! as_tlv_structure {
    ($fn_name:ident, $type_name:ty) => {
        pub fn $fn_name(&self) -> Option<$type_name> {
            Some(<$type_name>::try_from(self.tlv_data.as_ref()).unwrap())
        }
    };
}
macro_rules! into_tlv {
    ($type_name:ty, $tlv_type:expr) => {
        #[cfg(feature = "write")]
        use super::{TLVType, TLV};
        #[cfg(feature = "write")]
        impl Into<TLV> for $type_name {
            fn into(self) -> TLV {
                let bytes = self.to_bytes().unwrap();
                TLV {
                    tlv_type: $tlv_type,
                    tlv_length: bytes.len() as u16,
                    tlv_data: bytes,
                }
            }
        }
    };
}

#[cfg_attr(feature = "read", derive(DekuRead))]
#[cfg_attr(feature = "write", derive(DekuWrite))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
/// A **T**ype **L**ength **V**alue structure.
pub struct TLV {
    /// The type.
    pub tlv_type: TLVType,

    /// The length.
    #[deku(update = "self.tlv_data.len()")]
    pub tlv_length: u16,

    /// The data contained within the TLV.
    #[deku(count = "tlv_length")]
    pub tlv_data: Vec<u8>,
}
#[cfg(all(
    feature = "read",
    any(
        feature = "version_tlv",
        feature = "dns_sd_tlvs",
        feature = "sync_elect_tlvs",
        feature = "data_tlvs"
    )
))]
impl TLV {
    #[cfg(feature = "version_tlv")]
    as_tlv_structure! {as_version, VersionTLV}
    #[cfg(feature = "dns_sd_tlvs")]
    as_tlv_structure! {as_arpa, ArpaTLV}
    #[cfg(feature = "sync_elect_tlvs")]
    as_tlv_structure! {as_chan_seq, ChannelSequenceTLV}
}

#[cfg(feature = "version_tlv")]
pub mod version {
    use deku::prelude::*;

    use crate::action_frame::version::AWDLVersion;

    #[cfg(all(not(feature = "std"), feature = "read"))]
    use alloc::format;
    #[cfg(all(not(feature = "std"), feature = "write"))]
    use alloc::vec::Vec;

    #[cfg_attr(feature = "read", derive(DekuRead))]
    #[cfg_attr(feature = "write", derive(DekuWrite))]
    #[cfg_attr(feature = "debug", derive(Debug))]
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
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Copy, PartialEq, Eq)]
    /// A TLV containing the actual version of the AWDL protocol.
    pub struct VersionTLV {
        ///  The AWDL protocol version.
        pub version: AWDLVersion,

        /// The device class.
        pub device_class: AWDLDeviceClass,
    }
    into_tlv!(VersionTLV, TLVType::Version);
}
#[cfg(feature = "dns_sd_tlvs")]
pub mod dns_sd {
    use deku::{bitvec::Msb0, prelude::*};

    use crate::action_frame::dns_compression::AWDLDnsCompression;
    #[cfg(all(not(feature = "std"), feature = "write"))]
    use alloc::{vec::Vec};
    #[cfg(all(not(feature = "std"), feature = "read"))]
    use alloc::{format, string::String};
    #[cfg(not(feature = "std"))]
    use alloc::borrow::Cow;
    #[cfg(feature = "std")]
    use std::borrow::Cow;

    #[cfg(feature = "write")]
    use deku::bitvec::BitVec;
    #[cfg(feature = "read")]
    use deku::{bitvec::BitSlice, ctx::Endian};

    #[cfg(feature = "read")]
    fn read_string(
        rest: &BitSlice<u8, Msb0>,
        len: usize,
    ) -> Result<(&BitSlice<u8, Msb0>, Cow<'_, str>), DekuError> {
        let (rest, string) = <&[u8]>::read(&rest, (len.into(), Endian::Little))?;
        Ok((rest, String::from_utf8_lossy(string)))
    }
    #[cfg(feature = "write")]
    fn write_string(output: &mut BitVec<u8, Msb0>, string: &str) -> Result<(), DekuError> {
        string.as_bytes().write(output, ())
    }

    #[cfg_attr(feature = "read", derive(DekuRead))]
    #[cfg_attr(feature = "write", derive(DekuWrite))]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Default, PartialEq, Eq)]
    /// A hostname combined with the [domain](AWDLDnsCompression).
    pub struct Hostname<'a> {
        /// An unknown random prefix byte before the host.
        pub unknown: u8,

        /// An unknown random prefix byte before the host.
        #[deku(
            reader = "read_string(deku::rest, (deku::rest.len() / 8) - 2)",
            writer = "write_string(deku::output, &self.host)"
        )]
        /// The hostname of the peer.
        pub host: Cow<'a, str>,

        /// The domain in [compressed form](AWDLDnsCompression).
        pub domain: AWDLDnsCompression,
    }

    #[cfg_attr(feature = "read", derive(DekuRead))]
    #[cfg_attr(feature = "write", derive(DekuWrite))]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Default, PartialEq, Eq)]
    /// A TLV containing the hostname of the peer. Used for reverse DNS.
    pub struct ArpaTLV<'a> {
        /// A currently unknown flags header.
        pub flags: u8,

        /// The actual arpa data.
        pub arpa: Hostname<'a>,
    }
    into_tlv!(ArpaTLV<'_>, TLVType::Arpa);
}
#[cfg(feature = "sync_elect_tlvs")]
pub mod sync_elect {
    use deku::{
        prelude::*,
    };

    #[cfg(not(feature = "std"))]
    use alloc::{format, vec::Vec};
    #[cfg(all(not(feature = "std"), feature = "read"))]
    use alloc::vec;

    use crate::action_frame::channel::{Channel, ChannelEncoding};

    #[cfg_attr(feature = "read", derive(DekuRead))]
    #[cfg_attr(feature = "write", derive(DekuWrite))]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, PartialEq, Eq)]
    pub struct ChannelSequenceTLV {
        /// The number of channels minus one.
        #[deku(update = "self.channels.len()-1")]
        pub channel_count: u8,

        /// The channel encoding.
        pub channel_encoding: ChannelEncoding,

        /// The amount of duplicates in the channel sequence.
        pub duplicate_count: u8,

        /// The amount of AWs spent on one channel.
        pub step_count: u8,

        /// Honestly no idea.
        pub fill_channel: u16,

        /// The channels.
        #[deku(
            reader = "Self::read_channels(deku::rest, channel_encoding, &(channel_count + 1))",
            pad_bytes_after = "3"
        )]
        pub channels: Vec<Channel>,
    }
    impl ChannelSequenceTLV {
        #[cfg(feature = "read")]
        fn read_channels<'a>(
            rest: &'a deku::bitvec::BitSlice<u8, deku::bitvec::Msb0>,
            channel_encoding: &ChannelEncoding,
            channel_count: &u8,
        ) -> Result<(&'a deku::bitvec::BitSlice<u8, deku::bitvec::Msb0>, Vec<Channel>), DekuError> {
            let mut channels = vec![Channel::Simple(0xff); *channel_count as usize];
            let mut rest = rest;
            for channel in channels.iter_mut() {
                (rest, *channel) = Channel::read(rest, channel_encoding)?;
            }
            Ok((rest, channels))
        }
    }
    into_tlv!(ChannelSequenceTLV, TLVType::ChannelSequence);
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    use alloc::vec;

    use deku::DekuContainerWrite;

    use crate::action_frame::tlv::{TLVType, TLV};

    use super::{sync_elect::ChannelSequenceTLV, version::VersionTLV, ArpaTLV};

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
        include_bytes!("../../test_bins/arpa_tlv.bin")[3..].to_vec()
    );
    test_tlv!(
        VersionTLV,
        TLVType::Version,
        test_version,
        include_bytes!("../../test_bins/version_tlv.bin")[3..].to_vec()
    );
    test_tlv!(
        ChannelSequenceTLV,
        TLVType::ChannelSequence,
        test_channel_seq,
        include_bytes!("../../test_bins/channel_sequence_tlv.bin")[3..].to_vec()
    );
}
