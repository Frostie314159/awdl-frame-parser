use deku::prelude::*;

#[cfg(all(not(feature = "std"), feature = "write"))]
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::{
    format,
    string::{String, ToString},
};

#[cfg_attr(feature = "read", derive(DekuRead))]
#[cfg_attr(feature = "write", derive(DekuWrite))]
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
#[deku(type = "u16")]
/// Compressed dns-sd domains/services. Compression might be the wrong word though.
pub enum AWDLDnsCompression {
    #[deku(id = "0x00C0")]
    Null,

    #[deku(id = "0x01C0")]
    AirPlayTcpLocal,

    #[deku(id = "0x02C0")]
    AirPlayUdpLocal,

    #[deku(id = "0x03C0")]
    AirPlay,

    #[deku(id = "0x04C0")]
    RaopTcpLocal,

    #[deku(id = "0x05C0")]
    RaopUdpLocal,

    #[deku(id = "0x06C0")]
    Raop,

    #[deku(id = "0x07C0")]
    AirDropTcpLocal,

    #[deku(id = "0x08C0")]
    AirDropUdpLocal,

    #[deku(id = "0x09C0")]
    AirDrop,

    #[deku(id = "0x0AC0")]
    TcpLocal,

    #[deku(id = "0x0BC0")]
    UdpLocal,

    #[deku(id = "0x0CC0")]
    Local,

    #[deku(id = "0x0DC0")]
    Ip6Arpa,

    #[deku(id = "0x0EC0")]
    Ip4Arpa,

    #[deku(id_pat = "_")]
    Unknown(u16),
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
