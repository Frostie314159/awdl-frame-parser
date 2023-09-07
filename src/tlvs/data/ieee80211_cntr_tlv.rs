use bin_utils::*;
use tlv_rs::TLV;

pub type IEEE80211TLV = TLV<u8, u8, u16, false>;

/// This TLV just encapsulates an IEEE802.11 TLV.
///
/// In reality, this just contains an EHT capabilities TLV, but for future compatibility we'll just make it do this now.
/// Maybe there will be a parser for IEEE802.11 frames, relying on bin-utils and tlv-rs in the future(foreshadowing).
pub struct IEEE80211ContainerTLV {
    pub tlv: IEEE80211TLV,
}
#[cfg(feature = "read")]
impl Read for IEEE80211ContainerTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        Ok(Self {
            tlv: IEEE80211TLV::from_bytes(data)?,
        })
    }
}
#[cfg(feature = "write")]
impl Write for IEEE80211ContainerTLV {
    fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        self.tlv.to_bytes()
    }
}
