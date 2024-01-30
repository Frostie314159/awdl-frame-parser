mod awdl_dns_compression;
mod awdl_dns_name;
mod awdl_str;
mod awdl_version;

pub use awdl_dns_compression::AWDLDnsCompression;
pub use awdl_dns_name::{AWDLDnsName, DefaultAWDLDnsName, ReadLabelIterator};
pub use awdl_str::AWDLStr;
pub use awdl_version::AWDLVersion;
