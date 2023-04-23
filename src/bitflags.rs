pub fn read_bitflags<T>(rest: &BitSlice<u8, Msb0>) -> Result<(&BitSlice<u8, Msb0>, T), DekuError>
where
    T: BitFlags + Clone,
    T::Bits: for<'a> DekuRead<'a>,
{
    let (rest, value) = T::Bits::read(rest, ())?;
    Ok((
        rest,
        T::from_bits(value).ok_or(DekuError::Unexpected("from_bits was None.".to_string()))?,
    ))
}
pub fn write_bitflags<T>(output: &mut BitVec<u8, Msb0>, bitflags: &T) -> Result<(), DekuError>
where
    T: BitFlags + Clone,
    T::Bits: DekuWrite,
{
    let bits = bitflags.bits();
    bits.write(output, ())
}
