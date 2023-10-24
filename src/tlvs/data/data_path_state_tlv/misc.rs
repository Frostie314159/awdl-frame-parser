use bin_utils::*;
use macro_bits::{bit, bitfield, check_bit};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, PartialEq)]
pub struct DataPathStats {
    pub msec_since_activation: u32,
    pub aw_seq_counter: u32,
    pub pay_update_coutner: u32,
}
#[cfg(feature = "read")]
impl ReadFixed<12> for DataPathStats {
    fn from_bytes(data: &[u8; 12]) -> Result<Self, ParserError> {
        let mut data = data.iter().copied();
        Ok(DataPathStats {
            msec_since_activation: u32::from_le_bytes(data.next_chunk().unwrap()),
            aw_seq_counter: u32::from_le_bytes(data.next_chunk().unwrap()),
            pay_update_coutner: u32::from_le_bytes(data.next_chunk().unwrap()),
        })
    }
}
#[cfg(feature = "write")]
impl WriteFixed<12> for DataPathStats {
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
    pub fn from_u16(value: u16) -> Self {
        if !check_bit!(value, bit!(0)) {
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
bitfield! {
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Copy, Default, PartialEq)]
    pub struct UnicastOptions: u32 {
        pub start_airplay: bool => bit!(1),
        pub jumpstart_dfs_proxy: bool => bit!(5),
        pub airplay_on_dfs_channel: bool => bit!(6),
        pub start_sidecar: bool => bit!(9),
        pub sidecar_bg_request: bool => bit!(10),
        pub sidecar_fg_request: bool => bit!(11),
        pub stop_sidecar: bool => bit!(12),
        pub start_multi_peer_steering: bool => bit!(13),
        pub start_real_time_mode: bool => bit!(14),
        pub start_airplay_recovery: bool => bit!(16),
        pub start_ht_mode: bool => bit!(17),
        pub stop_ht_mode: bool => bit!(18),
        pub stop_airplay: bool => bit!(19),
        pub failed_multi_peer_steering: bool => bit!(20)
    }
}
