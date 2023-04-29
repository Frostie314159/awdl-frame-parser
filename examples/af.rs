use awdl_frame_parser::action_frame::AWDLActionFrame;
use deku::DekuContainerWrite;

fn main() {
    let bytes = include_bytes!("../test_bins/mif.bin").as_ref();
    let af = AWDLActionFrame::try_from(bytes).unwrap();

    println!("{af:#?}");
    assert_eq!(af.to_bytes().unwrap(), bytes);
}
