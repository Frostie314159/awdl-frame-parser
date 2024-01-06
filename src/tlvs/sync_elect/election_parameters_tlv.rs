use mac_parser::MACAddress;
use scroll::{
    ctx::{MeasureWith, SizeWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
/// A TLV describing the election parameters of a peer.
pub struct ElectionParametersTLV {
    /// Unknown
    pub flags: u8,

    /// Unknown
    pub id: u16,

    /// Distance to the mesh master
    pub distance_to_master: u8,

    /// Address of the master
    pub master_address: MACAddress,

    /// Self metric of the master
    pub master_metric: u32,

    /// Own self metric
    pub self_metric: u32,
}
impl ElectionParametersTLV {
    pub const fn size_in_bytes() -> usize {
        21
    }
}
impl SizeWith for ElectionParametersTLV {
    fn size_with(_ctx: &()) -> usize {
        Self::size_in_bytes()
    }
}
impl MeasureWith<()> for ElectionParametersTLV {
    fn measure_with(&self, _ctx: &()) -> usize {
        Self::size_in_bytes()
    }
}
impl<'a> TryFromCtx<'a> for ElectionParametersTLV {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;

        let flags = from.gread(&mut offset)?;
        let id = from.gread_with(&mut offset, Endian::Little)?;
        let distance_to_master = from.gread(&mut offset)?;
        offset += 1;
        let master_address = MACAddress::new(from.gread(&mut offset)?);
        let master_metric = from.gread_with(&mut offset, Endian::Little)?;
        let self_metric = from.gread_with(&mut offset, Endian::Little)?;

        Ok((
            Self {
                flags,
                id,
                distance_to_master,
                master_address,
                master_metric,
                self_metric,
            },
            offset,
        ))
    }
}
impl TryIntoCtx for ElectionParametersTLV {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;
        buf.gwrite(self.flags, &mut offset)?;
        buf.gwrite_with(self.id, &mut offset, Endian::Little)?;
        buf.gwrite(self.distance_to_master, &mut offset)?;
        offset += 1;
        buf.gwrite(self.master_address.as_slice(), &mut offset)?;
        buf.gwrite_with(self.master_metric, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.self_metric, &mut offset, Endian::Little)?;

        Ok(offset)
    }
}

#[cfg(test)]
#[test]
fn test_election_parameters_tlv() {
    let bytes = &include_bytes!("../../../test_bins/election_parameters_tlv.bin")[3..];

    let election_parameters_tlv = bytes.pread::<ElectionParametersTLV>(0).unwrap();

    assert_eq!(
        election_parameters_tlv,
        ElectionParametersTLV {
            flags: 0x00,
            id: 0x00,
            distance_to_master: 0x02,
            master_address: [0x3a, 0xb4, 0x08, 0x6e, 0x66, 0x3d].into(),
            master_metric: 541,
            self_metric: 60
        }
    );
    let mut buf = [0x00; ElectionParametersTLV::size_in_bytes()];
    buf.pwrite(election_parameters_tlv, 0).unwrap();
    assert_eq!(buf, bytes);
}
