#[cfg(feature = "debug")]
use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq)]
/// A version in AWDL format.
pub struct AWDLVersion {
    /// The major version.
    pub major: u8,

    /// The minor version.
    pub minor: u8,
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
#[cfg(feature = "read")]
impl From<u8> for AWDLVersion {
    fn from(value: u8) -> Self {
        Self {
            major: (value >> 4) & 0xf,
            minor: value & 0xf,
        }
    }
}
#[cfg(feature = "write")]
impl From<AWDLVersion> for u8 {
    fn from(value: AWDLVersion) -> Self {
        (value.major << 4) | value.minor
    }
}
