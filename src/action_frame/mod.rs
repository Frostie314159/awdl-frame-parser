#[cfg(all(not(feature = "std"), feature = "read"))]
use alloc::vec;
#[cfg(not(feature = "std"))]
use alloc::{format, vec::Vec};
use deku::prelude::*;
#[cfg(feature = "read")]
use {
    self::tlv::TLVType,
    deku::bitvec::{BitSlice, Msb0},
};

use self::{tlv::TLV, version::AWDLVersion};

pub mod dns_compression;
pub mod tlv;
pub mod version;

#[cfg_attr(feature = "read", derive(DekuRead))]
#[cfg_attr(feature = "write", derive(DekuWrite))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
#[deku(type = "u8")]
/// The subtype of the AF.
pub enum AWDLActionFrameSubType {
    /// **M**aster **I**ndication **F**rame
    MIF = 3,
    /// **P**eriodic **S**ynchronization **F**rame
    PSF = 0,
}

#[cfg_attr(feature = "read", derive(DekuRead))]
#[cfg_attr(feature = "write", derive(DekuWrite))]
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
#[deku(magic = b"\x08")]
/// An AWDL AF(**A**ction **F**rame).
pub struct AWDLActionFrame {
    /**
     * This is the version of the AWDL protocol.
     * This is, for an unknown reason, always 1.0, the actual version is found in the Version TLV.
     */
    pub awdl_version: AWDLVersion,

    /**
     * This is the subtype of the AF. Options are [MIF](AWDLActionFrameSubType::MIF) and [PSF](AWDLActionFrameSubType::PSF)
     */
    #[deku(pad_bytes_after = "1")]
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
    #[deku(reader = "Self::read_tlvs(deku::rest)")]
    pub tlvs: Vec<TLV>,
}
#[cfg(feature = "read")]
impl AWDLActionFrame {
    fn read_tlvs(rest: &BitSlice<u8, Msb0>) -> Result<(&BitSlice<u8, Msb0>, Vec<TLV>), DekuError> {
        let mut rest = rest;
        let mut tlvs = vec![];
        loop {
            match TLV::read(rest, ()) {
                Ok((rest2, tlv)) => {
                    rest = rest2;
                    tlvs.push(tlv);
                }
                Err(DekuError::Parse(_)) | Err(DekuError::Incomplete(_)) => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok((rest, tlvs))
    }
    pub fn get_tlvs(&self, tlv_type: TLVType) -> Vec<TLV> {
        self.tlvs
            .iter()
            .filter(|tlv| tlv.tlv_type == tlv_type)
            .map(|tlv| tlv.clone())
            .collect()
    }
}
