use macro_bits::serializable_enum;

serializable_enum! {
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Clone, Copy, Default, PartialEq, Eq)]
    /// Compressed dns-sd domains/services. Compression might be the wrong word though.
    pub enum AWDLDnsCompression: u16 {
        Null => 0xC000,

        AirPlayTcpLocal => 0xC001,

        AirPlayUdpLocal => 0xC002,

        AirPlay => 0xC003,

        RaopTcpLocal => 0xC004,

        RaopUdpLocal => 0xC005,

        Raop => 0xC006,

        AirDropTcpLocal => 0xC007,

        AirDropUdpLocal => 0xC008,

        AirDrop => 0xC009,

        TcpLocal => 0xC00A,

        UdpLocal => 0xC00B,

        #[default]
        Local => 0xC00C,

        Ip6Arpa => 0xC00D,

        Ip4Arpa => 0xC00E
    }
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
