use macro_bits::serializable_enum;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

#[cfg(feature = "debug")]
use core::fmt::Debug;
use core::{fmt::Debug, iter::repeat};

use crate::{
    common::LabelIterator,
    tlvs::{dns_sd::ReadValueIterator, sync_elect::ReadMACIterator, AWDLTLV},
};

serializable_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub enum AWDLActionFrameSubType: u8 {
        #[default]
        /// **P**eriodic **S**ynchronization **F**rame
        PSF => 0x00,
        /// **M**aster **I**ndication **F**rame
        MIF => 0x03
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// An AWDL AF(**A**ction **F**rame).
pub struct AWDLActionFrame<'a> {
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
    pub tagged_data: &'a [u8],
}
impl<'a> AWDLActionFrame<'a> {
    pub fn get_named_tlvs(
        &'a self,
    ) -> impl Iterator<
        Item = AWDLTLV<'a, ReadMACIterator<'a>, LabelIterator<'a>, ReadValueIterator<'a>>,
    > + Clone {
        repeat(()).scan(0, |offset, _| self.tagged_data.gread(offset).ok())
    }
}
/* impl Debug for AWDLActionFrame<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let debuged_list = f.debug_list().entries(self.get_named_tlvs()).finish();
        f.debug_struct("AWDLActionFrame")
            .field("subtype", &self.subtype)
            .field("phy_tx_time", &self.phy_tx_time)
            .field("target_tx_time", &self.target_tx_time)
            .field(
                "tagged_data",
                &f.debug_list().entries(self.get_named_tlvs()).finish(),
            )
            .finish()
    }
} */
impl MeasureWith<()> for AWDLActionFrame<'_> {
    fn measure_with(&self, _ctx: &()) -> usize {
        12 + self.tagged_data.len()
    }
}

impl<'a> TryFromCtx<'a> for AWDLActionFrame<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        if from.gread::<u8>(&mut offset)? != 0x8u8 {
            return Err(scroll::Error::BadInput {
                size: offset,
                msg: "AF didn't start with 0x8.",
            });
        }
        if from.gread::<u8>(&mut offset)? != 0x10u8 {
            return Err(scroll::Error::BadInput {
                size: offset,
                msg: "AF header version wasn't 1.0.",
            });
        }
        let subtype = AWDLActionFrameSubType::from_representation(from.gread(&mut offset)?);
        offset += 1;

        let phy_tx_time = from.gread_with(&mut offset, Endian::Little)?;
        let target_tx_time = from.gread_with(&mut offset, Endian::Little)?;
        let tags = &from[offset..];

        Ok((
            Self {
                subtype,
                phy_tx_time,
                target_tx_time,
                tagged_data: tags,
            },
            offset,
        ))
    }
}
impl<'a> TryIntoCtx for AWDLActionFrame<'a> {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        buf.gwrite(8u8, &mut offset)?;
        buf.gwrite(0x10u8, &mut offset)?;
        buf.gwrite(self.subtype.to_representation(), &mut offset)?;
        offset += 1;
        buf.gwrite_with(self.phy_tx_time, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.target_tx_time, &mut offset, Endian::Little)?;
        buf.gwrite(self.tagged_data, &mut offset)?;
        Ok(offset)
    }
}
#[cfg(test)]
#[test]
fn test_action_frame() {
    use alloc::vec;

    let packet_bytes = include_bytes!("../test_bins/mif.bin");
    let parsed_af = packet_bytes.pread::<AWDLActionFrame<'_>>(0).unwrap();
    let mut buf = vec![0; parsed_af.measure_with(&())];
    assert_eq!(buf.len(), packet_bytes.len());
    buf.pwrite(parsed_af, 0).unwrap();
    assert_eq!(buf, packet_bytes);
}
