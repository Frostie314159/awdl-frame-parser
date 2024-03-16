mod misc;

use mac_parser::MACAddress;
use macro_bits::{bit, bitfield};
use scroll::{
    ctx::{MeasureWith, TryFromCtx, TryIntoCtx},
    Endian, Pread, Pwrite,
};

pub use self::misc::{DataPathStats, DataPathChannel, UnicastOptions, ChannelMap};

bitfield! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct DataPathFlags: u16 {
        pub infra_bssid_channel_present: bool => bit!(0),
        pub infra_address_present: bool => bit!(1),
        pub awdl_address_present: bool => bit!(2),
        pub rsdb_support: bool => bit!(3),
        pub is_umi: bool => bit!(4),
        pub dualband_support: bool => bit!(5),
        pub airplay_sink: bool => bit!(6),
        pub follow_channel_sequence: bool => bit!(7),
        pub country_code_present: bool => bit!(8),
        pub channel_map_present: bool => bit!(9),
        pub airplay_solo_mode_support: bool => bit!(10),
        pub umi_support: bool => bit!(11),
        pub unicast_options_present: bool => bit!(12),
        pub is_realtime: bool => bit!(13),
        pub rangeable: bool => bit!(14),
        pub extended_flags: bool => bit!(15)
    }
}
bitfield! {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct DataPathExtendedFlags: u16 {
        pub log_trigger_id_present: bool => bit!(0),
        pub ranging_discovery_supported: bool => bit!(1),
        pub rlfc_present: bool => bit!(2),
        pub is_social_channel_map_supported: bool => bit!(3),
        pub dynamic_sdb_active: bool => bit!(4),
        pub stats_present: bool => bit!(5),
        pub dfs_proxy_support: bool => bit!(6),
        pub high_efficiency_support: bool => bit!(8),
        pub is_sidekick_hub: bool => bit!(9),
        pub fast_discovery_active: bool => bit!(10),
        pub wifi_six_e_support: bool => bit!(11),
        pub ultra_low_latency_infra_support: bool => bit!(12),
        pub pro_mode_active: bool => bit!(13),
        pub unknown: u8 => bit!(14, 15)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DataPathStateTLV {
    pub flags: DataPathFlags,
    pub country_code: Option<[char; 2]>,
    pub channel_map: Option<DataPathChannel>,
    pub infra_bssid_channel: Option<(MACAddress, u16)>,
    pub infra_address: Option<MACAddress>,
    pub awdl_address: Option<MACAddress>,
    pub unicast_options: Option<UnicastOptions>,
    pub unicast_options_ext: Option<u32>,

    pub extended_flags: Option<DataPathExtendedFlags>,
    pub rlfc: Option<u32>,
    pub log_trigger_id: Option<u16>,
    pub stats: Option<DataPathStats>,
}
impl DataPathStateTLV {
    pub const fn size_in_bytes(&self) -> usize {
        let mut size = 2;
        if self.country_code.is_some() {
            size += 3;
        }
        if self.channel_map.is_some() {
            size += 2;
        }
        if self.infra_bssid_channel.is_some() {
            size += 8;
        }
        if self.infra_address.is_some() {
            size += 6;
        }
        if self.awdl_address.is_some() {
            size += 6;
        }
        if self.unicast_options.is_some() {
            size += 6;
        }
        if self.unicast_options_ext.is_some() {
            size += 4;
        }
        if self.extended_flags.is_some() {
            size += 2;
        }
        if self.rlfc.is_some() {
            size += 4;
        }
        if self.log_trigger_id.is_some() {
            size += 2;
        }
        if self.stats.is_some() {
            size += 12;
        }
        size
    }
}
impl MeasureWith<()> for DataPathStateTLV {
    fn measure_with(&self, _ctx: &()) -> usize {
        self.size_in_bytes()
    }
}
impl<'a> TryFromCtx<'a> for DataPathStateTLV {
    type Error = scroll::Error;
    fn try_from_ctx(from: &'a [u8], _ctx: ()) -> Result<(Self, usize), Self::Error> {
        let mut offset = 0;
        let flags =
            DataPathFlags::from_bits(from.gread_with(&mut offset, Endian::Little)?);
        let country_code = flags
            .country_code_present
            .then(|| {
                let country_code = from.gread::<[u8; 2]>(&mut offset)?.map(|x| x as char);
                offset += 1;
                Ok::<[char; 2], scroll::Error>(country_code)
            })
            .transpose()?;
        let channel_map = flags
            .channel_map_present
            .then(|| {
                Ok::<DataPathChannel, scroll::Error>(DataPathChannel::from_u16(
                    from.gread_with(&mut offset, Endian::Little)?,
                ))
            })
            .transpose()?;
        let infra_bssid_channel = flags
            .infra_bssid_channel_present
            .then(|| {
                Ok::<(MACAddress, u16), scroll::Error>((
                    from.gread(&mut offset)?,
                    from.gread(&mut offset)?,
                ))
            })
            .transpose()?;
        let infra_address = flags
            .infra_address_present
            .then(|| from.gread(&mut offset))
            .transpose()?;
        let awdl_address = flags
            .awdl_address_present
            .then(|| from.gread(&mut offset))
            .transpose()?;
        let (unicast_options, unicast_options_ext) = flags
            .unicast_options_present
            .then(|| {
                Ok({
                    let unicast_options_length =
                        from.gread_with::<u16>(&mut offset, Endian::Little)?;
                    match unicast_options_length {
                        4 => (
                            Some(UnicastOptions::from_bits(
                                from.gread_with(&mut offset, Endian::Little)?,
                            )),
                            None,
                        ),
                        8 => (
                            Some(UnicastOptions::from_bits(
                                from.gread_with(&mut offset, Endian::Little)?,
                            )),
                            Some(from.gread_with(&mut offset, Endian::Little)?),
                        ),
                        _ => {
                            return Err(scroll::Error::BadInput {
                                size: offset,
                                msg: "Invalid unicast options length.",
                            })
                        }
                    }
                })
            })
            .transpose()?
            .unwrap_or_default();
        // I know, that I'm going to hell for this abomination.
        let (extended_flags, log_trigger_id, rlfc, stats) = flags
            .extended_flags
            .then(|| {
                Ok::<
                    (
                        Option<DataPathExtendedFlags>,
                        Option<u16>,
                        Option<u32>,
                        Option<DataPathStats>,
                    ),
                    scroll::Error,
                >({
                    let extended_flags = DataPathExtendedFlags::from_bits(
                        from.gread_with(&mut offset, Endian::Little)?,
                    );
                    let log_trigger_id = extended_flags
                        .log_trigger_id_present
                        .then(|| from.gread_with(&mut offset, Endian::Little))
                        .transpose()?;
                    let rlfc = extended_flags
                        .rlfc_present
                        .then(|| from.gread_with(&mut offset, Endian::Little))
                        .transpose()?;
                    let stats = extended_flags
                        .stats_present
                        .then(|| from.gread(&mut offset))
                        .transpose()?;
                    (Some(extended_flags), log_trigger_id, rlfc, stats)
                })
            })
            .transpose()?
            .unwrap_or_default();
        Ok((
            DataPathStateTLV {
                flags,
                country_code,
                channel_map,
                infra_bssid_channel,
                infra_address,
                awdl_address,
                unicast_options,
                unicast_options_ext,
                extended_flags,
                log_trigger_id,
                rlfc,
                stats,
            },
            offset,
        ))
    }
}
impl TryIntoCtx for DataPathStateTLV {
    type Error = scroll::Error;
    fn try_into_ctx(self, buf: &mut [u8], _ctx: ()) -> Result<usize, Self::Error> {
        let mut offset = 0;

        let log_trigger_id_present = self.log_trigger_id.is_some();
        let rlfc_present = self.rlfc.is_some();
        let stats_present = self.stats.is_some();
        let extended_flags = if log_trigger_id_present || rlfc_present || stats_present {
            let mut extended_flags = self.extended_flags.unwrap_or_default();

            extended_flags.log_trigger_id_present = log_trigger_id_present;
            extended_flags.rlfc_present = rlfc_present;
            extended_flags.stats_present = stats_present;
            Some(extended_flags)
        } else {
            self.extended_flags
        };

        buf.gwrite_with(
            {
                let mut flags = self.flags;

                flags.country_code_present = self.country_code.is_some();
                flags.channel_map_present = self.channel_map.is_some();
                flags.infra_bssid_channel_present = self.infra_bssid_channel.is_some();
                flags.infra_address_present = self.infra_address.is_some();
                flags.awdl_address_present = self.awdl_address.is_some();
                flags.unicast_options_present = self.unicast_options.is_some();
                flags.extended_flags = extended_flags.is_some();

                flags.into_bits()
            },
            &mut offset,
            Endian::Little,
        )?;

        if let Some(country_code) = self.country_code {
            buf.gwrite(country_code.map(|x| x as u8), &mut offset)?;
            buf.gwrite(0u8, &mut offset)?;
        }
        if let Some(channel_map) = self.channel_map {
            buf.gwrite_with(channel_map.as_u16(), &mut offset, Endian::Little)?;
        }
        if let Some((infra_bssid, channel)) = self.infra_bssid_channel {
            buf.gwrite(infra_bssid, &mut offset)?;
            buf.gwrite(channel, &mut offset)?;
        }
        if let Some(infra_address) = self.infra_address {
            buf.gwrite(infra_address, &mut offset)?;
        }
        if let Some(awdl_address) = self.awdl_address {
            buf.gwrite(awdl_address, &mut offset)?;
        }
        match (self.unicast_options, self.unicast_options_ext) {
            (Some(unicast_options), None) => {
                buf.gwrite(4u16, &mut offset)?;
                buf.gwrite_with(
                    unicast_options.into_bits(),
                    &mut offset,
                    Endian::Little,
                )?;
            }
            (Some(unicast_options), Some(unicast_options_ext)) => {
                buf.gwrite(8u16, &mut offset)?;
                buf.gwrite_with(
                    unicast_options.into_bits(),
                    &mut offset,
                    Endian::Little,
                )?;
                buf.gwrite_with(unicast_options_ext, &mut offset, Endian::Little)?;
            }
            (None, Some(unicast_options_ext)) => {
                buf.gwrite(8u16, &mut offset)?;
                buf.gwrite_with(0u32, &mut offset, Endian::Little)?;
                buf.gwrite_with(unicast_options_ext, &mut offset, Endian::Little)?;
            }
            _ => {}
        }
        if let Some(extended_flags) = extended_flags {
            buf.gwrite_with(
                extended_flags.into_bits(),
                &mut offset,
                Endian::Little,
            )?;
            if let Some(log_trigger_id) = self.log_trigger_id {
                buf.gwrite_with(log_trigger_id, &mut offset, Endian::Little)?;
            }
            if let Some(rlfc) = self.rlfc {
                buf.gwrite_with(rlfc, &mut offset, Endian::Little)?;
            }
            if let Some(stats) = self.stats {
                buf.gwrite(stats, &mut offset)?;
            }
        }

        Ok(offset)
    }
}

#[test]
fn test_data_path_state_tlv() {
    use self::misc::ChannelMap;
    use alloc::vec;
    use mac_parser::ZERO;

    let bytes = include_bytes!("../../../../test_bins/data_path_state_tlv.bin");
    let data_path_state = bytes.pread::<DataPathStateTLV>(0).unwrap();
    assert_eq!(
        data_path_state,
        DataPathStateTLV {
            flags: DataPathFlags {
                infra_bssid_channel_present: true,
                infra_address_present: true,
                dualband_support: true,
                country_code_present: true,
                channel_map_present: true,
                airplay_solo_mode_support: true,
                umi_support: true,
                unicast_options_present: true,
                extended_flags: true,
                ..Default::default()
            },
            country_code: Some(['D', 'E']),
            channel_map: Some(DataPathChannel::ChannelMap(ChannelMap {
                channel_6: true,
                channel_44: true,
                channel_149: false
            })),
            infra_bssid_channel: Some((ZERO, 0)),
            infra_address: Some(MACAddress::new([0xbe, 0x45, 0xa1, 0xd1, 0x49, 0xb6])),
            awdl_address: None,
            unicast_options: Some(UnicastOptions {
                ..Default::default()
            },),
            unicast_options_ext: None,
            extended_flags: Some(DataPathExtendedFlags {
                log_trigger_id_present: true,
                rlfc_present: true,
                is_social_channel_map_supported: true,
                stats_present: true,
                dfs_proxy_support: true,
                ..Default::default()
            }),
            rlfc: Some(10836),
            log_trigger_id: Some(0x00),
            stats: Some(DataPathStats {
                msec_since_activation: 183,
                aw_seq_counter: 0,
                pay_update_coutner: 32641,
            }),
        }
    );
    let mut buf = vec![0x00u8; data_path_state.size_in_bytes()];
    buf.pwrite(data_path_state, 0).unwrap();
    assert_eq!(bytes, buf.as_slice());
}
