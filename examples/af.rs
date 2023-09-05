use awdl_frame_parser::{
    action_frame::AWDLActionFrame,
    tlvs::{version::VersionTLV, TLVType, AWDLTLV},
};
use bin_utils::{Read, Write};

fn main() {
    let bytes = include_bytes!("../test_bins/mif.bin");
    let af = AWDLActionFrame::from_bytes(&mut bytes.iter().copied()).unwrap();

    println!("{af:#?}");
    assert_eq!(af.to_bytes(), bytes.to_vec());

    let version = af.get_tlvs(TLVType::Version).unwrap()[0].clone(); // Since a TLV could be present multiple times we have to this.
    let version: VersionTLV = version.try_into().unwrap();
    println!("awdl version: {version:#?}");
    let _tlv: AWDLTLV = version.into();
}
