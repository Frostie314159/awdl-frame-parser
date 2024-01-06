use macro_bits::{bit, bitfield, serializable_enum};

serializable_enum! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub enum SmPwSave: u8 {
        #[default]
        Static => 0,
        Dynamic => 1,
        Disabled => 3
    }
}
bitfield! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct HTCapabilitiesInfo: u16 {
        pub ldpc_coding_capability: bool => bit!(0),
        pub support_channel_width: bool => bit!(1),
        pub sm_power_save: SmPwSave => bit!(2,3),
        pub green_field: bool => bit!(4),
        pub short_gi_20mhz: bool => bit!(5),
        pub short_gi_40mhz: bool => bit!(6),
        pub tx_stbc: bool => bit!(7),
        pub rx_stbc: u8 => bit!(8, 9),
        pub delayed_block_ack: bool => bit!(10),
        pub is_max_amsdu_large: bool => bit!(11),
        pub dsss_40mhz: bool => bit!(12),
        pub psmp: bool => bit!(13),
        pub forty_mhz_intolerant: bool => bit!(14),
        pub txop_protection_support: bool => bit!(15)
    }
}
