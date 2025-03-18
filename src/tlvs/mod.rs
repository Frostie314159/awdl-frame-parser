/// TLVs regarding the data path.
pub mod data_path;
/// TLVs containing data about dns services.
pub mod dns_sd;
/// TLVs about the synchronization and election state of the peer.
pub mod sync_elect;
pub mod version;
use core::{fmt::Debug, iter::repeat, marker::PhantomData};

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
    dns_sd::{ArpaTLV, ServiceResponseTLV},
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

        // The service parameters.
        //ServiceParameters => 0x06,

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
/// A trait implemented by all AWDL TLVs.
pub trait AwdlTlv {
    const TLV_TYPE: AWDLTLVType;
}

pub type RawAWDLTLV<'a> = RawTLV<'a, u8, u16>;
pub type TypedAWDLTLV<'a, Payload> = TLV<u8, u16, AWDLTLVType, Payload>;

#[derive(Clone)]
pub enum AWDLTLV<'a, MACIterator, LabelIterator> {
    ServiceResponse(ServiceResponseTLV<'a, LabelIterator>),
    SynchronizationParameters(SynchronizationParametersTLV),
    ElectionParameters(ElectionParametersTLV),
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
                (Self::$path(lhs), AWDLTLV::<'a, RhsMACIterator, RhsLabelIterator>::$path(rhs)) => lhs == rhs,
            )*
            _ => false,
        }
    };
}
impl<'a, LhsMACIterator, RhsMACIterator, LhsLabelIterator, RhsLabelIterator>
    PartialEq<AWDLTLV<'a, RhsMACIterator, RhsLabelIterator>>
    for AWDLTLV<'a, LhsMACIterator, LhsLabelIterator>
where
    LhsMACIterator: IntoIterator<Item = MACAddress> + Clone,
    RhsMACIterator: IntoIterator<Item = MACAddress> + Clone,
    LhsLabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    RhsLabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
{
    fn eq(&self, other: &AWDLTLV<'a, RhsMACIterator, RhsLabelIterator>) -> bool {
        comparisons!(
            self,
            other,
            ServiceResponse,
            SynchronizationParameters,
            ElectionParameters,
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
impl<'a, MACIterator, LabelIterator> Eq for AWDLTLV<'a, MACIterator, LabelIterator>
where
    MACIterator: IntoIterator<Item = MACAddress> + Clone,
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
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
impl<'a, MACIterator, LabelIterator> Debug for AWDLTLV<'a, MACIterator, LabelIterator>
where
    MACIterator: IntoIterator<Item = MACAddress> + Clone + Debug,
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        debug_impls!(
            self,
            f,
            ServiceResponse,
            SynchronizationParameters,
            ElectionParameters,
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
impl<'a, MACIterator, LabelIterator> AWDLTLV<'a, MACIterator, LabelIterator>
where
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    <LabelIterator as IntoIterator>::IntoIter: Clone,
    MACIterator: IntoIterator<Item = MACAddress> + Clone,
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
impl<'a, MACIterator, LabelIterator> MeasureWith<()> for AWDLTLV<'a, MACIterator, LabelIterator>
where
    MACIterator: ExactSizeIterator,
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone + Debug,
{
    fn measure_with(&self, ctx: &()) -> usize {
        3 + measure_with_impls!(
            self,
            ctx,
            ServiceResponse,
            SynchronizationParameters,
            ElectionParameters,
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
        match AWDLTLVType::from_bits($raw_tlv.tlv_type) {
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
impl<'a> TryFromCtx<'a> for AWDLTLV<'a, ReadMACIterator<'a>, ReadLabelIterator<'a>> {
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
impl<'a, MACIterator, LabelIterator> TryIntoCtx for AWDLTLV<'a, MACIterator, LabelIterator>
where
    LabelIterator: IntoIterator<Item = AWDLStr<'a>> + Clone,
    <LabelIterator as IntoIterator>::IntoIter: Clone,
    MACIterator: IntoIterator<Item = MACAddress> + ExactSizeIterator + Clone,
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
pub type DefaultAWDLTLV<'a> = AWDLTLV<'a, ReadMACIterator<'a>, ReadLabelIterator<'a>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// A container for the TLVs in an action frame.
pub struct ReadTLVs<'a> {
    bytes: &'a [u8],
}
impl<'a> ReadTLVs<'a> {
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }
    /// Get an iterator over [RawAWDLTLV]'s.
    pub fn raw_tlv_iter(&self) -> impl Iterator<Item = RawAWDLTLV<'a>> + '_ {
        repeat(()).scan(0usize, |offset, _| {
            self.bytes.gread::<RawAWDLTLV>(offset).ok()
        })
    }
    /// Check if the TLV type matches and try to parse the TLV.
    fn match_and_parse_tlv<Tlv: AwdlTlv + TryFromCtx<'a, Error = scroll::Error>>(
        &self,
        raw_tlv: RawAWDLTLV<'a>,
    ) -> Option<Tlv> {
        if raw_tlv.tlv_type == Tlv::TLV_TYPE.into_bits() {
            raw_tlv.slice.pread::<Tlv>(0).ok()
        } else {
            None
        }
    }
    /// Get an iterator over matching TLVs.
    pub fn get_tlvs<Tlv: AwdlTlv + TryFromCtx<'a, Error = scroll::Error>>(
        &self,
    ) -> impl Iterator<Item = Tlv> + use<'_, 'a, Tlv> {
        self.raw_tlv_iter()
            .filter_map(|raw_tlv| self.match_and_parse_tlv(raw_tlv))
    }
    /// Get the first matching TLV.
    pub fn get_first_tlv<Tlv: AwdlTlv + TryFromCtx<'a, Error = scroll::Error>>(
        &self,
    ) -> Option<Tlv> {
        self.raw_tlv_iter()
            .find_map(|raw_tlv| self.match_and_parse_tlv(raw_tlv))
    }
}
impl MeasureWith<()> for ReadTLVs<'_> {
    fn measure_with(&self, _ctx: &()) -> usize {
        self.bytes.len()
    }
}
impl TryIntoCtx<()> for ReadTLVs<'_> {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        buf.pwrite(self.bytes, 0)
    }
}
