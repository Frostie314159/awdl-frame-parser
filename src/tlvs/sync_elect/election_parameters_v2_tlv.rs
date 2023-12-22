use mac_parser::MACAddress;
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
/// Another TLV describing the election parameters of the peer.
pub struct ElectionParametersV2TLV {
    /// MAC address of the master
    pub master_address: MACAddress,

    /// MAC address of the peer this peer is syncing to
    pub sync_address: MACAddress,

    /// Counter value of the master
    pub master_counter: u32,

    /// Distance to the current master
    pub distance_to_master: u32,

    /// Metric of the master
    pub master_metric: u32,

    /// Self metric of the peer
    pub self_metric: u32,

    /// Always zero, but found in some files.
    pub election_id: u32,

    /// Self counter of the peer
    pub self_counter: u32,
}
impl ElectionParametersV2TLV {
    pub const fn size_in_bytes() -> usize {
        40
    }
}
impl MeasureWith<()> for ElectionParametersV2TLV {
    fn measure_with(&self, _ctx: &()) -> usize {
        Self::size_in_bytes()
    }
}
impl<'a> TryFromCtx<'a> for ElectionParametersV2TLV {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;

        let master_address = MACAddress::new(from.gread(&mut offset)?);
        let sync_address = MACAddress::new(from.gread(&mut offset)?);
        let master_counter = from.gread_with(&mut offset, Endian::Little)?;
        let distance_to_master = from.gread_with(&mut offset, Endian::Little)?;
        let master_metric = from.gread_with(&mut offset, Endian::Little)?;
        let self_metric = from.gread_with(&mut offset, Endian::Little)?;
        let election_id = from.gread_with(&mut offset, Endian::Little)?;
        offset += 4;
        let self_counter = from.gread_with(&mut offset, Endian::Little)?;
        Ok((
            Self {
                master_address,
                sync_address,
                master_counter,
                distance_to_master,
                master_metric,
                self_metric,
                election_id,
                self_counter,
            },
            offset,
        ))
    }
}
impl TryIntoCtx for ElectionParametersV2TLV {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;

        buf.gwrite(self.master_address.as_slice(), &mut offset)?;
        buf.gwrite(self.sync_address.as_slice(), &mut offset)?;
        buf.gwrite_with(self.master_counter, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.distance_to_master, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.master_metric, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.self_metric, &mut offset, Endian::Little)?;
        buf.gwrite_with(self.election_id, &mut offset, Endian::Little)?;
        offset += 4;
        buf.gwrite_with(self.self_counter, &mut offset, Endian::Little)?;

        Ok(offset)
    }
}
#[cfg(test)]
#[test]
fn test_election_parameters_v2_tlv() {
    let bytes = &include_bytes!("../../../test_bins/election_parameters_v2_tlv.bin")[3..];

    let election_parameters_v2_tlv = bytes.pread::<ElectionParametersV2TLV>(0).unwrap();

    assert_eq!(
        election_parameters_v2_tlv,
        ElectionParametersV2TLV {
            master_address: [0xce, 0x21, 0x1f, 0x62, 0x21, 0x22].into(),
            sync_address: [0xce, 0x21, 0x1f, 0x62, 0x21, 0x22].into(),
            master_counter: 960,
            distance_to_master: 1,
            master_metric: 650,
            self_metric: 650,
            election_id: 0,
            self_counter: 30,
        }
    );

    let mut buf = [0x00; ElectionParametersV2TLV::size_in_bytes()];
    buf.pwrite(election_parameters_v2_tlv, 0).unwrap();

    assert_eq!(buf, bytes);
}
