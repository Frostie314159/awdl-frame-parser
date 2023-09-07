mod misc;

#[cfg(feature = "read")]
use {crate::common::check_bit, try_take::try_take};

use alloc::vec::Vec;
use bin_utils::*;
use mac_parser::MACAddress;
#[cfg(feature = "write")]
use {crate::common::set_bit, alloc::vec};

use crate::{
    common::bit,
    tlvs::{impl_tlv_conversion, TLVType},
};

use self::misc::DataPathChannel;
pub use self::misc::DataPathMisc;

pub const FLAG_INFRA_BSSID_CHANNEL: u16 = bit!(0);
pub const FLAG_INFRA_ADDRESS: u16 = bit!(1);
pub const FLAG_AWDL_ADDRESS: u16 = bit!(2);
pub const FLAG_UMI: u16 = bit!(4);
pub const FLAG_DUALBAND_SUPPORT: u16 = bit!(5);
pub const FLAG_IS_AIRPLAY: u16 = bit!(6);
pub const FLAG_COUNTRY_CODE: u16 = bit!(8);
pub const FLAG_CHANNEL_MAP: u16 = bit!(9);
pub const FLAG_UNICAST_OPTIONS: u16 = bit!(12);
pub const FLAG_IS_RANGEABLE: u16 = bit!(14);
pub const FLAG_EXTENSION_FLAGS: u16 = bit!(15);

pub const EXTENDED_FLAGS_LOGTRIGGER_ID: u16 = bit!(0);
pub const EXTENDED_FLAGS_RANGING_DISCOVERY: u16 = bit!(1);
pub const EXTENDED_FLAGS_RLFC: u16 = bit!(2);
pub const EXTENDED_FLAGS_IS_CHANNEL_MAP: u16 = bit!(3);
pub const EXTENDED_FLAGS_SDB: u16 = bit!(4);
pub const EXTENDED_FLAGS_DFS_PROXY_SUPPORT: u16 = bit!(5);
pub const EXTENDED_FLAGS_MISC: u16 = bit!(6);

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Default, Clone)]
pub struct DataPathStateTLV {
    // Normal flags
    pub country_code: Option<[char; 2]>,
    pub channel: Option<DataPathChannel>,
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
    pub sdb: bool,
    pub dfs_proxy_support: bool,
}
#[cfg(feature = "read")]
impl Read for DataPathStateTLV {
    fn from_bytes(data: &mut impl ExactSizeIterator<Item = u8>) -> Result<Self, ParserError> {
        let flags = <u16 as ReadCtx<&Endian>>::from_bytes(data, &Endian::Little)?;

        let mut data_path_state_tlv = DataPathStateTLV::default();

        if check_bit!(flags, FLAG_COUNTRY_CODE) {
            let mut data = try_take(data, 3)
                .map_err(ParserError::TooLittleData)?
                .map(|x| x as char);
            data_path_state_tlv.country_code = Some(data.next_chunk().unwrap());
            let _ = data.next();
        }
        let mut channel = None;
        if check_bit!(flags, FLAG_CHANNEL_MAP) {
            channel = <u16 as ReadCtx<&Endian>>::from_bytes(data, &Endian::Little).ok();
        }
        if check_bit!(flags, FLAG_INFRA_BSSID_CHANNEL) {
            let mut data = try_take(data, 8).map_err(ParserError::TooLittleData)?;
            data_path_state_tlv.infra_bssid_channel = Some((
                <MACAddress as ReadFixed<6>>::from_bytes(&data.next_chunk().unwrap())?,
                <u16 as ReadCtx<&Endian>>::from_bytes(&mut data, &Endian::Little)? as u8,
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
            let umi_options_length = <u16 as ReadCtx<&Endian>>::from_bytes(data, &Endian::Little)?;
            data_path_state_tlv.unicast_options = Some(
                try_take(data, umi_options_length as usize)
                    .map_err(ParserError::TooLittleData)?
                    .collect(),
            );
        }
        if check_bit!(flags, FLAG_UMI) {
            data_path_state_tlv.umi = Some(<u16 as ReadCtx<&Endian>>::from_bytes(
                data,
                &Endian::Little,
            )?);
        }
        data_path_state_tlv.is_airplay = check_bit!(flags, FLAG_IS_AIRPLAY);
        data_path_state_tlv.is_rangeable = check_bit!(flags, FLAG_IS_RANGEABLE);
        data_path_state_tlv.dualband_support = check_bit!(flags, FLAG_DUALBAND_SUPPORT);
        if check_bit!(flags, FLAG_EXTENSION_FLAGS) {
            let extended_flags = <u16 as ReadCtx<&Endian>>::from_bytes(data, &Endian::Little)?;

            if let Some(channel) = channel {
                data_path_state_tlv.channel = Some(DataPathChannel::from_u16(
                    channel,
                    !check_bit!(extended_flags, EXTENDED_FLAGS_IS_CHANNEL_MAP),
                ));
            }
            data_path_state_tlv.ranging_discovery =
                check_bit!(extended_flags, EXTENDED_FLAGS_RANGING_DISCOVERY);
            data_path_state_tlv.sdb = check_bit!(extended_flags, EXTENDED_FLAGS_SDB);
            data_path_state_tlv.dfs_proxy_support =
                check_bit!(extended_flags, EXTENDED_FLAGS_DFS_PROXY_SUPPORT);
            if check_bit!(extended_flags, EXTENDED_FLAGS_LOGTRIGGER_ID) {
                data_path_state_tlv.log_trigger_id = Some(<u16 as ReadCtx<&Endian>>::from_bytes(
                    data,
                    &Endian::Little,
                )?)
            }
            if check_bit!(extended_flags, EXTENDED_FLAGS_RLFC) {
                data_path_state_tlv.rlfc = Some(<u32 as ReadCtx<&Endian>>::from_bytes(
                    data,
                    &Endian::Little,
                )?);
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
#[cfg(feature = "write")]
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
        if let Some(channel) = self.channel {
            set_bit!(flags, FLAG_CHANNEL_MAP);
            bytes.extend(channel.as_u16().to_le_bytes());
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
            EXTENDED_FLAGS_IS_CHANNEL_MAP,
            self.channel
                .is_some_and(|x| matches!(x, DataPathChannel::SingleChannel { .. }))
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
impl_tlv_conversion!(false, DataPathStateTLV, TLVType::DataPathState, 2);
#[test]
fn test_data_path_state_tlv() {
    let bytes = include_bytes!("../../../../test_bins/data_path_state_tlv.bin")[3..].to_vec();
    let _ = DataPathStateTLV::from_bytes(&mut bytes.iter().copied()).unwrap();
}
