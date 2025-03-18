use awdl_frame_parser::{action_frame::DefaultAWDLActionFrame, tlvs::version::VersionTLV};
use scroll::Pread;

fn main() {
    let bytes = include_bytes!("../test_bins/mif.bin");
    let af = bytes.pread::<DefaultAWDLActionFrame>(0).unwrap();

    println!(
        "awdl version: {:#?}",
        af.tagged_data.get_first_tlv::<VersionTLV>()
    );
}
