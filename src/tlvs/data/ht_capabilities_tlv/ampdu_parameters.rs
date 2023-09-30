use macro_bits::{bit, bitfield, serializable_enum};

serializable_enum! {
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub enum MAXAMpduLength: u8 {
        /// 8kb
        #[default]
        Small => 0,
        /// 16kb
        Medium => 1,
        /// 32kb
        Large => 2,
        /// 64kb
        VeryLarge => 3
    }
}

serializable_enum! {#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub enum MpduDensity: u8 {
        #[default]
        NoRestriction => 0,
        Quarter => 1,
        Half => 2,
        One => 3,
        Two => 4,
        Four => 5,
        Eight => 6,
        Sixteen => 7
    }
}

bitfield! {
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Copy, PartialEq, Eq, Default)]
    pub struct AMpduParameters: u8 {
        pub max_a_mpdu_length: MAXAMpduLength => bit!(0,1),
        pub mpdu_density: MpduDensity => bit!(2,3,4)
    }
}
