use awdl_frame_parser::action_frame::{
    tlv::{TLVType, TLV},
    version::AWDLVersion,
    AWDLActionFrame, AWDLActionFrameSubType,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deku::DekuContainerWrite;

fn bench_read_af(bytes: &[u8]) {
    let _af = AWDLActionFrame::try_from(bytes).unwrap();
}
fn bench_write_af(af: &AWDLActionFrame) {
    let _bytes = af.to_bytes().unwrap();
}
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench_read_af", |b| {
        b.iter(|| bench_read_af(black_box(include_bytes!("../mif.bin").as_ref())))
    });
    c.bench_function("bench_write_af", |b| {
        b.iter(|| {
            bench_write_af(&black_box(AWDLActionFrame {
                awdl_version: AWDLVersion {
                    major: 3,
                    minor: 14,
                },
                subtype: AWDLActionFrameSubType::MIF,
                phy_tx_time: 0x666C6168,
                target_tx_time: 0x6566696C,
                tlvs: vec![TLV {
                    tlv_type: TLVType::Version,
                    tlv_length: 0x02,
                    tlv_data: vec![(0x03 << 4 | 0x14), 0x03],
                }],
            }))
        })
    });
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
