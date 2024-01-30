use macro_bits::{bit, bitfield, check_bit};
use scroll::{
    ctx::{TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DataPathStats {
    pub msec_since_activation: u32,
    pub aw_seq_counter: u32,
    pub pay_update_coutner: u32,
}
impl TryFromCtx<'_> for DataPathStats {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'_ [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        Ok((
            Self {
                msec_since_activation: from.gread_with(&mut offset, Endian::Little)?,
                aw_seq_counter: from.gread_with(&mut offset, Endian::Little)?,
                pay_update_coutner: from.gread_with(&mut offset, Endian::Little)?,
            },
            offset,
        ))
    }
}
impl TryIntoCtx for DataPathStats {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        buf.gwrite_with(self.msec_since_activation, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.aw_seq_counter, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.pay_update_coutner, &mut offset, Endian::Little)?;
        Ok(offset)
    }
}
bitfield! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct ChannelMap: u16 {
        pub channel_6: bool => bit!(0),
        pub channel_44: bool => bit!(1),
        pub channel_149: bool => bit!(2)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct UnicastOptions: u32 {
        pub start_airplay: bool => bit!(1),
        pub cache_request: bool => bit!(3),
        pub jumpstart_dfs_proxy: bool => bit!(5),
        pub airplay_on_dfs_channel: bool => bit!(6),
        pub start_sidecar: bool => bit!(9),
        pub sidecar_bg_request: bool => bit!(10),
        pub sidecar_fg_request: bool => bit!(11),
        pub stop_sidecar: bool => bit!(12),
        pub start_multi_peer_steering: bool => bit!(13),
        pub start_real_time_mode: bool => bit!(14),
        pub stop_real_time_mode: bool => bit!(15),
        pub start_airplay_recovery: bool => bit!(16),
        pub start_ht_mode: bool => bit!(17),
        pub stop_ht_mode: bool => bit!(18),
        pub stop_airplay: bool => bit!(19),
        pub failed_multi_peer_steering: bool => bit!(20),
        pub unknown: u8 => bit!(21, 22, 23),
        pub start_rtg_ensemble: bool => bit!(24),
        pub stop_rtg_ensemble: bool => bit!(25),
        pub start_airplay_in_rtg_mode: bool => bit!(26),
        pub stop_airplay_in_rtg_mode: bool => bit!(27),
        pub start_sidecar_in_rtg_mode: bool => bit!(28),
        pub stop_sidecar_in_rtg_mode: bool => bit!(29),
        pub start_remote_camera: bool => bit!(30),
        pub stop_remote_camera: bool => bit!(31)
    }
}
