use alloc::vec::Vec;
use bin_utils::*;

use crate::{
    common::process_bitmask,
    tlvs::{impl_tlv_conversion, TLVType},
};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
/// We don't know what these values mean, but we do know how to decode/encode them.
pub struct ServiceParametersTLV {
    /// An increment causes a DNS flush at the peer.
    pub sui: u16,
    /// No idea honestly.
    pub encoded_values: Vec<u8>,
}
#[cfg(feature = "read")]
impl Read for ServiceParametersTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        if data.len() < 9 {
            return Err(ParserError::TooLittleData(9 - data.len()));
        }
        let mut data = data.skip(3); // Padding
        let sui = u16::from_le_bytes(data.next_chunk().unwrap());
        let offsets = u32::from_le_bytes(data.next_chunk().unwrap());
        let encoded_values = process_bitmask(&offsets)
            .filter_map(|(set, bit)| if set { Some(bit << 3) } else { None }) // times 8
            .zip(
                data.flat_map(|x| process_bitmask(&x).next_chunk::<8>().unwrap())
                    .filter_map(|(set, bit)| if set { Some(bit) } else { None }),
            )
            .map(|(offset, value)| offset + value)
            .collect();
        Ok(Self {
            sui,
            encoded_values,
        })
    }
}
#[cfg(feature = "write")]
impl Write for ServiceParametersTLV {
    fn to_bytes(&self) -> Vec<u8> {
        let mut offsets = 0u32;
        let mut values = [0u8; 32];
        self.encoded_values.iter().for_each(|x| {
            let offset = x >> 3;
            offsets |= 1 << offset;
            values[(offset - 1) as usize] |= 1 << (x - (offset << 3));
        });
        [0x00; 3]
            .into_iter()
            .chain(self.sui.to_le_bytes())
            .chain(offsets.to_le_bytes())
            .chain(values.into_iter().filter(|x| x != &0))
            .collect()
    }
}
impl_tlv_conversion!(false, ServiceParametersTLV, TLVType::ServiceParameters, 8);
#[cfg(test)]
#[test]
fn test_service_parameters_tlv() {
    let bytes = include_bytes!("../../../test_bins/service_parameters_tlv.bin")[3..].to_vec();

    let service_parameters_tlv =
        ServiceParametersTLV::from_bytes(&mut bytes.iter().copied()).unwrap();
    assert_eq!(
        service_parameters_tlv,
        ServiceParametersTLV {
            sui: 55,
            encoded_values: alloc::vec![100, 111, 128, 142, 150, 173, 237]
        }
    );
    assert_eq!(bytes, service_parameters_tlv.to_bytes());
}
