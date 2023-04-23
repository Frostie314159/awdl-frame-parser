use deku::prelude::*;

#[cfg(all(not(feature = "std"), feature = "read"))]
use alloc::format;
#[cfg(all(not(feature = "std"), feature = "write"))]
use alloc::vec::Vec;

#[cfg_attr(feature = "read", derive(DekuRead))]
#[cfg_attr(feature = "write", derive(DekuWrite))]
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
/// A version in AWDL format.
pub struct AWDLVersion {
    /// The major version.
    #[deku(bits = "4")]
    pub major: u8,

    /// The minor version.
    #[deku(bits = "4")]
    pub minor: u8,
}
