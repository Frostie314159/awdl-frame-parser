use ether_type::EtherType;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AWDLDataFrame<P> {
    pub sequence_number: u16,
    pub ether_type: EtherType,
    pub payload: P,
}
impl<P: MeasureWith<()>> MeasureWith<()> for AWDLDataFrame<P> {
    fn measure_with(&self, ctx: &()) -> usize {
        8 + self.payload.measure_with(ctx)
    }
}
impl<'a> TryFromCtx<'a> for AWDLDataFrame<&'a [u8]> {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        let header_data = from.gread::<[u8; 2]>(&mut offset)?;
        if header_data != [0x03, 0x04] {
            return Err(scroll::Error::BadInput {
                size: offset,
                msg: "Unknown header format.",
            });
        }
        let sequence_number = from.gread_with(&mut offset, Endian::Little)?;
        offset += 2;
        let ether_type = EtherType::from_representation(from.gread_with(&mut offset, Endian::Big)?);
        let payload_len = from.len() - offset;
        let payload = from.gread_with(&mut offset, payload_len)?;
        Ok((
            Self {
                sequence_number,
                ether_type,
                payload,
            },
            offset,
        ))
    }
}
impl<P: TryIntoCtx<Error = scroll::Error>> TryIntoCtx for AWDLDataFrame<P> {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;

        buf.gwrite::<[u8; 2]>([0x03, 0x04], &mut offset)?;
        buf.gwrite_with(self.sequence_number, &mut offset, Endian::Little)?;
        offset += 2;
        buf.gwrite_with(
            self.ether_type.to_representation(),
            &mut offset,
            Endian::Big,
        )?;
        buf.gwrite(self.payload, &mut offset)?;

        Ok(offset)
    }
}
#[test]
fn test_data_frame() {
    use alloc::vec;
    let bytes = include_bytes!("../test_bins/data_frame.bin");
    let data_frame = bytes.pread::<AWDLDataFrame<&[u8]>>(0).unwrap();
    assert_eq!(
        data_frame,
        AWDLDataFrame {
            sequence_number: 2981,
            ether_type: EtherType::IPv6,
            payload: &bytes[8..]
        }
    );
    let mut buf = vec![0x00u8; data_frame.measure_with(&())];
    buf.pwrite(data_frame, 0).unwrap();
    assert_eq!(bytes, buf.as_slice());
}
