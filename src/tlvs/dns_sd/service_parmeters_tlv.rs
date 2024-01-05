use core::fmt::Debug;

use macro_bits::{bit, check_bit};
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

pub type ReadValueIterator<'a> = impl IntoIterator<Item = u8> + Clone + 'a;

/// We don't know what these values mean, but we do know how to decode/encode them.
#[derive(Clone)]
pub struct ServiceParametersTLV<I> {
    /// An increment causes a DNS flush at the peer.
    pub sui: u16,
    /// No idea honestly.
    pub encoded_values: I,
}
impl<I> Debug for ServiceParametersTLV<I>
where
    I: IntoIterator<Item = u8> + Clone,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ServiceParametersTLV")
            .field("sui", &self.sui)
            .field_with("encoded_values", |f: &mut core::fmt::Formatter<'_>| {
                f.debug_list().entries(self.encoded_values.clone()).finish()
            })
            .finish()
    }
}
impl<LhsIterator, RhsIterator> PartialEq<ServiceParametersTLV<RhsIterator>>
    for ServiceParametersTLV<LhsIterator>
where
    LhsIterator: IntoIterator<Item = u8> + Clone,
    RhsIterator: IntoIterator<Item = u8> + Clone,
{
    fn eq(&self, other: &ServiceParametersTLV<RhsIterator>) -> bool {
        self.sui == other.sui
            && self
                .encoded_values
                .clone()
                .into_iter()
                .eq(other.encoded_values.clone())
    }
}
impl<I: Clone + Eq + IntoIterator<Item = u8>> Eq for ServiceParametersTLV<I> {}
impl<I> MeasureWith<()> for ServiceParametersTLV<I>
where
    I: IntoIterator<Item = u8> + Clone,
{
    fn measure_with(&self, _ctx: &()) -> usize {
        let mut offsets = 0u32;
        self.encoded_values.clone().into_iter().for_each(|x| {
            offsets |= 1 << (x >> 3);
        });
        9 + offsets.count_ones() as usize
    }
}
impl<'a> TryFromCtx<'a> for ServiceParametersTLV<ReadValueIterator<'a>> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;

        offset += 3;
        let sui = from.gread_with(&mut offset, Endian::Little)?;
        let offsets = from.gread_with::<u32>(&mut offset, Endian::Little)?;
        let encoded_values = (0..31)
            .filter(move |bit| check_bit!(offsets, bit!(bit)))
            .zip(from[offset..].iter())
            .flat_map(|(bit, byte)| {
                let base = bit * 8;
                (0..8)
                    .map(|bit| {
                        if check_bit!(byte, bit!(bit)) {
                            Some(base + bit)
                        } else {
                            None
                        }
                    })
                    .next_chunk::<8>()
                    .unwrap()
            })
            .flatten();
        Ok((
            ServiceParametersTLV {
                sui,
                encoded_values,
            },
            offset,
        ))
    }
}
impl<I> TryIntoCtx for ServiceParametersTLV<I>
where
    I: IntoIterator<Item = u8>,
{
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        offset += 3;

        buf.gwrite_with(self.sui, &mut offset, Endian::Little)?;

        let mut offsets = 0u32;
        let mut values = [0u8; 32];
        self.encoded_values.into_iter().for_each(|x| {
            let offset = x >> 3;
            offsets |= 1 << offset;
            values[(offset - 1) as usize] |= 1 << (x - (offset << 3));
        });
        buf.gwrite_with(offsets, &mut offset, Endian::Little)?;
        for x in values.into_iter().filter(|x| *x != 0) {
            buf.gwrite(x, &mut offset)?;
        }
        Ok(offset)
    }
}
#[cfg(test)]
#[test]
fn test_service_parameters_tlv() {
    use alloc::vec;

    let bytes = &include_bytes!("../../../test_bins/service_parameters_tlv.bin")[3..];

    let service_parameters_tlv = bytes
        .pread::<ServiceParametersTLV<ReadValueIterator>>(0)
        .unwrap();
    assert_eq!(
        service_parameters_tlv,
        ServiceParametersTLV {
            sui: 55,
            encoded_values: [100, 111, 128, 142, 150, 173, 237],
        }
    );
    let mut buf = vec![0x00; service_parameters_tlv.measure_with(&())];
    buf.as_mut_slice()
        .pwrite(service_parameters_tlv, 0)
        .unwrap();
    assert_eq!(buf, bytes);
}
