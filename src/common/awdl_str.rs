use core::ops::{Deref, DerefMut};

use scroll::{
    ctx::{MeasureWith, StrCtx, TryFromCtx, TryIntoCtx},
    Pread, Pwrite,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
/// A string in the format used by AWDL.
/// The characters are preceeded by a length byte.
pub struct AWDLStr<'a>(pub &'a str);
impl<'a> AWDLStr<'a> {
    pub const fn size_in_bytes(&'a self) -> usize {
        self.0.len() + 1
    }
}
impl<'a> MeasureWith<()> for AWDLStr<'a> {
    fn measure_with(&self, _ctx: &()) -> usize {
        self.size_in_bytes()
    }
}
impl<'a> TryFromCtx<'a> for AWDLStr<'a> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;

        let length = from.gread::<u8>(&mut offset)? as usize;
        let str = from.gread_with::<&'a str>(&mut offset, StrCtx::Length(length))?;
        Ok((Self(str), offset))
    }
}
impl<'a> TryIntoCtx for AWDLStr<'a> {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        if self.len() >= u8::MAX as usize {
            return Err(scroll::Error::TooBig {
                size: u8::MAX as usize,
                len: self.len(),
            });
        }
        buf.gwrite(self.0.len() as u8, &mut offset)?;
        buf.gwrite(self.0, &mut offset)?;
        Ok(offset)
    }
}
impl<'a> Deref for AWDLStr<'a> {
    type Target = &'a str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> DerefMut for AWDLStr<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<'a> From<&'a str> for AWDLStr<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}
#[cfg(test)]
#[test]
fn test_awdl_str() {
    use alloc::vec;

    let bytes = [0x06, 0x6c, 0x61, 0x6d, 0x62, 0x64, 0x61].as_slice();
    let string = bytes.pread::<AWDLStr<'_>>(0).unwrap();
    assert_eq!(string, "lambda".into());
    let mut buf = vec![0x00; string.measure_with(&())];
    let _ = buf.pwrite::<AWDLStr<'_>>(string, 0).unwrap();
    assert_eq!(bytes, buf);
}
