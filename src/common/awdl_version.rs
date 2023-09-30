#[cfg(feature = "debug")]
use core::fmt::Debug;

use macro_bits::bitfield;

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq)]
    /// A version in AWDL format.
    pub struct AWDLVersion: u8 {
        /// The major version.
        pub major: u8 => 0xf0,

        /// The minor version.
        pub minor: u8 => 0x0f
    }
}
impl PartialOrd for AWDLVersion {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        use core::cmp::Ordering::*;
        let cmp = (self.major.cmp(&other.major), self.minor.cmp(&other.minor));
        match cmp {
            (Less, _) | (Equal, Less) => Some(Less),
            (Equal, Equal) => Some(Equal),
            _ => Some(Greater),
        }
    }
}
#[cfg(feature = "debug")]
impl Debug for AWDLVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&alloc::format!("{}.{}", self.major, self.minor))
    }
}
