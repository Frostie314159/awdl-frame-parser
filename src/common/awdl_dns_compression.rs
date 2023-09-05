use bin_utils::*;

#[non_exhaustive]
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

    0xC000,
    AWDLDnsCompression::Null,
    0xC001,
    AWDLDnsCompression::AirPlayTcpLocal,
    0xC002,
    AWDLDnsCompression::AirPlayUdpLocal,
    0xC003,
    AWDLDnsCompression::AirPlay,
    0xC004,
    AWDLDnsCompression::RaopTcpLocal,
    0xC005,
    AWDLDnsCompression::RaopUdpLocal,
    0xC006,
    AWDLDnsCompression::Raop,
    0xC007,
    AWDLDnsCompression::AirDropTcpLocal,
    0xC008,
    AWDLDnsCompression::AirDropUdpLocal,
    0xC009,
    AWDLDnsCompression::AirDrop,
    0xC00A,
    AWDLDnsCompression::TcpLocal,
    0xC00B,
    AWDLDnsCompression::UdpLocal,
    0xC00C,
    AWDLDnsCompression::Local,
    0xC00D,
    AWDLDnsCompression::Ip6Arpa,
    0xC00E,
    AWDLDnsCompression::Ip4Arpa
}
impl AWDLDnsCompression {
    pub fn to_string(&self) -> &'static str {
        match self {
            AWDLDnsCompression::Null => "",
            AWDLDnsCompression::AirPlayTcpLocal => "_airplay._tcp.local",
            AWDLDnsCompression::AirPlayUdpLocal => "_airplay._udp.local",
            AWDLDnsCompression::AirPlay => "_airplay",
            AWDLDnsCompression::RaopTcpLocal => "_raop._tcp.local",
            AWDLDnsCompression::RaopUdpLocal => "_raop._udp.local",
            AWDLDnsCompression::Raop => "raop",
            AWDLDnsCompression::AirDropTcpLocal => "_airdrop._tcp.local",
            AWDLDnsCompression::AirDropUdpLocal => "_airdrop._udp.local",
            AWDLDnsCompression::AirDrop => "_airdrop",
            AWDLDnsCompression::TcpLocal => "_tcp.local",
            AWDLDnsCompression::UdpLocal => "_udp.local",
            AWDLDnsCompression::Local => "local",
            AWDLDnsCompression::Ip6Arpa => "ip6.arpa",
            AWDLDnsCompression::Ip4Arpa => "ip4.arpa",
            AWDLDnsCompression::Unknown(_) => "Unknown Compression",
        }
    }
}
