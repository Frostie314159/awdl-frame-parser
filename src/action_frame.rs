use alloc::vec::Vec;

use bin_utils::*;

#[cfg(feature = "debug")]
use core::fmt::Debug;

use crate::{
    common::awdl_version::AWDLVersion,
    tlvs::{TLVType, AWDLTLV},
};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
/// The subtype of the AF.
pub enum AWDLActionFrameSubType {
    /// **P**eriodic **S**ynchronization **F**rame
    PSF,
    /// **M**aster **I**ndication **F**rame
    MIF,

    Unknown(u8),
}
enum_to_int! {
    u8,
    AWDLActionFrameSubType,

    0x00,
    AWDLActionFrameSubType::PSF,
    0x03,
    AWDLActionFrameSubType::MIF
}

#[derive(Clone, PartialEq, Eq)]
/// An AWDL AF(**A**ction **F**rame).
pub struct AWDLActionFrame<'a> {
    /**
     * This is the version of the AWDL protocol.
     * This is, for an unknown reason, always 1.0, the actual version is found in the Version TLV.
     */
    pub awdl_version: AWDLVersion,

    /**
     * This is the subtype of the AF. Options are [MIF](AWDLActionFrameSubType::MIF) and [PSF](AWDLActionFrameSubType::PSF)
     */
    pub subtype: AWDLActionFrameSubType,

    /**
     * The time the NIC physically started sending the frame, in μs.
     */
    pub phy_tx_time: u32,
    /**
     * The time the driver send the frame to the NIC, in μs.
     */
    pub target_tx_time: u32,

    //TLVs
    /// The TLVs contained in the action frame.
    pub tlvs: Vec<AWDLTLV<'a>>,
}
impl AWDLActionFrame<'_> {
    pub fn get_tlvs(&self, tlv_type: TLVType) -> Option<Vec<&AWDLTLV>> {
        return Some(
            self.tlvs
                .iter()
                .filter(|tlv| tlv.tlv_type == tlv_type)
                .collect(),
        );
    }
}
#[cfg(feature = "read")]
impl<'a> Read for AWDLActionFrame<'a> {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        if data.len() < 12 {
            return Err(ParserError::HeaderIncomplete(12 - data.len()));
        }

        // Using unwrap is ok now, since we would've already returned if data is shorter than 12 bytes.
        match data.next() {
            Some(0x08) => {}
            None => unreachable!(),
            _ => return Err(ParserError::InvalidMagic),
        };
        let awdl_version = AWDLVersion::from_bytes(&[data.next().unwrap()]).unwrap();

        let subtype = data.next().unwrap().into();
        let _ = data.next();

        let phy_tx_time = u32::from_le_bytes(data.next_chunk().unwrap());
        let target_tx_time = u32::from_le_bytes(data.next_chunk().unwrap());

        let tlvs = <Vec<AWDLTLV<'_>> as Read>::from_bytes(data)?;

        Ok(Self {
            awdl_version,
            subtype,
            phy_tx_time,
            target_tx_time,
            tlvs,
        })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for AWDLActionFrame<'a> {
    fn to_bytes(&self) -> alloc::borrow::Cow<'a, [u8]> {
        let mut header = [0x08u8; 12];
        header[1] = self.awdl_version.to_bytes()[0];
        header[2] = self.subtype.into();
        header[3] = 0x00;
        header[4..8].copy_from_slice(&self.phy_tx_time.to_le_bytes());
        header[8..12].copy_from_slice(&self.target_tx_time.to_le_bytes());
        let body = self.tlvs.to_bytes();
        header.iter().copied().chain(body.iter().copied()).collect()
    }
}
#[cfg(feature = "debug")]
impl Debug for AWDLActionFrame<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AWDLActionFrame")
            .field("awdl_version", &self.awdl_version)
            .field("subtype", &self.subtype)
            .field("phy_tx_time", &self.phy_tx_time)
            .field("target_tx_time", &self.target_tx_time)
            .field(
                "tlvs",
                &self
                    .tlvs
                    .iter()
                    .map(|x| x.tlv_type)
                    .collect::<alloc::borrow::Cow<[TLVType]>>(),
            )
            .finish()
    }
}
#[cfg(test)]
#[test]
fn test_action_frame() {
    let packet_bytes: &[u8] = include_bytes!("../test_bins/mif.bin");

    let frame = AWDLActionFrame::from_bytes(&mut packet_bytes.iter().copied()).unwrap();
    assert_eq!(frame.to_bytes(), packet_bytes);
}
