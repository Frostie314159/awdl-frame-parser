use std::borrow::Cow;

use awdl_frame_parser::{
    action_frame::{
        tlv::{version::VersionTLV, TLVType, TLV},
        AWDLActionFrame,
    },
    parser::{Read, Write},
};

fn main() {
    let bytes = include_bytes!("../test_bins/mif.bin");
    let af = AWDLActionFrame::from_bytes(&mut bytes.iter().copied()).unwrap();

    println!("{af:#?}");
    assert_eq!(af.to_bytes(), <&[u8] as Into<Cow<[u8]>>>::into(bytes));

    let version = af.get_tlvs(TLVType::Version).unwrap()[0].clone(); // Since a TLV could be present multiple times we have to this.
    let version: VersionTLV = version.try_into().unwrap();
    println!("awdl version: {version:#?}");
    let _tlv: TLV = version.into();
}
