use core::fmt::Debug;

use alloc::{borrow::Cow, vec::Vec};

#[derive(Debug)]
pub enum ParserError {
    /// The parser expected more data than it got.
    TooLittleData(usize),
    /// Just like TooLittleData, but more specific.
    HeaderIncomplete(usize),
    /// The expected magic was invalid.
    InvalidMagic,
    /// A value wasn't understood by the parser.
    ValueNotUnderstood,
}

pub trait Read
where
    Self: Sized,
{
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError>;
}
pub trait ReadCtx<Ctx>
where
    Self: Sized,
{
    fn from_bytes(
        data: &mut impl ExactSizeIterator<Item = u8>,
        ctx: Ctx,
    ) -> Result<Self, ParserError>;
}
pub trait Write<'a> {
    fn to_bytes(&self) -> Cow<'a, [u8]>;
}
pub trait ReadFixed<const N: usize>
where
    Self: Sized,
{
    fn from_bytes(data: &[u8; N]) -> Result<Self, ParserError>;
}
pub trait ReadFixedCtx<const N: usize, Ctx>
where
    Self: Sized,
{
    fn from_bytes(data: &[u8; N], ctx: Ctx) -> Result<Self, ParserError>;
}
pub trait WriteFixed<const N: usize>
where
    Self: Sized,
{
    fn to_bytes(&self) -> [u8; N];
}
#[macro_export]
macro_rules! enum_to_int {
    ($a:ty, $b:ty, $($x:expr, $y:path), +) => {
        impl From<$a> for $b {
            fn from(value: $a) -> Self {
                match value {
                    $($x => $y,)+
                    _ => Self::Unknown(value),
                }
            }
        }
        impl From<$b> for $a {
            fn from(value: $b) -> Self {
                match value {
                    $($y => $x,)+
                    <$b>::Unknown(value) => value
                }
            }
        }
    }
}
impl<'a> Read for Cow<'a, str> {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        use alloc::{str, string::String};

        let length = data.next().ok_or(ParserError::TooLittleData(1))? as usize;
        if data.len() < length {
            return Err(ParserError::TooLittleData(length - data.len() + 1));
        }
        let binding = data.by_ref().take(length).collect();
        Ok(match binding {
            Cow::Borrowed(bytes) => match str::from_utf8(bytes) {
                Ok(str_ref) => Cow::Borrowed(str_ref),
                Err(_) => Cow::Owned(String::from_utf8_lossy(bytes).into_owned()),
            },
            Cow::Owned(bytes) => match String::from_utf8(bytes) {
                Ok(string) => Cow::Owned(string),
                Err(err) => Cow::Owned(
                    err.into_bytes()
                        .into_iter()
                        .map(|b| b as char)
                        .collect::<String>(),
                ),
            },
        })
    }
}
impl<'a> ReadCtx<&(u16, &str)> for Cow<'a, str> {
    fn from_bytes(
        data: &mut impl ExactSizeIterator<Item = u8>,
        (length, seperator): &(u16, &str),
    ) -> Result<Self, ParserError> {
        if data.len() < *length as usize {
            return Err(ParserError::TooLittleData(*length as usize - data.len()));
        }
        let mut string_data = data.by_ref().take(*length as usize);
        let mut string = alloc::string::String::new();
        while let Ok(str) = <Cow<str> as Read>::from_bytes(&mut string_data) {
            if !string.is_empty() {
                string += seperator;
            }
            string += &str;
        }
        Ok(string.into())
    }
}
impl<'a> Write<'a> for Cow<'a, str> {
    fn to_bytes(&self) -> Cow<'a, [u8]> {
        if self.contains('\n') {
            let substrs = self
                .split('\n')
                .map(|x| <&str as Into<Cow<str>>>::into(x).to_bytes())
                .collect::<Vec<Cow<[u8]>>>()
                .concat();
            let length = (substrs.len() as u16).to_le_bytes();
            length.iter().chain(substrs.iter()).copied().collect()
        } else {
            let bytes = self.as_bytes();
            [bytes.len() as u8]
                .iter()
                .chain(bytes.iter())
                .copied()
                .collect()
        }
    }
}
