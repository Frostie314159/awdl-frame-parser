use alloc::vec::Vec;

use crate::tlvs::AWDLTLV;

pub struct ExtendedHeader {
    pub tlvs: Vec<AWDLTLV>,
    pub header_data: Vec<u8>,
}
pub struct AWDLDataFrame {
    pub sequence_number: u16,
    pub header_type: u16,
    pub ext_header: Option<ExtendedHeader>,
    pub payload: Vec<u8>,
}
