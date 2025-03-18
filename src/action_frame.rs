use macro_bits::serializable_enum;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

use core::{fmt::Debug, time::Duration};

use crate::tlvs::{ReadTLVs, AWDLTLV};

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

#[derive(Clone, PartialEq, Eq)]
/// An AWDL AF(**A**ction **F**rame).
pub struct AWDLActionFrame<I> {
    /// This is the subtype of the AF. Options are [MIF](AWDLActionFrameSubType::MIF) and [PSF](AWDLActionFrameSubType::PSF).
    pub subtype: AWDLActionFrameSubType,

    /// The time the NIC physically started sending the frame, in μs.
    pub phy_tx_time: Duration,

    /// The time the driver send the frame to the NIC, in μs.
    pub target_tx_time: Duration,

    /// The TLVs contained in the action frame.
    pub tagged_data: I,
}
impl<I> AWDLActionFrame<I> {
    /// Calculate the time, between the driver sending the frame to the WNIC and the transmission starting.
    pub fn tx_delta(&self) -> Duration {
        self.phy_tx_time - self.target_tx_time
    }
}
impl<'a, I: Debug, MACIterator, LabelIterator> Debug for AWDLActionFrame<I>
where
    AWDLTLV<'a, MACIterator, LabelIterator>: Debug,
    I: IntoIterator<Item = AWDLTLV<'a, MACIterator, LabelIterator>> + Clone,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("AWDLActionFrame")
            .field("subtype", &self.subtype)
            .field("phy_tx_time", &self.phy_tx_time)
            .field("target_tx_time", &self.target_tx_time)
            .field("tagged_data", &self.tagged_data)
            .finish()
    }
}
impl<I: MeasureWith<()>> MeasureWith<()> for AWDLActionFrame<I>
{
    fn measure_with(&self, ctx: &()) -> usize {
        12 + self
            .tagged_data.measure_with(ctx)
    }
}

impl<'a> TryFromCtx<'a> for AWDLActionFrame<ReadTLVs<'a>> {
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
        let subtype = AWDLActionFrameSubType::from_bits(from.gread(&mut offset)?);
        offset += 1;

        let phy_tx_time =
            Duration::from_micros(from.gread_with::<u32>(&mut offset, Endian::Little)? as u64);
        let target_tx_time =
            Duration::from_micros(from.gread_with::<u32>(&mut offset, Endian::Little)? as u64);
        let tagged_data = ReadTLVs::new(&from[offset..]);

        Ok((
            Self {
                subtype,
                phy_tx_time,
                target_tx_time,
                tagged_data,
            },
            offset,
        ))
    }
}
impl<I: TryIntoCtx<(), Error = scroll::Error>> TryIntoCtx for AWDLActionFrame<I>
{
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        buf.gwrite(8u8, &mut offset)?;
        buf.gwrite(0x10u8, &mut offset)?;
        buf.gwrite(self.subtype.into_bits(), &mut offset)?;
        offset += 1;
        buf.gwrite_with(
            self.phy_tx_time.as_micros() as u32,
            &mut offset,
            Endian::Little,
        )?;
        buf.gwrite_with(
            self.target_tx_time.as_micros() as u32,
            &mut offset,
            Endian::Little,
        )?;
        buf.gwrite(self.tagged_data, &mut offset)?;
        Ok(offset)
    }
}
/// The default awdl action frame returned by reading.
pub type DefaultAWDLActionFrame<'a> = AWDLActionFrame<ReadTLVs<'a>>;
#[cfg(test)]
#[test]
fn test_action_frame() {
    use alloc::vec;

    let packet_bytes = include_bytes!("../test_bins/mif.bin");
    let parsed_af = packet_bytes.pread::<DefaultAWDLActionFrame>(0).unwrap();
    //panic!("{parsed_af:#?}");
    let mut buf = vec![0; parsed_af.measure_with(&())];
    buf.pwrite(parsed_af, 0).unwrap();
    assert_eq!(packet_bytes, buf.as_slice());
}
