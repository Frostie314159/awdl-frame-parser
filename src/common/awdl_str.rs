use core::ops::{Deref, DerefMut};

use bin_utils::*;

use alloc::borrow::Cow;
#[cfg(feature = "read")]
use try_take::try_take;

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, PartialEq, Eq)]
/// A string in the format used by AWDL.
/// The characters are preceeded by a length byte.
pub struct AWDLStr<'a> {
    string: Cow<'a, str>,
}
impl AWDLStr<'_> {
    #[inline]
    /// Returns the string as an iterator without reallocating.
    pub fn iter(&self) -> impl Iterator<Item = u8> + '_ {
        let chars = self.string.chars().map(|x| x as u8);
        core::iter::once(self.string.len() as u8).chain(chars)
    }
    #[inline]
    /// Returns the length of the string in bytes, including the length byte.
    pub fn total_len(&self) -> usize {
        self.len() + 1
    }
}
#[cfg(feature = "read")]
impl Read for AWDLStr<'_> {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let length = data.next().ok_or(ParserError::HeaderIncomplete(1))? as usize;
        let data = try_take(data, length).map_err(ParserError::TooLittleData)?;
        Ok(Self {
            string: data.map(|x| x as char).collect(),
        })
    }
}
#[cfg(feature = "write")]
impl<'a> Write<'a> for AWDLStr<'a> {
    fn to_bytes(&self) -> alloc::borrow::Cow<'a, [u8]> {
        self.iter().collect()
    }
}
impl<'a> Deref for AWDLStr<'a> {
    type Target = Cow<'a, str>;
    fn deref(&self) -> &Self::Target {
        &self.string
    }
}
impl DerefMut for AWDLStr<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.string
    }
}
impl<'a> From<&'a str> for AWDLStr<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            string: value.into(),
        }
    }
}
#[cfg(test)]
#[test]
fn test_awdl_str() {
    use alloc::borrow::ToOwned;

    let bytes = [0x06, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61];
    let string = AWDLStr::from_bytes(&mut bytes.iter().copied()).unwrap();
    assert_eq!(string, "lambda".into());
    assert_eq!(bytes.as_slice().to_owned(), string.to_bytes().into_owned());
}
