#![no_main]

use libfuzzer_sys::fuzz_target;
use bin_utils::*;
extern crate awdl_frame_parser;

fuzz_target!(|data: &[u8]| {
    let _ = awdl_frame_parser::tlvs::dns_sd::ServiceParametersTLV::from_bytes(&mut data.iter().copied());
});
