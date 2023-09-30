use bin_utils::*;
use macro_bits::{bit, bitfield};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, PartialEq)]
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
bitfield! {
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Copy, Default, Eq, PartialEq)]
    pub struct ChannelMap: u16 {
        pub channel_6: bool => bit!(0),
        pub channel_44: bool => bit!(1),
        pub channel_149: bool => bit!(2)
    }
}
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub enum DataPathChannel {
    SingleChannel { channel: u8 },
    ChannelMap(ChannelMap),
}
impl DataPathChannel {
    pub fn from_u16(value: u16, single_channel: bool) -> Self {
        if single_channel {
            Self::SingleChannel {
                channel: value as u8,
            }
        } else {
            Self::ChannelMap(ChannelMap::from_representation(value))
        }
    }
    pub fn as_u16(&self) -> u16 {
        match *self {
            DataPathChannel::SingleChannel { channel } => channel as u16,
            DataPathChannel::ChannelMap(channel_map) => channel_map.to_representation(),
        }
    }
}
