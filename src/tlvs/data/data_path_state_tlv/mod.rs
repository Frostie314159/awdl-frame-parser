mod misc;

use core::iter::repeat;

#[cfg(feature = "read")]
use try_take::try_take;

#[cfg(feature = "write")]
use alloc::vec;
use bin_utils::*;
use mac_parser::MACAddress;
use macro_bits::{bit, bitfield};

use crate::tlvs::{impl_tlv_conversion, TLVType};

pub use self::misc::DataPathStats;
use self::misc::{DataPathChannel, UnicastOptions};

bitfield! {
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Default, PartialEq)]
    struct DataPathFlags: u16 {
        pub infra_bssid_channel: bool => bit!(0),
        pub infra_address: bool => bit!(1),
        pub awdl_address: bool => bit!(2),
        pub is_umi: bool => bit!(4),
        pub dualband_support: bool => bit!(5),
        pub airplay_sink: bool => bit!(6),
        pub follow_channel_sequence: bool => bit!(7),
        pub country_code: bool => bit!(8),
        pub channel_map: bool => bit!(9),
        pub airplay_solo_mode: bool => bit!(10),
        pub umi_support: bool => bit!(11),
        pub unicast_options: bool => bit!(12),
        pub is_realtime: bool => bit!(13),
        pub rangeable: bool => bit!(14),
        pub extension_flags: bool => bit!(15)
    }
}
bitfield! {
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Default, PartialEq)]
    struct DataPathExtendedFlags: u32 {
        pub ranging_discovery_supported: bool => bit!(0),
        pub log_trigger_id: bool => bit!(1),
        pub rlfc: bool => bit!(2),
        pub dynamic_sdb_active: bool => bit!(4),
        pub misc: bool => bit!(5),
        pub dfs_proxy_supported: bool => bit!(6),
        pub high_efficiency_supported: bool => bit!(8)
    }
}

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Default, PartialEq)]
pub struct DataPathStateTLV {
    // Normal flags
    pub country_code: Option<[char; 2]>,
    pub channel: Option<DataPathChannel>,
    pub infra_bssid_channel: Option<(MACAddress, u8)>,
    pub infra_address: Option<MACAddress>,
    pub awdl_address: Option<MACAddress>,
    pub unicast_options: Option<UnicastOptions>,

    pub airplay_sink: bool,
    pub airplay_solo_mode: bool,
    pub rangeable: bool,
    pub dualband_support: bool,
    pub is_realtime: bool,

    pub rlfc: Option<u32>,
    pub log_trigger_id: Option<u16>,
    pub misc: Option<DataPathStats>,

    pub ranging_discovery_supported: bool,
    pub dynamic_sdb_supported: bool,
    pub dfs_proxy_supported: bool,
}
#[cfg(feature = "read")]
impl Read for DataPathStateTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let flags = DataPathFlags::from_representation(<u16 as ReadCtx<&Endian>>::from_bytes(
            data,
            &Endian::Little,
        )?);

        let mut data_path_state_tlv = DataPathStateTLV::default();

        if flags.country_code {
            let mut data = try_take(data, 3)
                .map_err(ParserError::TooLittleData)?
                .map(|x| x as char);
            data_path_state_tlv.country_code = Some(data.next_chunk().unwrap());
            let _ = data.next();
        }
        if flags.channel_map {
            data_path_state_tlv.channel = Some(DataPathChannel::from_u16(<u16 as ReadCtx<&Endian>>::from_bytes(data, &Endian::Little)?));
        }
        if flags.infra_bssid_channel {
            let mut data = try_take(data, 8).map_err(ParserError::TooLittleData)?;
            data_path_state_tlv.infra_bssid_channel = Some((
                <MACAddress as ReadFixed<6>>::from_bytes(&data.next_chunk().unwrap())?,
                <u16 as ReadCtx<&Endian>>::from_bytes(&mut data, &Endian::Little)? as u8,
            ));
        }
        if flags.infra_address {
            data_path_state_tlv.infra_address = Some(MACAddress::from_bytes(
                &try_take(data, 6)
                    .map_err(ParserError::TooLittleData)?
                    .next_chunk()
                    .unwrap(),
            )?);
        }
        if flags.awdl_address {
            data_path_state_tlv.awdl_address = Some(MACAddress::from_bytes(
                &try_take(data, 6)
                    .map_err(ParserError::TooLittleData)?
                    .next_chunk()
                    .unwrap(),
            )?);
        }
        if flags.unicast_options {
            let _ = data.advance_by(2);
                data_path_state_tlv.unicast_options = Some(UnicastOptions::from_representation(
                    u32::from_le_bytes(data.chain(repeat(0)).next_chunk().unwrap()),
                ));
        }
        data_path_state_tlv.airplay_sink = flags.airplay_sink;
        data_path_state_tlv.airplay_solo_mode = flags.airplay_solo_mode;
        data_path_state_tlv.rangeable = flags.rangeable;
        data_path_state_tlv.dualband_support = flags.dualband_support;
        data_path_state_tlv.is_realtime = flags.is_realtime;
        if flags.extension_flags {
            let extended_flags =
                DataPathExtendedFlags::from_representation(<u32 as ReadCtx<&Endian>>::from_bytes(
                    data,
                    &Endian::Little,
                )?);
            data_path_state_tlv.ranging_discovery_supported =
                extended_flags.ranging_discovery_supported;
            data_path_state_tlv.dynamic_sdb_supported = extended_flags.dynamic_sdb_active;
            data_path_state_tlv.dfs_proxy_supported = extended_flags.dfs_proxy_supported;
            if extended_flags.rlfc {
                data_path_state_tlv.rlfc = Some(<u32 as ReadCtx<&Endian>>::from_bytes(
                    data,
                    &Endian::Little,
                )?);
            }
            if extended_flags.log_trigger_id {
                data_path_state_tlv.log_trigger_id = Some(<u16 as ReadCtx<&Endian>>::from_bytes(
                    data,
                    &Endian::Little,
                )?)
            }
            if extended_flags.misc {
                data_path_state_tlv.misc = DataPathStats::from_bytes(
                    &try_take(data, 12)
                        .map_err(ParserError::TooLittleData)?
                        .next_chunk()
                        .unwrap(),
                )
                .ok();
            }
        }

        Ok(data_path_state_tlv)
    }
}
#[cfg(feature = "write")]
impl Write for DataPathStateTLV {
    fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        let mut flags = DataPathFlags::default();
        let mut bytes = vec![];
        if let Some(country_code) = self.country_code {
            flags.country_code = true;

            bytes.extend(
                country_code
                    .into_iter()
                    .map(|x| x as u8)
                    .chain(core::iter::once(0x00)),
            );
        }
        if let Some(channel) = self.channel {
            flags.channel_map = true;
            bytes.extend(channel.as_u16().to_le_bytes());
        }
        if let Some((mac_address, channel)) = self.infra_bssid_channel {
            flags.infra_bssid_channel = true;
            bytes.extend(
                mac_address
                    .into_iter()
                    .chain((channel as u16).to_le_bytes()),
            );
        }
        if let Some(infra_address) = self.infra_address {
            flags.infra_address = true;
            bytes.extend(infra_address.iter());
        }
        if let Some(awdl_address) = self.awdl_address {
            flags.awdl_address = true;
            bytes.extend(awdl_address.iter());
        }
        if let Some(unicast_options) = &self.unicast_options {
            flags.unicast_options = true;
            bytes.extend(
                    4u16
                    .to_le_bytes()
                    .into_iter()
                    .chain(unicast_options.to_representation().to_le_bytes()),
            )
        }
        flags.airplay_sink = self.airplay_sink;
        flags.rangeable = self.rangeable;
        flags.dualband_support = self.dualband_support;
        flags.extension_flags = true;

        let mut extended_flags = DataPathExtendedFlags::default();
        let mut extended_bytes = vec![];

        if let Some(rlfc) = self.rlfc {
            extended_flags.rlfc = true;
            extended_bytes.extend(rlfc.to_le_bytes());
        }
        if let Some(log_trigger_id) = self.log_trigger_id {
            extended_flags.log_trigger_id = true;
            extended_bytes.extend(log_trigger_id.to_le_bytes());
        }
        if let Some(misc) = &self.misc {
            extended_flags.misc = true;
            extended_bytes.extend(misc.to_bytes());
        }
        extended_flags.ranging_discovery_supported = self.ranging_discovery_supported;
        extended_flags.dynamic_sdb_active = self.dynamic_sdb_supported;
        extended_flags.dfs_proxy_supported = self.dfs_proxy_supported;
        flags
            .to_representation()
            .to_le_bytes()
            .into_iter()
            .chain(bytes)
            .chain(extended_flags.to_representation().to_le_bytes())
            .chain(extended_bytes)
            .collect()
    }
}
impl_tlv_conversion!(false, DataPathStateTLV, TLVType::DataPathState, 2);
#[test]
fn test_data_path_state_tlv() {
    let bytes = include_bytes!("../../../../test_bins/data_path_state_tlv.bin")[3..].to_vec();
    let data_path_state = DataPathStateTLV::from_bytes(&mut bytes.iter().copied()).unwrap();
    panic!("{data_path_state:#?}")
}
