use core::ops::{Deref, DerefMut};

use bin_utils::*;

use alloc::string::String;
#[cfg(feature = "read")]
use try_take::try_take;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, PartialEq, Eq)]
/// A string in the format used by AWDL.
/// The characters are preceeded by a length byte.
pub struct AWDLStr(String);
impl AWDLStr {
    #[inline]
    /// Returns the string as an iterator without reallocating.
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        let chars = self.chars().map(|x| x as u8);
        core::iter::once(self.len() as u8).chain(chars)
    }
    #[inline]
    /// Returns the length of the string in bytes, including the length byte.
    pub fn total_len(&self) -> usize {
        self.len() + 1
    }
}
#[cfg(feature = "read")]
impl Read for AWDLStr {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let length = data.next().ok_or(ParserError::HeaderIncomplete(1))? as usize;
        let data = try_take(data, length).map_err(ParserError::TooLittleData)?;
        Ok(Self(data.map(|x| x as char).collect()))
    }
}
#[cfg(feature = "write")]
impl Write for AWDLStr {
    fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        self.iter().collect()
    }
}
impl Deref for AWDLStr {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for AWDLStr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<String> for AWDLStr {
    fn from(value: String) -> Self {
        Self(value)
    }
}
impl<'a> From<&'a str> for AWDLStr {
    fn from(value: &'a str) -> Self {
        Self(String::from(value))
    }
}
#[cfg(test)]
#[test]
fn test_awdl_str() {
    let bytes = [0x06, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61];
    let string = AWDLStr::from_bytes(&mut bytes.iter().copied()).unwrap();
    assert_eq!(string, "lambda".into());
    assert_eq!(bytes.to_vec(), string.to_bytes());
}
