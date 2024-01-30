/// TLVs regarding the data path.
pub mod data_path;
/// TLVs containing data about dns services.
pub mod dns_sd;
/// TLVs about the synchronization and election state of the peer.
pub mod sync_elect;
pub mod version;
use core::{fmt::Debug, marker::PhantomData};

use mac_parser::MACAddress;
use macro_bits::serializable_enum;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};
use tlv_rs::{raw_tlv::RawTLV, TLV};

use crate::common::{AWDLStr, ReadLabelIterator};

use self::{
    data_path::{DataPathStateTLV, HTCapabilitiesTLV, IEEE80211ContainerTLV},
    dns_sd::{ArpaTLV, ReadValueIterator, ServiceParametersTLV, ServiceResponseTLV},
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

#[derive(Clone)]
pub enum AWDLTLV<'a, MACIterator, LabelIterator, ValueIterator> {
    ServiceResponse(ServiceResponseTLV<'a, LabelIterator>),
    SynchronizationParameters(SynchronizationParametersTLV),
    ElectionParameters(ElectionParametersTLV),
    ServiceParameters(ServiceParametersTLV<ValueIterator>),
    HTCapabilities(HTCapabilitiesTLV),
    DataPathState(DataPathStateTLV),
    Arpa(ArpaTLV<LabelIterator>),
    IEEE80211Container(IEEE80211ContainerTLV<'a>),
    ChannelSequence(ChannelSequenceTLV),
    SynchronizationTree(SyncTreeTLV<MACIterator>),
    Version(VersionTLV),
    ElectionParametersV2(ElectionParametersV2TLV),
    Unknown(RawAWDLTLV<'a>),
}
macro_rules! comparisons {
    ($self:expr, $other:expr, $($path:ident),*) => {
        match ($self, $other) {
            $(
                (Self::$path(lhs), AWDLTLV::<'a, RhsMACIterator, RhsLabelIterator, RhsValueIterator>::$path(rhs)) => lhs == rhs,
            )*
            _ => false,
        }
    };
}
impl<
        'a,
        LhsMACIterator,
        RhsMACIterator,
        LhsLabelIterator,
        RhsLabelIterator,
        LhsValueIterator,
        RhsValueIterator,
    > PartialEq<AWDLTLV<'a, RhsMACIterator, RhsLabelIterator, RhsValueIterator>>
    for AWDLTLV<'a, LhsMACIterator, LhsLabelIterator, LhsValueIterator>
where
    LhsMACIterator: IntoIterator<Item = MACAddress> + Clone,
    RhsMACIterator: IntoIterator<Item = MACAddress> + Clone,
    LhsLabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    RhsLabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    LhsValueIterator: IntoIterator<Item = u8> + Clone,
    RhsValueIterator: IntoIterator<Item = u8> + Clone,
{
    fn eq(&self, other: &AWDLTLV<'a, RhsMACIterator, RhsLabelIterator, RhsValueIterator>) -> bool {
        comparisons!(
            self,
            other,
            ServiceResponse,
            SynchronizationParameters,
            ElectionParameters,
            ServiceParameters,
            HTCapabilities,
            DataPathState,
            Arpa,
            IEEE80211Container,
            ChannelSequence,
            SynchronizationTree,
            Version,
            ElectionParametersV2,
            Unknown
        )
    }
}
impl<'a, MACIterator, LabelIterator, ValueIterator> Eq
    for AWDLTLV<'a, MACIterator, LabelIterator, ValueIterator>
where
    MACIterator: IntoIterator<Item = MACAddress> + Clone,
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    ValueIterator: IntoIterator<Item = u8> + Clone,
{
}
macro_rules! debug_impls {
    ($self:expr, $f:expr, $($path:ident),*) => {
        match $self {
            $(
                Self::$path(inner) => inner.fmt($f),
            )*
        }
    };
}
impl<'a, MACIterator, LabelIterator, ValueIterator> Debug
    for AWDLTLV<'a, MACIterator, LabelIterator, ValueIterator>
where
    MACIterator: IntoIterator<Item = MACAddress> + Clone + Debug,
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone + Debug,
    ValueIterator: IntoIterator<Item = u8> + Clone,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_impls!(
            self,
            f,
            ServiceResponse,
            SynchronizationParameters,
            ElectionParameters,
            ServiceParameters,
            HTCapabilities,
            DataPathState,
            Arpa,
            IEEE80211Container,
            ChannelSequence,
            SynchronizationTree,
            Version,
            ElectionParametersV2,
            Unknown
        )
    }
}
impl<'a, MACIterator, LabelIterator, ValueIterator>
    AWDLTLV<'a, MACIterator, LabelIterator, ValueIterator>
where
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    <LabelIterator as IntoIterator>::IntoIter: Clone,
    MACIterator: IntoIterator<Item = MACAddress> + Clone,
    ValueIterator: IntoIterator<Item = u8> + Clone,
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
macro_rules! measure_with_impls {
    ($self:expr, $ctx:expr, $($path:ident),*) => {
        match $self {
            $(
                Self::$path(inner) => inner.measure_with($ctx),
            )*
            Self::Unknown(raw_tlv) => raw_tlv.slice.len()
        }
    };
}
impl<'a, MACIterator, LabelIterator, ValueIterator> MeasureWith<()>
    for AWDLTLV<'a, MACIterator, LabelIterator, ValueIterator>
where
    MACIterator: ExactSizeIterator,
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone + Debug,
    ValueIterator: IntoIterator<Item = u8> + Clone,
{
    fn measure_with(&self, ctx: &()) -> usize {
        3 + measure_with_impls!(
            self,
            ctx,
            ServiceResponse,
            SynchronizationParameters,
            ElectionParameters,
            ServiceParameters,
            HTCapabilities,
            DataPathState,
            Arpa,
            IEEE80211Container,
            ChannelSequence,
            SynchronizationTree,
            Version,
            ElectionParametersV2
        )
    }
}
macro_rules! read_impls {
    ($self:expr, $raw_tlv:expr, $($path:ident),*) => {
        match AWDLTLVType::from_representation($raw_tlv.tlv_type) {
            $(
                AWDLTLVType::$path => Self::$path($raw_tlv.slice.pread(0)?),
            )*
            AWDLTLVType::Unknown(tlv_type) => Self::Unknown(RawTLV {
                tlv_type,
                slice: $raw_tlv.slice,
                _phantom: PhantomData,
            }),
            AWDLTLVType::Null => Self::Unknown(RawTLV {
                tlv_type: 0,
                slice: $raw_tlv.slice,
                _phantom: PhantomData,
            }),
        }
    };
}
impl<'a> TryFromCtx<'a>
    for AWDLTLV<'a, ReadMACIterator<'a>, ReadLabelIterator<'a>, ReadValueIterator<'a>>
{
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let (raw_tlv, len) =
            <RawAWDLTLV<'a> as TryFromCtx<'a, Endian>>::try_from_ctx(from, Endian::Little)?;
        Ok((
            read_impls!(
                self,
                raw_tlv,
                ServiceResponse,
                SynchronizationParameters,
                ElectionParameters,
                ServiceParameters,
                HTCapabilities,
                DataPathState,
                Arpa,
                IEEE80211Container,
                ChannelSequence,
                SynchronizationTree,
                Version,
                ElectionParametersV2
            ),
            len,
        ))
    }
}
macro_rules! write_impls {
    ($self:expr, $buf:expr, $tlv_type:expr, $($path:ident),*) => {
        match $self {
            $(
                Self::$path(payload) => $buf.pwrite_with(
                    TypedAWDLTLV {
                        tlv_type: $tlv_type,
                        payload,
                        _phantom: PhantomData,
                    },
                    0,
                    Endian::Little,
                ),
            )*
            Self::Unknown(tlv) => $buf.pwrite(tlv, 0)
        }
    };
}
impl<'a, MACIterator, LabelIterator, ValueIterator> TryIntoCtx
    for AWDLTLV<'a, MACIterator, LabelIterator, ValueIterator>
where
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    <LabelIterator as IntoIterator>::IntoIter: Clone,
    MACIterator: IntoIterator<Item = MACAddress> + ExactSizeIterator + Clone,
    ValueIterator: IntoIterator<Item = u8> + Clone,
{
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let tlv_type = self.get_type();
        write_impls!(
            self,
            buf,
            tlv_type,
            ServiceResponse,
            SynchronizationParameters,
            ElectionParameters,
            ServiceParameters,
            HTCapabilities,
            DataPathState,
            Arpa,
            IEEE80211Container,
            ChannelSequence,
            SynchronizationTree,
            Version,
            ElectionParametersV2
        )
    }
}

/// Default [AWDLTLV] returned by reading.
pub type DefaultAWDLTLV<'a> =
    AWDLTLV<'a, ReadMACIterator<'a>, ReadLabelIterator<'a>, ReadValueIterator<'a>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TLVReadIterator<'a> {
    bytes: Option<&'a [u8]>,
}
impl<'a> TLVReadIterator<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes: Some(bytes) }
    }
}
impl<'a> Iterator for TLVReadIterator<'a> {
    type Item = DefaultAWDLTLV<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut offset = 0;

        let tlv = match self.bytes?.gread(&mut offset).ok() {
            Some(tlv) => tlv,
            None => {
                self.bytes = None;
                return None;
            }
        };
        self.bytes = self.bytes.map(|bytes| &bytes[offset..]);

        Some(tlv)
    }
}
