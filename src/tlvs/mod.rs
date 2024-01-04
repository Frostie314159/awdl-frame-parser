/// TLVs regarding the data path.
pub mod data_path;
/// TLVs containing data about dns services.
pub mod dns_sd;
/// TLVs about the synchronization and election state of the peer.
pub mod sync_elect;
pub mod version;
use core::marker::PhantomData;

use mac_parser::MACAddress;
use macro_bits::serializable_enum;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};
use tlv_rs::{raw_tlv::RawTLV, TLV};

use crate::common::{AWDLStr, LabelIterator};

use self::{
    data_path::{DataPathStateTLV, HTCapabilitiesTLV, IEEE80211ContainerTLV},
    dns_sd::{ArpaTLV, ServiceParametersTLV, ServiceResponseTLV},
    sync_elect::{
        ChannelSequenceTLV, ElectionParametersTLV, ElectionParametersV2TLV, ReadMACIterator,
        SyncTreeTLV, SynchronizationParametersTLV,
    },
    version::VersionTLV,
};

serializable_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    /// The type of the TLV.
    pub enum AWDLTLVType: u8 {
        #[default]
        /// Required for `tlv-rs`.
        Null => 0x00,

        /// The service parameters.
        ServiceResponse => 0x02,

        /// The synchronization parameters.
        SynchronizationParameters => 0x04,

        /// The election parameters.
        ElectionParameters => 0x05,

        /// The service parameters.
        ServiceParameters => 0x06,

        /// The HT capabilities.
        HTCapabilities => 0x07,

        /// The data path state.
        DataPathState => 0x0C,

        /// The hostname of the peer.
        Arpa => 0x10,

        /// The VHT capabilities.
        IEEE80211Container => 0x11,

        /// The channel sequence.
        ChannelSequence => 0x12,

        /// The synchronization tree.
        SynchronizationTree => 0x14,

        /// The actual version of the AWDL protocol, that's being used.
        Version => 0x15,

        /// The V2 Election Parameters.
        ElectionParametersV2 => 0x18
    }
}

pub type RawAWDLTLV<'a> = RawTLV<'a, u8, u16>;
pub type TypedAWDLTLV<'a, Payload> = TLV<u8, u16, AWDLTLVType, Payload>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AWDLTLV<'a, MACIterator, LabelIterator>
where
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    <LabelIterator as IntoIterator>::IntoIter: Clone,
    MACIterator: IntoIterator<Item = MACAddress>,
    <MACIterator as IntoIterator>::IntoIter: Clone,
{
    ServiceResponse(ServiceResponseTLV<'a, LabelIterator>),
    SynchronizationParameters(SynchronizationParametersTLV),
    ElectionParameters(ElectionParametersTLV),
    ServiceParameters(ServiceParametersTLV),
    HTCapabilities(HTCapabilitiesTLV),
    DataPathState(DataPathStateTLV),
    Arpa(ArpaTLV<'a, LabelIterator>),
    IEEE80211Container(IEEE80211ContainerTLV<'a>),
    ChannelSequence(ChannelSequenceTLV),
    SynchronizationTree(SyncTreeTLV<MACIterator>),
    Version(VersionTLV),
    ElectionParametersV2(ElectionParametersV2TLV),
    Unknown(RawAWDLTLV<'a>),
}
impl<'a, MACIterator, LabelIterator> AWDLTLV<'a, MACIterator, LabelIterator>
where
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    <LabelIterator as IntoIterator>::IntoIter: Clone,
    MACIterator: IntoIterator<Item = MACAddress>,
    <MACIterator as IntoIterator>::IntoIter: Clone,
{
    pub const fn get_type(&self) -> AWDLTLVType {
        match self {
            AWDLTLV::Arpa(_) => AWDLTLVType::Arpa,
            AWDLTLV::ChannelSequence(_) => AWDLTLVType::ChannelSequence,
            AWDLTLV::DataPathState(_) => AWDLTLVType::DataPathState,
            AWDLTLV::ElectionParameters(_) => AWDLTLVType::ElectionParameters,
            AWDLTLV::ElectionParametersV2(_) => AWDLTLVType::ElectionParametersV2,
            AWDLTLV::HTCapabilities(_) => AWDLTLVType::HTCapabilities,
            AWDLTLV::IEEE80211Container(_) => AWDLTLVType::IEEE80211Container,
            AWDLTLV::ServiceParameters(_) => AWDLTLVType::ServiceParameters,
            AWDLTLV::ServiceResponse(_) => AWDLTLVType::ServiceResponse,
            AWDLTLV::SynchronizationParameters(_) => AWDLTLVType::SynchronizationParameters,
            AWDLTLV::SynchronizationTree(_) => AWDLTLVType::SynchronizationTree,
            AWDLTLV::Version(_) => AWDLTLVType::Version,
            AWDLTLV::Unknown(raw_tlv) => AWDLTLVType::Unknown(raw_tlv.tlv_type),
        }
    }
}
impl<'a> TryFromCtx<'a> for AWDLTLV<'a, ReadMACIterator<'a>, LabelIterator<'a>> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let (raw_tlv, len) =
            <RawAWDLTLV<'a> as TryFromCtx<'a, Endian>>::try_from_ctx(from, Endian::Little)?;
        Ok((
            match AWDLTLVType::from_representation(raw_tlv.tlv_type) {
                AWDLTLVType::ServiceResponse => Self::ServiceResponse(raw_tlv.slice.pread(0)?),
                AWDLTLVType::SynchronizationParameters => {
                    Self::SynchronizationParameters(raw_tlv.slice.pread(0)?)
                }
                AWDLTLVType::ElectionParameters => {
                    Self::ElectionParameters(raw_tlv.slice.pread(0)?)
                }
                AWDLTLVType::ServiceParameters => Self::ServiceParameters(raw_tlv.slice.pread(0)?),
                AWDLTLVType::HTCapabilities => Self::HTCapabilities(raw_tlv.slice.pread(0)?),
                AWDLTLVType::DataPathState => Self::DataPathState(raw_tlv.slice.pread(0)?),
                AWDLTLVType::Arpa => Self::Arpa(raw_tlv.slice.pread(0)?),
                AWDLTLVType::IEEE80211Container => {
                    Self::IEEE80211Container(raw_tlv.slice.pread(0)?)
                }
                AWDLTLVType::ChannelSequence => Self::ChannelSequence(raw_tlv.slice.pread(0)?),
                AWDLTLVType::SynchronizationTree => {
                    Self::SynchronizationTree(raw_tlv.slice.pread(0)?)
                }
                AWDLTLVType::Version => Self::Version(raw_tlv.slice.pread(0)?),
                AWDLTLVType::ElectionParametersV2 => {
                    Self::ElectionParametersV2(raw_tlv.slice.pread(0)?)
                }
                AWDLTLVType::Unknown(tlv_type) => Self::Unknown(RawTLV {
                    tlv_type,
                    slice: raw_tlv.slice,
                    _phantom: PhantomData,
                }),
                AWDLTLVType::Null => Self::Unknown(RawTLV {
                    tlv_type: 0,
                    slice: raw_tlv.slice,
                    _phantom: PhantomData,
                }),
            },
            len,
        ))
    }
}
impl<'a, MACIterator: IntoIterator<Item = MACAddress> + MeasureWith<()>, LabelIterator> TryIntoCtx
    for AWDLTLV<'a, MACIterator, LabelIterator>
where
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    <LabelIterator as IntoIterator>::IntoIter: Clone,
    MACIterator: IntoIterator<Item = MACAddress> + ExactSizeIterator,
    <MACIterator as IntoIterator>::IntoIter: Clone,
{
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let tlv_type = self.get_type();
        match self {
            AWDLTLV::ServiceResponse(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::SynchronizationParameters(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::ElectionParameters(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::ServiceParameters(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::HTCapabilities(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::DataPathState(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::Arpa(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::IEEE80211Container(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::ChannelSequence(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::SynchronizationTree(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::Version(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::ElectionParametersV2(payload) => buf.pwrite_with(
                TypedAWDLTLV {
                    tlv_type,
                    payload,
                    _phantom: PhantomData,
                },
                0,
                Endian::Little,
            ),
            AWDLTLV::Unknown(tlv) => buf.pwrite(tlv, 0),
        }
    }
}
