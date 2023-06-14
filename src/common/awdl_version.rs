use bin_utils::*;

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
impl ReadFixed<1> for AWDLVersion {
    fn from_bytes(data: &[u8; 1]) -> Result<Self, ParserError> {
        Ok(Self {
            major: (data[0] >> 4) & 0xf,
            minor: data[0] & 0xf,
        })
    }
}
#[cfg(feature = "write")]
impl WriteFixed<1> for AWDLVersion {
    fn to_bytes(&self) -> [u8; 1] {
        [(self.major << 4) | self.minor]
    }
}
