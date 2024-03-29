use awdl_frame_parser::{action_frame::DefaultAWDLActionFrame, tlvs::AWDLTLV};
use scroll::Pread;

fn main() {
    let bytes = include_bytes!("../test_bins/mif.bin");
    let af = bytes.pread::<DefaultAWDLActionFrame>(0).unwrap();

    println!("{af:#?}");

    println!(
        "awdl version: {:#?}",
        af.tagged_data
            .clone()
            .find_map(|tlv| if let AWDLTLV::Version(version_tlv) = tlv {
                Some(version_tlv)
            } else {
                None
            })
            .unwrap()
    );
}
