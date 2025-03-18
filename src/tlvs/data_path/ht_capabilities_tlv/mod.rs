pub mod ampdu_parameters;
pub mod ht_capabilities_info;

use ampdu_parameters::*;
use ht_capabilities_info::*;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

use crate::tlvs::{AWDLTLVType, AwdlTlv};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct HTCapabilitiesTLV {
    pub ht_capabilities_info: HTCapabilitiesInfo,
    pub a_mpdu_parameters: AMpduParameters,
    pub rx_spatial_stream_count: u8,
}
impl AwdlTlv for HTCapabilitiesTLV {
    const TLV_TYPE: AWDLTLVType = AWDLTLVType::HTCapabilities;
}
impl MeasureWith<()> for HTCapabilitiesTLV {
    fn measure_with(&self, _ctx: &()) -> usize {
        7 + self.rx_spatial_stream_count as usize
    }
}
impl<'a> TryFromCtx<'a> for HTCapabilitiesTLV {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;

        offset += 2;
        Ok((
            Self {
                ht_capabilities_info: HTCapabilitiesInfo::from_bits(
                    from.gread_with(&mut offset, Endian::Little)?,
                ),
                a_mpdu_parameters: AMpduParameters::from_bits(
                    from.gread_with(&mut offset, Endian::Little)?,
                ),
                rx_spatial_stream_count: (from.len() - offset - 2) as u8,
            },
            offset,
        ))
    }
}
impl TryIntoCtx for HTCapabilitiesTLV {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;

        offset += 2;
        buf.gwrite_with(
            self.ht_capabilities_info.into_bits(),
            &mut offset,
            Endian::Little,
        )?;
        buf.gwrite(self.a_mpdu_parameters.into_bits(), &mut offset)?;
        for _ in 0..self.rx_spatial_stream_count {
            buf.gwrite(0xffu8, &mut offset)?;
        }
        offset += 2;

        Ok(offset)
    }
}
#[cfg(test)]
#[test]
fn test_ht_capabilities() {
    use alloc::vec;

    let bytes = &include_bytes!("../../../../test_bins/ht_capabilities_tlv.bin")[3..];
    assert_eq!(
        HTCapabilitiesInfo::from(0x1u16 | 0xCu16),
        HTCapabilitiesInfo {
            ldpc_coding_capability: true,
            sm_power_save: SmPwSave::Disabled,
            ..Default::default()
        }
    );
    let ht_capabilities_tlv = bytes.pread::<HTCapabilitiesTLV>(0).unwrap();
    assert_eq!(
        ht_capabilities_tlv,
        HTCapabilitiesTLV {
            ht_capabilities_info: HTCapabilitiesInfo {
                ldpc_coding_capability: true,
                support_channel_width: true,
                sm_power_save: SmPwSave::Disabled,
                short_gi_20mhz: true,
                short_gi_40mhz: true,
                rx_stbc: 1,
                ..Default::default()
            },
            a_mpdu_parameters: AMpduParameters {
                max_a_mpdu_length: MAXAMpduLength::VeryLarge,
                mpdu_density: MpduDensity::Sixteen,
            },
            rx_spatial_stream_count: 2
        }
    );
    let mut buf = vec![0x00; ht_capabilities_tlv.measure_with(&())];
    buf.as_mut_slice().pwrite(ht_capabilities_tlv, 0).unwrap();
    assert_eq!(buf, bytes);
}
