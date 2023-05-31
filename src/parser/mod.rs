use core::fmt::Debug;

use alloc::borrow::Cow;

#[derive(Debug)]
pub enum ParserError {
    /// The parser expected more data than it got.
    TooLittleData(usize),
    /// Just like TooLittleData, but more specific.
    HeaderIncomplete(u8),
    /// The expected magic was invalid.
    InvalidMagic,
    /// A value wasn't understood by the parser.
    ValueNotUnderstood,
}

pub trait Read
where
    Self: Sized,
    Self::Error: Debug,
{
    type Error;
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, Self::Error>;
}
pub trait ReadCtx<Ctx>
where
    Self: Sized,
    Self::Error: Debug,
{
    type Error;
    fn from_bytes(
        data: &mut impl ExactSizeIterator<Item = u8>,
        ctx: Ctx,
    ) -> Result<Self, Self::Error>;
}
pub trait Write<'a> {
    fn to_bytes(&self) -> Cow<'a, [u8]>;
}
pub trait ReadFixed<const N: usize>
where
    Self: Sized,
    Self::Error: Debug,
{
    type Error;
    fn from_bytes(data: &[u8; N]) -> Result<Self, Self::Error>;
}
pub trait ReadFixedCtx<const N: usize, Ctx>
where
    Self: Sized,
    Self::Error: Debug,
{
    type Error;
    fn from_bytes(data: &[u8; N], ctx: Ctx) -> Result<Self, Self::Error>;
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
