use bin_utils::*;

use crate::common::{bit, check_bit, set_bit};

pub const CHANNEL_6: u16 = bit!(0);
pub const CHANNEL_44: u16 = bit!(1);
pub const CHANNEL_149: u16 = bit!(2);

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Default, Clone)]
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
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq)]
pub enum DataPathChannel {
    SingleChannel {
        channel: u8,
    },
    ChannelMap {
        channel_6: bool,
        channel_44: bool,
        channel_149: bool,
    },
}
impl DataPathChannel {
    pub fn from_u16(value: u16, single_channel: bool) -> Self {
        if single_channel {
            Self::SingleChannel {
                channel: value as u8,
            }
        } else {
            Self::ChannelMap {
                channel_6: check_bit!(value, CHANNEL_6),
                channel_44: check_bit!(value, CHANNEL_44),
                channel_149: check_bit!(value, CHANNEL_149),
            }
        }
    }
    pub fn as_u16(&self) -> u16 {
        match *self {
            DataPathChannel::SingleChannel { channel } => channel as u16,
            DataPathChannel::ChannelMap {
                channel_6,
                channel_44,
                channel_149,
            } => {
                let mut channel_map = 0;
                set_bit!(channel_map, CHANNEL_6, channel_6);
                set_bit!(channel_map, CHANNEL_44, channel_44);
                set_bit!(channel_map, CHANNEL_149, channel_149);
                channel_map
            }
        }
    }
}
