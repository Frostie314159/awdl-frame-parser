use crate::enum_to_int;

use alloc::{
    format,
    string::{String, ToString},
};

#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, Default, PartialEq, Eq)]
/// Compressed dns-sd domains/services. Compression might be the wrong word though.
pub enum AWDLDnsCompression {
    Null,

    AirPlayTcpLocal,

    AirPlayUdpLocal,

    AirPlay,

    RaopTcpLocal,

    RaopUdpLocal,

    Raop,

    AirDropTcpLocal,

    AirDropUdpLocal,

    AirDrop,

    TcpLocal,

    UdpLocal,

    #[default]
    Local,

    Ip6Arpa,

    Ip4Arpa,

    Unknown(u16),
}
enum_to_int! {
    u16,
    AWDLDnsCompression,

    0x00C0,
    AWDLDnsCompression::Null,
    0x01C0,
    AWDLDnsCompression::AirPlayTcpLocal,
    0x02C0,
    AWDLDnsCompression::AirPlayUdpLocal,
    0x03C0,
    AWDLDnsCompression::AirPlay,
    0x04C0,
    AWDLDnsCompression::RaopTcpLocal,
    0x05C0,
    AWDLDnsCompression::RaopUdpLocal,
    0x06C0,
    AWDLDnsCompression::Raop,
    0x07C0,
    AWDLDnsCompression::AirDropTcpLocal,
    0x08C0,
    AWDLDnsCompression::AirDropUdpLocal,
    0x09C0,
    AWDLDnsCompression::AirDrop,
    0x0AC0,
    AWDLDnsCompression::TcpLocal,
    0x0BC0,
    AWDLDnsCompression::UdpLocal,
    0x0CC0,
    AWDLDnsCompression::Local,
    0x0DC0,
    AWDLDnsCompression::Ip6Arpa,
    0x0EC0,
    AWDLDnsCompression::Ip4Arpa
}
macro_rules! string {
    ($str:expr) => {
        stringify!($expr).to_string()
    };
}
impl ToString for AWDLDnsCompression {
    fn to_string(&self) -> String {
        match self {
            AWDLDnsCompression::Null => string!("null"),
            AWDLDnsCompression::AirPlayTcpLocal => string!("_airplay._tcp.local"),
            AWDLDnsCompression::AirPlayUdpLocal => string!("_airplay._udp.local"),
            AWDLDnsCompression::AirPlay => string!("_airplay"),
            AWDLDnsCompression::RaopTcpLocal => string!("_raop._tcp.local"),
            AWDLDnsCompression::RaopUdpLocal => string!("_raop._udp.local"),
            AWDLDnsCompression::Raop => string!("raop"),
            AWDLDnsCompression::AirDropTcpLocal => string!("_airdrop._tcp.local"),
            AWDLDnsCompression::AirDropUdpLocal => string!("_airdrop._udp.local"),
            AWDLDnsCompression::AirDrop => string!("_airdrop"),
            AWDLDnsCompression::TcpLocal => string!("_tcp.local"),
            AWDLDnsCompression::UdpLocal => string!("_udp.local"),
            AWDLDnsCompression::Local => string!("local"),
            AWDLDnsCompression::Ip6Arpa => string!("ip6.arpa"),
            AWDLDnsCompression::Ip4Arpa => string!("ip4.arpa"),
            AWDLDnsCompression::Unknown(v) => format!("unknown: {v}"),
        }
    }
}
