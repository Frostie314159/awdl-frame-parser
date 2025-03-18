use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian,
};
use tlv_rs::raw_tlv::RawTLV;

use crate::tlvs::{AWDLTLVType, AwdlTlv};

pub type IEEE80211TLV<'a> = RawTLV<'a, u8, u8>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// This TLV just encapsulates an IEEE802.11 TLV.
///
/// In reality, this just contains a VHT capabilities TLV, but for future compatibility we'll just make it do this now.
/// Maybe there will be a parser for IEEE802.11 frames, relying on bin-utils and tlv-rs in the future(foreshadowing).
pub struct IEEE80211ContainerTLV<'a> {
    pub tlv: IEEE80211TLV<'a>,
}
impl AwdlTlv for IEEE80211ContainerTLV<'_> {
    const TLV_TYPE: AWDLTLVType = AWDLTLVType::IEEE80211Container;
}
impl<'a> MeasureWith<()> for IEEE80211ContainerTLV<'a> {
    fn measure_with(&self, _ctx: &()) -> usize {
        2 + self.tlv.slice.len()
    }
}
impl<'a> TryFromCtx<'a> for IEEE80211ContainerTLV<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        IEEE80211TLV::try_from_ctx(from, Endian::Little).map(|(tlv, offset)| (Self { tlv }, offset))
    }
}
impl<'a> TryIntoCtx for IEEE80211ContainerTLV<'a> {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        IEEE80211TLV::try_into_ctx(self.tlv, buf, Endian::Little)
    }
}
