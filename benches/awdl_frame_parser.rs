use awdl_frame_parser::{
    action_frame::AWDLActionFrame,
    tlvs::{
        dns_sd::{ArpaTLV, ServiceParametersTLV, ServiceResponseTLV},
        sync_elect::{ChannelSequenceTLV, SyncTreeTLV},
        TLVType,
    },
};
use bin_utils::{Read, Write};
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

bench_read!(bench_read_service_parmeters_tlv, ServiceParametersTLV);
bench_write!(bench_write_service_parameters_tlv, ServiceParametersTLV);
bench_read!(bench_read_arpa_tlv, ArpaTLV);
bench_write!(bench_write_arpa_tlv, ArpaTLV);
bench_read!(bench_read_channel_sequence_tlv, ChannelSequenceTLV);
bench_write!(bench_write_channel_sequence_tlv, ChannelSequenceTLV);
bench_read!(bench_read_service_response_tlv, ServiceResponseTLV);
bench_write!(bench_write_service_response_tlv, ServiceResponseTLV);
bench_read!(bench_read_sync_tree_tlv, SyncTreeTLV);
bench_write!(bench_write_sync_tree_tlv, SyncTreeTLV);

fn criterion_benchmark(c: &mut Criterion) {
    let af_bytes = include_bytes!("../test_bins/mif.bin").to_vec();
    let af = AWDLActionFrame::from_bytes(&mut af_bytes.clone().into_iter()).unwrap();
    register_bench_fn!(c, bench_read_af, af_bytes.clone());
    register_bench_fn!(c, bench_write_af, &af);
    register_bench_fn!(c, bench_get_tlvs, &af);

    let service_parameters_tlv_bytes =
        include_bytes!("../test_bins/service_parameters_tlv.bin")[3..].to_vec();
    register_bench_fn!(
        c,
        bench_read_service_parmeters_tlv,
        service_parameters_tlv_bytes.clone()
    );
    let service_parameters =
        ServiceParametersTLV::from_bytes(&mut service_parameters_tlv_bytes.clone().into_iter())
            .unwrap();
    register_bench_fn!(c, bench_write_service_parameters_tlv, &service_parameters);

    let sync_tree_tlv_bytes = include_bytes!("../test_bins/sync_tree_tlv.bin")[3..].to_vec();
    register_bench_fn!(c, bench_read_sync_tree_tlv, sync_tree_tlv_bytes.clone());
    let sync_tree = SyncTreeTLV::from_bytes(&mut sync_tree_tlv_bytes.clone().into_iter()).unwrap();
    register_bench_fn!(c, bench_write_sync_tree_tlv, &sync_tree);

    let service_response_tlv_bytes =
        include_bytes!("../test_bins/service_response_tlv_txt.bin")[3..].to_vec();
    register_bench_fn!(
        c,
        bench_read_service_response_tlv,
        service_response_tlv_bytes.clone()
    );
    let service_response =
        ServiceResponseTLV::from_bytes(&mut service_response_tlv_bytes.clone().into_iter())
            .unwrap();
    register_bench_fn!(c, bench_write_service_response_tlv, &service_response);

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
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
