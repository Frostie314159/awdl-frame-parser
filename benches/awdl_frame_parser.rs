use std::borrow::Cow;

use awdl_frame_parser::action_frame::{
    dns_compression::AWDLDnsCompression,
    tlv::{
        dns_sd::{ArpaTLV, Hostname},
        version::{AWDLDeviceClass, VersionTLV},
        TLVType, TLV,
    },
    version::AWDLVersion,
    AWDLActionFrame, AWDLActionFrameSubType,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use deku::{bitvec::BitSlice, DekuContainerRead, DekuContainerWrite};

macro_rules! bench_read {
    ($bench_name:ident, $type_name:ty) => {
        fn $bench_name(bytes: &[u8]) {
            let (_rest, _v) = <$type_name>::from_bytes((bytes, 0)).unwrap();
        }
    };
}
macro_rules! bench_write {
    ($bench_name:ident, $type_name:ty) => {
        fn $bench_name(data: &$type_name) {
            let _bytes = data.to_bytes().unwrap();
        }
    };
}
macro_rules! register_bench_fn {
    ($criterion:expr, $fn_name:ident, $arg:expr) => {
        $criterion.bench_function(stringify!($fn_name), |b| {
            b.iter(|| $fn_name(black_box($arg)))
        });
    };
}

bench_read!(bench_read_af, AWDLActionFrame);
bench_write!(bench_write_af, AWDLActionFrame);
bench_read!(bench_read_version, AWDLVersion);
bench_write!(bench_write_version, AWDLVersion);
bench_read!(bench_read_subtype, AWDLActionFrameSubType);
bench_write!(bench_write_subtype, AWDLActionFrameSubType);
bench_read!(bench_read_tlv, TLV);
bench_write!(bench_write_tlv, TLV);
bench_read!(bench_read_version_tlv, VersionTLV);
bench_write!(bench_write_version_tlv, VersionTLV);
bench_read!(bench_read_arpa_tlv, ArpaTLV);
bench_write!(bench_write_arpa_tlv, ArpaTLV);
fn bench_read_tlvs(bytes: &[u8]) {
    let _tlvs = AWDLActionFrame::read_tlvs(BitSlice::from_slice(bytes)).unwrap();
}
fn bench_get_tlvs(af: &AWDLActionFrame) {
    let _tlvs = af.get_tlvs(TLVType::ServiceResponse);
}
fn criterion_benchmark(c: &mut Criterion) {
    let bytes = include_bytes!("../test_bins/mif.bin");
    let af = AWDLActionFrame::try_from(bytes.as_ref()).unwrap();
    register_bench_fn!(c, bench_read_af, bytes.as_ref());
    register_bench_fn!(c, bench_write_af, &af);
    register_bench_fn!(
        c,
        bench_read_version,
        include_bytes!("../test_bins/version_tlv.bin")[3..].as_ref()
    );
    register_bench_fn!(
        c,
        bench_write_version,
        &AWDLVersion {
            major: 0x03,
            minor: 0x14
        }
    );
    register_bench_fn!(c, bench_read_subtype, [0x03].as_ref());
    register_bench_fn!(c, bench_write_subtype, &AWDLActionFrameSubType::MIF);
    register_bench_fn!(
        c,
        bench_read_tlv,
        include_bytes!("../test_bins/version_tlv.bin").as_ref()
    );
    register_bench_fn!(
        c,
        bench_write_tlv,
        &TLV {
            tlv_type: TLVType::Unknown(0x10),
            tlv_length: 0x06,
            tlv_data: [0x00; 0x6].to_vec()
        }
    );
    register_bench_fn!(
        c,
        bench_read_version_tlv,
        include_bytes!("../test_bins/version_tlv.bin").as_ref()
    );
    register_bench_fn!(
        c,
        bench_write_version_tlv,
        &VersionTLV {
            version: AWDLVersion {
                major: 0x03,
                minor: 0x14
            },
            device_class: AWDLDeviceClass::TVOS
        }
    );
    register_bench_fn!(
        c,
        bench_read_arpa_tlv,
        include_bytes!("../test_bins/arpa_tlv.bin")[3..].as_ref()
    );
    register_bench_fn!(
        c,
        bench_write_arpa_tlv,
        &ArpaTLV {
            flags: 0xff,
            arpa: Hostname {
                domain: AWDLDnsCompression::Local,
                host: Cow::Borrowed("Black Mesa"),
                ..Default::default()
            }
        }
    );
    register_bench_fn!(
        c,
        bench_read_tlvs,
        include_bytes!("../test_bins/tlvs.bin").as_ref()
    );
    register_bench_fn!(c, bench_get_tlvs, &af);
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
