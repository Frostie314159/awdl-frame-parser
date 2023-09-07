pub mod ampdu_parameters;
pub mod ht_capabilities_info;

use ampdu_parameters::*;
use bin_utils::*;
use ht_capabilities_info::*;
#[cfg(feature = "read")]
use try_take::try_take;

use crate::tlvs::{impl_tlv_conversion, TLVType};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct HTCapabilitiesTLV {
    pub ht_capabilities_info: HTCapabilitiesInfo,
    pub a_mpdu_parameters: AMpduParameters,
    pub rx_spatial_stream_count: u8,
}
#[cfg(feature = "read")]
impl ReadFixed<8> for HTCapabilitiesTLV {
    fn from_bytes(data: &[u8; 8]) -> Result<Self, ParserError> {
        let mut data = data.into_iter().copied();
        let _ = data.next_chunk::<2>();
        let mut fixed_data = try_take(&mut data, 3).map_err(ParserError::TooLittleData)?;
        let ht_capabilities_info =
            HTCapabilitiesInfo::from(u16::from_le_bytes(fixed_data.next_chunk().unwrap()));
        let a_mpdu_parameters = AMpduParameters::from(fixed_data.next().unwrap());
        let rx_spatial_stream_count = data
            .len()
            .checked_sub(2)
            .ok_or(ParserError::TooLittleData(2))? as u8;

        Ok(Self {
            ht_capabilities_info,
            a_mpdu_parameters,
            rx_spatial_stream_count,
        })
    }
}
#[cfg(feature = "write")]
impl Write for HTCapabilitiesTLV {
    fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        let mut header = [0x00; 5];
        header[2..4].copy_from_slice(
            <HTCapabilitiesInfo as Into<u16>>::into(self.ht_capabilities_info)
                .to_le_bytes()
                .as_slice(),
        );
        header[4] = <AMpduParameters as Into<u8>>::into(self.a_mpdu_parameters);
        header
            .into_iter()
            .chain((0..self.rx_spatial_stream_count).map(|_| 0xff))
            .chain([0x00; 2])
            .collect()
    }
}
impl_tlv_conversion!(true, HTCapabilitiesTLV, TLVType::HTCapabilities, 8);

#[cfg(test)]
#[test]
fn test_ht_capabilities() {
    use alloc::borrow::ToOwned;

    let bytes = include_bytes!("../../../../test_bins/ht_capabilities_tlv.bin");
    assert_eq!(
        HTCapabilitiesInfo::from(0x1u16 | 0xCu16),
        HTCapabilitiesInfo {
            ldpc_coding_capability: true,
            sm_power_save: SmPwSave::Disabled,
            ..Default::default()
        }
    );
    let ht_capabilities_tlv = HTCapabilitiesTLV::from_bytes(&bytes[3..].try_into().unwrap()).unwrap();
    assert_eq!(
        ht_capabilities_tlv,
        HTCapabilitiesTLV {
            ht_capabilities_info: HTCapabilitiesInfo {
                ldpc_coding_capability: true,
                support_channel_width: true,
                sm_power_save: SmPwSave::Disabled,
                short_gi_20mhz: true,
                short_gi_40mhz: true,
                rx_stbc: RxSpatialStreams::One,
                ..Default::default()
            },
            a_mpdu_parameters: AMpduParameters {
                max_a_mpdu_length: MAXAMpduLength::VeryLarge,
                mpdu_density: MpduDensity::Sixteen,
            },
            rx_spatial_stream_count: 2
        }
    );
    assert_eq!(ht_capabilities_tlv.to_bytes(), bytes.as_slice().to_owned());
}
