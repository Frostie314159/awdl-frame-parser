use crate::enum_to_int;

use alloc::{borrow::Cow, vec::Vec};

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
impl crate::parser::Read for TLV<'_> {
    type Error = crate::parser::ParserError;

    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, Self::Error> {
        use crate::parser::ParserError;
        use core::cmp::Ordering;
        if data.len() < 3 {
            return Err(ParserError::TooLittleData(3 - data.len()));
        }

        let tlv_type = data.next().unwrap().into();
        let tlv_length = u16::from_le_bytes(data.next_chunk().unwrap());
        let tlv_data = match data.len().cmp(&(tlv_length as usize)) {
            Ordering::Less => {
                return Err(ParserError::TooLittleData(tlv_length as usize - data.len()))
            }
            Ordering::Equal | Ordering::Greater => {
                Cow::Owned((0..tlv_length).map(|_| data.next().unwrap()).collect())
            }
        };

        Ok(Self { tlv_type, tlv_data })
    }
}
#[cfg(feature = "write")]
impl<'a> crate::parser::Write<'a> for TLV<'a> {
    fn to_bytes(&self) -> Cow<'a, [u8]> {
        let tlv_length = self.tlv_data.len().to_le_bytes();
        let tlv_header = [self.tlv_type.into(), tlv_length[0], tlv_length[1]];
        [tlv_header.as_slice(), &self.tlv_data].concat().into()
    }
}
#[cfg(feature = "read")]
impl<'a> crate::parser::Read for Vec<TLV<'a>> {
    type Error = ();

    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, Self::Error> {
        let mut tlvs = alloc::vec![]; // Evil allocation.
        while let Ok(tlv) = TLV::from_bytes(data) {
            tlvs.push(tlv);
        }
        Ok(tlvs)
    }
}
#[cfg(feature = "write")]
impl<'a> crate::parser::Write<'a> for Vec<TLV<'a>> {
    fn to_bytes(&self) -> Cow<'a, [u8]> {
        use alloc::borrow::ToOwned;
        Cow::Owned(
            self.iter()
                .map(|x| x.to_bytes())
                .collect::<Vec<Cow<[u8]>>>()
                .concat()
                .as_slice()
                .to_owned(),
        )
    }
}
#[cfg(test)]
#[test]
fn test_tlv() {
    use crate::parser::{Read, Write};
    use alloc::borrow::ToOwned;
    let bytes = &[0x04, 0x05, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff];
    let tlv = TLV::from_bytes(&mut bytes.into_iter().map(|x| *x)).unwrap();
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
    ParserError(crate::parser::ParserError),
}
macro_rules! impl_tlv_conversion_fixed {
    ($ntype:ty, $tlv_type:expr, $tlv_length:expr) => {
        #[cfg(feature = "write")]
        impl From<$ntype> for super::TLV<'_> {
            fn from(value: $ntype) -> Self {
                use crate::parser::WriteFixed;
                use alloc::borrow::ToOwned;
                Self {
                    tlv_type: $tlv_type,
                    tlv_data: alloc::borrow::Cow::Owned(value.to_bytes().as_slice().to_owned()),
                }
            }
        }

        #[cfg(feature = "read")]
        impl TryFrom<super::TLV<'_>> for $ntype {
            type Error = super::FromTLVError;
            fn try_from(value: super::TLV<'_>) -> Result<Self, Self::Error> {
                use crate::parser::ReadFixed;
                if value.tlv_data.len() < $tlv_length {
                    return Err(crate::action_frame::tlv::FromTLVError::IncorrectTlvLength);
                }
                if value.tlv_type != $tlv_type {
                    return Err(crate::action_frame::tlv::FromTLVError::IncorrectTlvType);
                }
                Self::from_bytes(&value.tlv_data.iter().map(|x| *x).next_chunk().unwrap())
                    .map_err(|e| crate::action_frame::tlv::FromTLVError::ParserError(e))
            }
        }
    };
}

#[cfg(feature = "version_tlv")]
pub mod version {
    use crate::{action_frame::version::AWDLVersion, enum_to_int};

    use super::TLVType;

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
        0x03,
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
    impl crate::parser::ReadFixed<2> for VersionTLV {
        type Error = crate::parser::ParserError;
        fn from_bytes(data: &[u8; 2]) -> Result<Self, Self::Error> {
            let mut data = data.iter().copied();
            Ok(Self {
                version: AWDLVersion::from_bytes(&data.next_chunk().unwrap()).unwrap(),
                device_class: data.next().unwrap().into(),
            })
        }
    }
    #[cfg(feature = "write")]
    impl crate::parser::WriteFixed<2> for VersionTLV {
        fn to_bytes(&self) -> [u8; 2] {
            [self.version.to_bytes()[0], self.device_class.into()]
        }
    }

    #[cfg(test)]
    #[test]
    fn test_version_tlv() {
        use super::TLV;
        use crate::parser::{Read, WriteFixed};

        let bytes = include_bytes!("../../test_bins/version_tlv.bin");

        let tlv = TLV::from_bytes(&mut bytes.iter().map(|x| *x)).unwrap();

        let version_tlv = VersionTLV::try_from(tlv.clone()).unwrap();
        assert_eq!(tlv, <VersionTLV as Into<TLV>>::into(version_tlv));

        assert_eq!(
            version_tlv,
            VersionTLV {
                version: AWDLVersion { major: 3, minor: 4 },
                device_class: AWDLDeviceClass::MacOS,
            }
        );
        assert_eq!(version_tlv.to_bytes(), bytes[3..]);
    }
}
#[cfg(feature = "dns_sd_tlvs")]
pub mod dns_sd {
    use crate::action_frame::dns_compression::AWDLDnsCompression;
    use alloc::{borrow::Cow, str};

    use super::TLVType;

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Default, PartialEq, Eq)]
    /// A hostname combined with the [domain](AWDLDnsCompression).
    pub struct Hostname<'a> {
        /// An unknown random prefix byte before the host.
        pub unknown: u8,

        /// The hostname of the peer.
        pub host: Cow<'a, str>,

        /// The domain in [compressed form](AWDLDnsCompression).
        pub domain: AWDLDnsCompression,
    }
    #[cfg(feature = "read")]
    impl crate::parser::Read for Hostname<'_> {
        type Error = crate::parser::ParserError;

        fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, Self::Error> {
            #[cfg(not(feature = "std"))]
            use alloc::string::String;

            if data.len() < 3 {
                return Err(crate::parser::ParserError::TooLittleData(3 - data.len()));
            }
            let unknown = data.next().unwrap();
            let binding = (0..(data.len() - 2))
                .map(|_| data.next().unwrap())
                .collect::<Cow<[u8]>>();
            let host = match binding {
                Cow::Borrowed(bytes) => match str::from_utf8(bytes) {
                    Ok(str_ref) => Cow::Borrowed(str_ref),
                    Err(_) => Cow::Owned(String::from_utf8_lossy(bytes).into_owned()),
                },
                Cow::Owned(bytes) => match String::from_utf8(bytes) {
                    Ok(string) => Cow::Owned(string),
                    Err(err) => Cow::Owned(
                        err.into_bytes()
                            .into_iter()
                            .map(|b| b as char)
                            .collect::<String>(),
                    ),
                },
            };
            let domain = u16::from_le_bytes(data.next_chunk().unwrap()).into();

            Ok(Self {
                unknown,
                host,
                domain,
            })
        }
    }
    #[cfg(feature = "write")]
    impl<'a> crate::parser::Write<'a> for Hostname<'a> {
        fn to_bytes(&self) -> Cow<'a, [u8]> {
            let host = self.host.as_bytes();
            let binding = <AWDLDnsCompression as Into<u16>>::into(self.domain).to_le_bytes();
            let domain = binding.as_slice();
            [[self.unknown].as_slice(), host, domain].concat().into()
        }
    }

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Default, PartialEq, Eq)]
    /// A TLV containing the hostname of the peer. Used for reverse DNS.
    pub struct ArpaTLV<'a> {
        /// A currently unknown flags header.
        pub flags: u8,

        /// The actual arpa data.
        pub arpa: Hostname<'a>,
    }
    #[cfg(feature = "read")]
    impl<'a> crate::parser::Read for ArpaTLV<'a> {
        type Error = crate::parser::ParserError;

        fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, Self::Error> {
            if data.len() < 4 {
                return Err(crate::parser::ParserError::TooLittleData(data.len() - 4));
            }
            let flags = data.next().unwrap();
            let arpa = Hostname::from_bytes(data)?;
            Ok(Self { flags, arpa })
        }
    }
    #[cfg(feature = "write")]
    impl<'a> crate::parser::Write<'a> for ArpaTLV<'a> {
        fn to_bytes(&self) -> Cow<'a, [u8]> {
            [[self.flags].as_slice(), &self.arpa.to_bytes()]
                .concat()
                .into()
        }
    }
    #[cfg(feature = "write")]
    impl<'a> From<ArpaTLV<'a>> for super::TLV<'a> {
        fn from(value: ArpaTLV<'a>) -> Self {
            use crate::parser::Write;

            Self {
                tlv_type: TLVType::Arpa,
                tlv_data: value.to_bytes(),
            }
        }
    }
    #[cfg(feature = "read")]
    impl<'a> TryFrom<super::TLV<'a>> for ArpaTLV<'a> {
        type Error = super::FromTLVError;
        fn try_from(value: super::TLV<'a>) -> Result<Self, Self::Error> {
            use crate::parser::Read;

            if value.tlv_data.len() < 4 {
                return Err(crate::action_frame::tlv::FromTLVError::IncorrectTlvLength);
            }
            if value.tlv_type != TLVType::Arpa {
                return Err(crate::action_frame::tlv::FromTLVError::IncorrectTlvType);
            }
            Self::from_bytes(&mut value.tlv_data.iter().copied())
                .map_err(crate::action_frame::tlv::FromTLVError::ParserError)
        }
    }
    #[cfg(test)]
    #[test]
    fn test_arpa_tlv() {
        use super::TLV;
        use crate::parser::{Read, Write};

        let bytes = include_bytes!("../../test_bins/arpa_tlv.bin");

        let tlv = TLV::from_bytes(&mut bytes.iter().map(|x| *x)).unwrap();

        let arpa_tlv = ArpaTLV::try_from(tlv.clone()).unwrap();
        assert_eq!(tlv, <ArpaTLV as Into<TLV>>::into(arpa_tlv.clone()));

        assert_eq!(
            arpa_tlv,
            ArpaTLV {
                flags: 0x03,
                arpa: Hostname {
                    unknown: 0x0f,
                    host: "simon-framework".into(), // My hostname so calm down.
                    domain: AWDLDnsCompression::Local
                }
            }
        );

        assert_eq!(arpa_tlv.to_bytes(), &bytes[3..]);
    }
}
#[cfg(feature = "sync_elect_tlvs")]
pub mod sync_elect {
    use crate::action_frame::channel::{ChannelEncoding, ChannelSequence};

    use super::{TLVType, TLV};

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, PartialEq, Eq)]
    pub struct ChannelSequenceTLV {
        /// The channel encoding.
        pub channel_encoding: ChannelEncoding,

        /// The amount of duplicates in the channel sequence.
        pub duplicate_count: u8,

        /// The amount of AWs spent on one channel.
        pub step_count: u8,

        /// Honestly no idea.
        pub fill_channel: u16,

        /// The channels.
        pub channel_sequence: ChannelSequence,
    }
    type ChannelSequenceHeader = (u8, ChannelEncoding, u8, u8, u16);

    #[cfg(feature = "read")]
    impl crate::parser::ReadFixed<6> for ChannelSequenceHeader {
        type Error = crate::parser::ParserError;
        fn from_bytes(data: &[u8; 6]) -> Result<Self, Self::Error> {
            let mut data = data.iter().copied();

            let channel_count = data.next().unwrap() + 1; // Don't ask.
            let channel_encoding = data.next().unwrap().into();
            let duplicate_count = data.next().unwrap();
            let step_count = data.next().unwrap() + 1;
            let fill_channels = u16::from_le_bytes(data.next_chunk().unwrap());
            Ok((
                channel_count,
                channel_encoding,
                duplicate_count,
                step_count,
                fill_channels,
            ))
        }
    }
    #[cfg(feature = "write")]
    impl crate::parser::WriteFixed<6> for ChannelSequenceHeader {
        fn to_bytes(&self) -> [u8; 6] {
            let channel_encoding = self.1.into();
            let fill_channel = self.4.to_le_bytes();
            [
                self.0 - 1,
                channel_encoding,
                self.2,
                self.3 - 1,
                fill_channel[0],
                fill_channel[1],
            ]
        }
    }
    #[cfg(feature = "read")]
    impl crate::parser::Read for ChannelSequenceTLV {
        type Error = crate::parser::ParserError;
        fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, Self::Error> {
            use crate::parser::{ParserError, ReadCtx, ReadFixed};
            if data.len() < 9 {
                return Err(ParserError::TooLittleData(6 - data.len()));
            }
            let (channel_count, channel_encoding, duplicate_count, step_count, fill_channel) =
                ChannelSequenceHeader::from_bytes(&data.next_chunk().unwrap()).unwrap();
            let channel_sequence =
                ChannelSequence::from_bytes(data, (&channel_count, &channel_encoding)).unwrap();
            let _ = data.next_chunk::<3>(); // Discard padding.
            Ok(Self {
                channel_encoding,
                duplicate_count,
                step_count,
                fill_channel,
                channel_sequence,
            })
        }
    }
    #[cfg(feature = "write")]
    impl<'a> crate::parser::Write<'a> for ChannelSequenceTLV {
        fn to_bytes(&self) -> alloc::borrow::Cow<'a, [u8]> {
            use crate::parser::WriteFixed;

            let binding = (
                self.channel_sequence.len() as u8,
                self.channel_encoding,
                self.duplicate_count,
                self.step_count,
                self.fill_channel,
            )
                .to_bytes();
            let header = binding.iter();
            let binding = self.channel_sequence.to_bytes();
            let channel_sequence = binding.iter();
            let padding = [0; 3].iter();
            header
                .chain(channel_sequence.chain(padding))
                .copied()
                .collect()
        }
    }
    impl From<ChannelSequenceTLV> for TLV<'_> {
        fn from(value: ChannelSequenceTLV) -> Self {
            use crate::parser::Write;
            Self {
                tlv_type: TLVType::ChannelSequence,
                tlv_data: value.to_bytes(),
            }
        }
    }
    impl TryFrom<TLV<'_>> for ChannelSequenceTLV {
        type Error = crate::action_frame::tlv::FromTLVError;
        fn try_from(value: TLV) -> Result<Self, Self::Error> {
            use crate::{action_frame::tlv::FromTLVError, parser::Read};

            if value.tlv_data.len() < 9 {
                return Err(FromTLVError::IncorrectTlvLength);
            }
            if value.tlv_type != TLVType::ChannelSequence {
                return Err(FromTLVError::IncorrectTlvType);
            }
            Self::from_bytes(&mut value.tlv_data.iter().copied()).map_err(FromTLVError::ParserError)
        }
    }
    #[cfg(test)]
    #[test]
    fn test_channel_sequence_tlv() {
        use super::TLV;
        use crate::{
            action_frame::channel::{fixed_channel_sequence, Channel},
            parser::{Read, Write},
        };

        let bytes = include_bytes!("../../test_bins/channel_sequence_tlv.bin");

        let tlv = TLV::from_bytes(&mut bytes.iter().map(|x| *x)).unwrap();

        let channel_sequence_tlv = ChannelSequenceTLV::try_from(tlv.clone()).unwrap();
        assert_eq!(
            tlv,
            <ChannelSequenceTLV as Into<TLV>>::into(channel_sequence_tlv.clone())
        );

        assert_eq!(
            channel_sequence_tlv,
            ChannelSequenceTLV {
                channel_encoding: ChannelEncoding::OpClass,
                duplicate_count: 0,
                step_count: 4,
                fill_channel: 0xffff,
                channel_sequence: fixed_channel_sequence(Channel::OpClass(0x6, 0x51)),
            }
        );

        assert_eq!(channel_sequence_tlv.to_bytes(), &bytes[3..]);
    }

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, PartialEq, Eq)]
    /// A TLV describing the election parameters of a peer.
    pub struct ElectionParametersTLV {
        /// Unknown
        pub flags: u8,

        /// Unknown
        pub id: u16,

        /// Distance to the mesh master
        pub distance_to_master: u8,

        /// Address of the master
        pub master_address: [u8; 6],

        /// Self metric of the master
        pub master_metric: u32,

        /// Own self metric
        pub self_metric: u32,
    }
    #[cfg(feature = "read")]
    impl crate::parser::ReadFixed<21> for ElectionParametersTLV {
        type Error = crate::parser::ParserError;

        fn from_bytes(data: &[u8; 21]) -> Result<Self, Self::Error> {
            let mut data = data.iter().copied();
            let flags = data.next().unwrap();
            let id = u16::from_le_bytes(data.next_chunk().unwrap()); // In reality this is always zero.
            let distance_to_master = data.next().unwrap();
            let _ = data.next();
            let master_address = data.next_chunk::<6>().unwrap();
            let master_metric = u32::from_le_bytes(data.next_chunk().unwrap());
            let self_metric = u32::from_le_bytes(data.next_chunk().unwrap());
            Ok(Self {
                flags,
                id,
                distance_to_master,
                master_address,
                master_metric,
                self_metric,
            })
        }
    }
    #[cfg(feature = "write")]
    impl crate::parser::WriteFixed<21> for ElectionParametersTLV {
        fn to_bytes(&self) -> [u8; 21] {
            let mut bytes = [0x00; 21];
            bytes[0] = self.flags;
            bytes[1..3].copy_from_slice(&self.id.to_le_bytes());
            bytes[3] = self.distance_to_master;
            bytes[5..11].copy_from_slice(&self.master_address);
            bytes[11..15].copy_from_slice(&self.master_metric.to_le_bytes());
            bytes[15..19].copy_from_slice(&self.self_metric.to_le_bytes());
            bytes
        }
    }
    impl_tlv_conversion_fixed!(ElectionParametersTLV, TLVType::ElectionParameters, 21);

    #[cfg(test)]
    #[test]
    fn test_election_parameters_tlv() {
        use super::TLV;
        use crate::parser::{Read, WriteFixed};

        let bytes = include_bytes!("../../test_bins/election_parameters_tlv.bin");

        let tlv = TLV::from_bytes(&mut bytes.iter().map(|x| *x)).unwrap();

        let election_parameters_tlv = ElectionParametersTLV::try_from(tlv.clone()).unwrap();
        assert_eq!(
            tlv,
            <ElectionParametersTLV as Into<TLV>>::into(election_parameters_tlv.clone())
        );

        assert_eq!(
            election_parameters_tlv,
            ElectionParametersTLV {
                flags: 0x00,
                id: 0x00,
                distance_to_master: 0x02,
                master_address: [0x3a, 0xb4, 0x08, 0x6e, 0x66, 0x3d],
                master_metric: 541,
                self_metric: 60
            }
        );

        assert_eq!(election_parameters_tlv.to_bytes(), bytes[3..]);
    }

    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, PartialEq, Eq)]
    /// Another TLV describing the election parameters of the peer.
    pub struct ElectionParametersV2TLV {
        /// MAC address of the master
        pub master_address: [u8; 6],

        /// MAC address of the peer this peer is syncing to
        pub sync_address: [u8; 6],

        /// Counter value of the master
        pub master_counter: u32,

        /// Distance to the current master
        pub distance_to_master: u32,

        /// Metric of the master
        pub master_metric: u32,

        /// Self metric of the peer
        pub self_metric: u32,

        /// Self counter of the peer
        pub self_counter: u32,
    }
    #[cfg(feature = "read")]
    impl crate::parser::ReadFixed<40> for ElectionParametersV2TLV {
        type Error = crate::parser::ParserError;

        fn from_bytes(data: &[u8; 40]) -> Result<Self, Self::Error> {
            let mut data = data.iter().copied();
            let master_address = data.next_chunk().unwrap();
            let sync_address = data.next_chunk().unwrap();
            let master_counter = u32::from_le_bytes(data.next_chunk().unwrap());
            let distance_to_master = u32::from_le_bytes(data.next_chunk().unwrap());
            let master_metric = u32::from_le_bytes(data.next_chunk().unwrap());
            let self_metric = u32::from_le_bytes(data.next_chunk().unwrap());
            let _ = data.next_chunk::<8>();
            let self_counter = u32::from_le_bytes(data.next_chunk().unwrap());

            Ok(Self {
                master_address,
                sync_address,
                master_counter,
                distance_to_master,
                master_metric,
                self_metric,
                self_counter,
            })
        }
    }
    #[cfg(feature = "write")]
    impl crate::parser::WriteFixed<40> for ElectionParametersV2TLV {
        fn to_bytes(&self) -> [u8; 40] {
            let mut bytes = [0x00; 40];
            bytes[0..6].copy_from_slice(&self.master_address);
            bytes[6..12].copy_from_slice(&self.sync_address);
            bytes[12..16].copy_from_slice(&self.master_counter.to_le_bytes());
            bytes[16..20].copy_from_slice(&self.distance_to_master.to_le_bytes());
            bytes[20..24].copy_from_slice(&self.master_metric.to_le_bytes());
            bytes[24..28].copy_from_slice(&self.self_metric.to_le_bytes());
            bytes[36..40].copy_from_slice(&self.self_counter.to_le_bytes());

            bytes
        }
    }
    impl_tlv_conversion_fixed!(ElectionParametersV2TLV, TLVType::ElectionParametersV2, 40);
    #[cfg(test)]
    #[test]
    fn test_election_parameters_v2_tlv() {
        use super::TLV;
        use crate::parser::{Read, WriteFixed};

        let bytes = include_bytes!("../../test_bins/election_parameters_v2_tlv.bin");

        let tlv = TLV::from_bytes(&mut bytes.iter().map(|x| *x)).unwrap();

        let election_parameters_v2_tlv = ElectionParametersV2TLV::try_from(tlv.clone()).unwrap();
        assert_eq!(
            tlv,
            <ElectionParametersV2TLV as Into<TLV>>::into(election_parameters_v2_tlv.clone())
        );

        assert_eq!(
            election_parameters_v2_tlv,
            ElectionParametersV2TLV {
                master_address: [0xce, 0x21, 0x1f, 0x62, 0x21, 0x22],
                sync_address: [0xce, 0x21, 0x1f, 0x62, 0x21, 0x22],
                master_counter: 960,
                distance_to_master: 1,
                master_metric: 650,
                self_metric: 650,
                self_counter: 30,
            }
        );

        assert_eq!(election_parameters_v2_tlv.to_bytes(), bytes[3..]);
    }
}
