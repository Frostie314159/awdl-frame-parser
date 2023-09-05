mod misc;

use alloc::{vec, vec::Vec};
use bin_utils::{Endian, ParserError, Read, ReadCtx, ReadFixed, Write, WriteFixed};
use mac_parser::MACAddress;
use try_take::try_take;

use crate::common::{bit, check_bit, set_bit};

pub use self::misc::DataPathMisc;

const FLAG_INFRA_BSSID_CHANNEL: u16 = bit!(0);
const FLAG_INFRA_ADDRESS: u16 = bit!(1);
const FLAG_AWDL_ADDRESS: u16 = bit!(2);
const FLAG_UMI: u16 = bit!(4);
const FLAG_DUALBAND_SUPPORT: u16 = bit!(5);
const FLAG_IS_AIRPLAY: u16 = bit!(6);
const FLAG_COUNTRY_CODE: u16 = bit!(8);
const FLAG_CHANNEL_MAP: u16 = bit!(9);
const FLAG_UNICAST_OPTIONS: u16 = bit!(12);
const FLAG_IS_RANGEABLE: u16 = bit!(14);
const FLAG_EXTENSION_FLAGS: u16 = bit!(15);

const EXTENDED_FLAGS_LOGTRIGGER_ID: u16 = bit!(0);
const EXTENDED_FLAGS_RANGING_DISCOVERY: u16 = bit!(1);
const EXTENDED_FLAGS_RLFC: u16 = bit!(2);
const EXTENDED_FLAGS_CHANNEL_MAP_CHANGED: u16 = bit!(3);
const EXTENDED_FLAGS_SDB: u16 = bit!(4);
const EXTENDED_FLAGS_DFS_PROXY_SUPPORT: u16 = bit!(5);
const EXTENDED_FLAGS_MISC: u16 = bit!(6);

const CHANNEL_6: u16 = bit!(0);
const CHANNEL_44: u16 = bit!(1);
const CHANNEL_149: u16 = bit!(2);

#[derive(Default, Debug, Clone)]
pub struct DataPathStateTLV {
    // Normal flags
    pub country_code: Option<[char; 2]>,
    pub channel_map: Option<(bool, bool, bool)>,
    pub infra_bssid_channel: Option<(MACAddress, u8)>,
    pub infra_address: Option<MACAddress>,
    pub awdl_address: Option<MACAddress>,
    pub unicast_options: Option<Vec<u8>>,
    pub umi: Option<u16>,

    pub is_airplay: bool,
    pub is_rangeable: bool,
    pub dualband_support: bool,

    pub log_trigger_id: Option<u16>,
    pub rlfc: Option<u32>,
    pub misc: Option<DataPathMisc>,

    pub ranging_discovery: bool,
    pub channel_map_changed: bool,
    pub sdb: bool,
    pub dfs_proxy_support: bool,
}
#[cfg(feature = "read")]
impl Read for DataPathStateTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let flags = u16::from_bytes(data, &Endian::Little)?;

        let mut data_path_state_tlv = DataPathStateTLV::default();

        if check_bit!(flags, FLAG_COUNTRY_CODE) {
            let mut data = try_take(data, 3)
                .map_err(ParserError::TooLittleData)?
                .map(|x| x as char);
            data_path_state_tlv.country_code = Some(data.next_chunk().unwrap());
            let _ = data.next();
        }
        if check_bit!(flags, FLAG_CHANNEL_MAP) {
            let channel_map = u16::from_bytes(data, &Endian::Little)?;
            data_path_state_tlv.channel_map = Some((
                check_bit!(channel_map, CHANNEL_6),
                check_bit!(channel_map, CHANNEL_44),
                check_bit!(channel_map, CHANNEL_149),
            ));
        }
        if check_bit!(flags, FLAG_INFRA_BSSID_CHANNEL) {
            let mut data = try_take(data, 8).map_err(ParserError::TooLittleData)?;
            data_path_state_tlv.infra_bssid_channel = Some((
                <MACAddress as ReadFixed<6>>::from_bytes(&data.next_chunk().unwrap())?,
                u16::from_bytes(&mut data, &Endian::Little)? as u8,
            ));
        }
        if check_bit!(flags, FLAG_INFRA_ADDRESS) {
            data_path_state_tlv.infra_address = Some(MACAddress::from_bytes(
                &try_take(data, 6)
                    .map_err(ParserError::TooLittleData)?
                    .next_chunk()
                    .unwrap(),
            )?);
        }
        if check_bit!(flags, FLAG_AWDL_ADDRESS) {
            data_path_state_tlv.awdl_address = Some(MACAddress::from_bytes(
                &try_take(data, 6)
                    .map_err(ParserError::TooLittleData)?
                    .next_chunk()
                    .unwrap(),
            )?);
        }
        if check_bit!(flags, FLAG_UNICAST_OPTIONS) {
            let umi_options_length = u16::from_bytes(data, &Endian::Little)?;
            data_path_state_tlv.unicast_options = Some(
                try_take(data, umi_options_length as usize)
                    .map_err(ParserError::TooLittleData)?
                    .collect(),
            );
        }
        if check_bit!(flags, FLAG_UMI) {
            data_path_state_tlv.umi = Some(u16::from_bytes(data, &Endian::Little)?);
        }
        data_path_state_tlv.is_airplay = check_bit!(flags, FLAG_IS_AIRPLAY);
        data_path_state_tlv.is_rangeable = check_bit!(flags, FLAG_IS_RANGEABLE);
        data_path_state_tlv.dualband_support = check_bit!(flags, FLAG_DUALBAND_SUPPORT);
        if check_bit!(flags, FLAG_EXTENSION_FLAGS) {
            let extended_flags = u16::from_bytes(data, &Endian::Little)?;
            data_path_state_tlv.ranging_discovery =
                check_bit!(extended_flags, EXTENDED_FLAGS_RANGING_DISCOVERY);
            data_path_state_tlv.channel_map_changed =
                check_bit!(extended_flags, EXTENDED_FLAGS_CHANNEL_MAP_CHANGED);
            data_path_state_tlv.sdb = check_bit!(extended_flags, EXTENDED_FLAGS_SDB);
            data_path_state_tlv.dfs_proxy_support =
                check_bit!(extended_flags, EXTENDED_FLAGS_DFS_PROXY_SUPPORT);
            if check_bit!(extended_flags, EXTENDED_FLAGS_LOGTRIGGER_ID) {
                data_path_state_tlv.log_trigger_id = Some(u16::from_bytes(data, &Endian::Little)?)
            }
            if check_bit!(extended_flags, EXTENDED_FLAGS_RLFC) {
                data_path_state_tlv.rlfc = Some(u32::from_bytes(data, &Endian::Little)?);
            }
            if check_bit!(extended_flags, EXTENDED_FLAGS_MISC) {
                data_path_state_tlv.misc = DataPathMisc::from_bytes(
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
impl Write for DataPathStateTLV {
    fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        let mut flags = 0u16;
        let mut bytes = vec![];
        if let Some(country_code) = self.country_code {
            set_bit!(flags, FLAG_COUNTRY_CODE);

            bytes.extend(
                country_code
                    .into_iter()
                    .map(|x| x as u8)
                    .chain(core::iter::once(0x00)),
            );
        }
        if let Some((channel_6, channel_44, channel_149)) = self.channel_map {
            set_bit!(flags, FLAG_CHANNEL_MAP);

            let mut channel_map = 0u16;

            set_bit!(channel_map, CHANNEL_6, channel_6);
            set_bit!(channel_map, CHANNEL_44, channel_44);
            set_bit!(channel_map, CHANNEL_149, channel_149);

            bytes.extend(channel_map.to_le_bytes())
        }
        if let Some((mac_address, channel)) = self.infra_bssid_channel {
            set_bit!(flags, FLAG_INFRA_BSSID_CHANNEL);

            bytes.extend(
                mac_address
                    .into_iter()
                    .chain((channel as u16).to_le_bytes()),
            );
        }
        if let Some(infra_address) = self.infra_address {
            set_bit!(flags, FLAG_INFRA_ADDRESS);

            bytes.extend(infra_address.iter());
        }
        if let Some(awdl_address) = self.awdl_address {
            set_bit!(flags, FLAG_AWDL_ADDRESS);

            bytes.extend(awdl_address.iter());
        }
        if let Some(unicast_options) = &self.unicast_options {
            set_bit!(flags, FLAG_UNICAST_OPTIONS);

            bytes.extend(
                (unicast_options.len() as u16)
                    .to_le_bytes()
                    .into_iter()
                    .chain(unicast_options.iter().copied()),
            )
        }
        if let Some(umi) = self.umi {
            set_bit!(flags, FLAG_UMI);

            bytes.extend(umi.to_le_bytes());
        }
        set_bit!(flags, FLAG_IS_AIRPLAY, self.is_airplay);
        set_bit!(flags, FLAG_IS_RANGEABLE, self.is_rangeable);
        set_bit!(flags, FLAG_DUALBAND_SUPPORT, self.dualband_support);
        set_bit!(flags, FLAG_EXTENSION_FLAGS);

        let mut extended_flags = 0u16;
        let mut extended_bytes = vec![];

        if let Some(log_trigger_id) = self.log_trigger_id {
            set_bit!(extended_flags, EXTENDED_FLAGS_LOGTRIGGER_ID);

            extended_bytes.extend(log_trigger_id.to_le_bytes());
        }
        if let Some(rlfc) = self.rlfc {
            set_bit!(extended_flags, EXTENDED_FLAGS_RLFC);

            extended_bytes.extend(rlfc.to_le_bytes());
        }
        if let Some(misc) = &self.misc {
            set_bit!(extended_flags, EXTENDED_FLAGS_MISC);

            extended_bytes.extend(misc.to_bytes());
        }
        set_bit!(
            extended_flags,
            EXTENDED_FLAGS_RANGING_DISCOVERY,
            self.ranging_discovery
        );
        set_bit!(
            extended_flags,
            EXTENDED_FLAGS_CHANNEL_MAP_CHANGED,
            self.channel_map_changed
        );
        set_bit!(extended_flags, EXTENDED_FLAGS_SDB, self.sdb);
        set_bit!(
            extended_flags,
            EXTENDED_FLAGS_DFS_PROXY_SUPPORT,
            self.dfs_proxy_support
        );
        flags
            .to_le_bytes()
            .into_iter()
            .chain(bytes)
            .chain(extended_flags.to_le_bytes())
            .chain(extended_bytes)
            .collect()
    }
}
#[test]
fn test_data_path_state_tlv() {
    let bytes = include_bytes!("../../../../test_bins/data_path_state_tlv.bin")[3..].to_vec();
    let _ = DataPathStateTLV::from_bytes(&mut bytes.iter().copied()).unwrap();
}
