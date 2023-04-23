#[cfg(not(feature = "std"))]
use alloc::string::String;
#[cfg(all(not(feature = "std"), feature = "read"))]
use alloc::{string::ToString, vec::Vec};
#[cfg(feature = "read")]
use deku::{bitvec::BitSlice, ctx::Endian};

#[cfg(feature = "write")]
use deku::bitvec::BitVec;

use deku::{bitvec::Msb0, prelude::*};

#[cfg(feature = "read")]
pub fn read_string(
    rest: &BitSlice<u8, Msb0>,
    len: usize,
) -> Result<(&BitSlice<u8, Msb0>, String), DekuError> {
    let (rest, string) = Vec::<u8>::read(&rest, (len.into(), Endian::Little))?;
    Ok((rest, String::from_utf8_lossy(string.as_ref()).to_string()))
}
#[cfg(feature = "write")]
pub fn write_string(output: &mut BitVec<u8, Msb0>, string: &String) -> Result<(), DekuError> {
    string.as_bytes().write(output, ())
}
