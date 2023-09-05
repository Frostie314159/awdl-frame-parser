use bin_utils::{ParserError, ReadFixed, WriteFixed};

#[derive(Default, Debug, Clone)]
pub struct DataPathMisc {
    pub msec_since_activation: u32,
    pub aw_seq_counter: u32,
    pub pay_update_coutner: u32,
}
#[cfg(feature = "read")]
impl ReadFixed<12> for DataPathMisc {
    fn from_bytes(data: &[u8; 12]) -> Result<Self, ParserError> {
        let mut data = data.iter().copied();
        Ok(DataPathMisc {
            msec_since_activation: u32::from_le_bytes(data.next_chunk().unwrap()),
            aw_seq_counter: u32::from_le_bytes(data.next_chunk().unwrap()),
            pay_update_coutner: u32::from_le_bytes(data.next_chunk().unwrap()),
        })
    }
}
#[cfg(feature = "write")]
impl WriteFixed<12> for DataPathMisc {
    fn to_bytes(&self) -> [u8; 12] {
        self.msec_since_activation
            .to_le_bytes()
            .into_iter()
            .chain(self.aw_seq_counter.to_le_bytes())
            .chain(self.pay_update_coutner.to_le_bytes())
            .next_chunk()
            .unwrap()
    }
}
