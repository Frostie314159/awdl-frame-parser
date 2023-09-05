mod awdl_dns_compression;
mod awdl_dns_name;
mod awdl_str;
mod awdl_version;

pub use awdl_dns_compression::AWDLDnsCompression;
pub use awdl_dns_name::AWDLDnsName;
pub use awdl_str::AWDLStr;
pub use awdl_version::AWDLVersion;

#[cfg(feature = "read")]
use core::ops::{BitAnd, Shl};
#[cfg(feature = "read")]
use num_integer::Integer;

#[cfg(feature = "read")]
#[inline]
pub(crate) fn process_bitmask<T>(bitmask: &'_ T) -> impl Iterator<Item = (bool, u8)> + '_
where
    T: Integer + BitAnd<<T as Shl>::Output> + Copy + From<u8> + Shl,
    <T as Shl>::Output: Copy,
    <T as BitAnd<<T as Shl>::Output>>::Output: PartialEq<T>,
{
    (0..(core::mem::size_of::<T>() * 8) as u8)
        .map(|bit| {
            let mask = T::from(1) << T::from(bit);
            *bitmask & mask != T::from(0)
        })
        .zip(0..)
}
macro_rules! bit {
    ($index:expr) => {
        (1 << $index)
    };
    ($($index:expr), +) => {
        $(bit!($index) | )+ 0
    };
}
macro_rules! check_bit {
    ($flags:expr, $mask:expr) => {
        ($flags & $mask != 0x0)
    };
}
macro_rules! set_bit {
    ($flags:expr, $mask:expr) => {
        $flags |= $mask
    };
    ($flags:expr, $mask:expr, $value:expr) => {
        if $value {
            set_bit!($flags, $mask);
        }
    };
}
pub(crate) use {bit, check_bit, set_bit};
