use awdl_frame_parser::{
    action_frame::{
        channel::{Channel, ChannelEncoding},
        tlv::{dns_sd::ArpaTLV, sync_elect::ChannelSequenceTLV, TLVType, TLV},
        AWDLActionFrame,
    },
    parser::{Read, ReadCtx, Write},
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench_read {
    ($bench_name:ident, $type_name:ty) => {
        fn $bench_name(bytes: Vec<u8>) {
            let _ = <$type_name>::from_bytes(&mut bytes.into_iter()).unwrap();
        }
    };
    ($bench_name:ident, $type_name:ty, $ctx:ty) => {
        fn $bench_name(bytes: Vec<u8>, ctx: &$ctx) {
            let _ = <$type_name>::from_bytes(&mut bytes.into_iter(), ctx).unwrap();
        }
    };
    ($bench_name:ident, $type_name:ty, $tlv_length:expr) => {
        fn $bench_name(bytes: &[u8; $tlv_length]) {
            let _ = <$type_name>::from_bytes(bytes).unwrap();
        }
    };
}
macro_rules! bench_write {
    ($bench_name:ident, $type_name:ty) => {
        fn $bench_name(data: &$type_name) {
            let _bytes = data.to_bytes();
        }
    };
}
macro_rules! register_bench_fn {
    ($criterion:expr, $fn_name:ident, $arg:expr) => {
        $criterion.bench_function(stringify!($fn_name), |b| {
            b.iter(|| $fn_name(black_box($arg)))
        });
    };
    ($criterion:expr, $fn_name:ident, $arg:expr, $ctx:expr) => {
        $criterion.bench_function(stringify!($fn_name), |b| {
            b.iter(|| $fn_name(black_box($arg), $ctx))
        });
    };
}

bench_read!(bench_read_af, AWDLActionFrame);
bench_write!(bench_write_af, AWDLActionFrame);
fn bench_get_tlvs(af: &AWDLActionFrame) {
    let _tlvs = af.get_tlvs(TLVType::SynchronizationParameters);
}

bench_read!(bench_read_tlv, TLV);
bench_write!(bench_write_tlv, TLV);
bench_read!(bench_read_arpa_tlv, ArpaTLV);
bench_write!(bench_write_arpa_tlv, ArpaTLV);
bench_read!(bench_read_channel_sequence_tlv, ChannelSequenceTLV);
bench_write!(bench_write_channel_sequence_tlv, ChannelSequenceTLV);
bench_read!(bench_read_channel, Channel, ChannelEncoding);
bench_write!(bench_write_channel, Channel);

fn criterion_benchmark(c: &mut Criterion) {
    let af_bytes = include_bytes!("../test_bins/mif.bin").to_vec();
    let af = AWDLActionFrame::from_bytes(&mut af_bytes.clone().into_iter()).unwrap();
    register_bench_fn!(c, bench_read_af, af_bytes.clone());
    register_bench_fn!(c, bench_write_af, &af);
    register_bench_fn!(c, bench_get_tlvs, &af);

    let tlv_bytes = vec![0x02, 0x05, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff];
    register_bench_fn!(c, bench_read_tlv, tlv_bytes.clone());
    let tlv = TLV::from_bytes(&mut tlv_bytes.clone().into_iter()).unwrap();
    register_bench_fn!(c, bench_write_tlv, &tlv);

    let arpa_tlv_bytes = include_bytes!("../test_bins/arpa_tlv.bin")[3..].to_vec();
    register_bench_fn!(c, bench_read_arpa_tlv, arpa_tlv_bytes.clone());
    let arpa_tlv = ArpaTLV::from_bytes(&mut arpa_tlv_bytes.clone().into_iter()).unwrap();
    register_bench_fn!(c, bench_write_arpa_tlv, &arpa_tlv);

    let channel_sequence_tlv_bytes =
        include_bytes!("../test_bins/channel_sequence_tlv.bin")[3..].to_vec();
    register_bench_fn!(
        c,
        bench_read_channel_sequence_tlv,
        channel_sequence_tlv_bytes.clone()
    );
    let channel_sequence_tlv =
        ChannelSequenceTLV::from_bytes(&mut channel_sequence_tlv_bytes.clone().into_iter())
            .unwrap();
    register_bench_fn!(c, bench_write_channel_sequence_tlv, &channel_sequence_tlv);

    let channel_bytes = vec![0x6, 0x51];
    register_bench_fn!(
        c,
        bench_read_channel,
        channel_bytes.clone(),
        &ChannelEncoding::OpClass
    );
    let channel = Channel::OpClass(0x6, 0x51);
    register_bench_fn!(c, bench_write_channel, &channel);
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
