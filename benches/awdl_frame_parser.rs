use awdl_frame_parser::{
    action_frame::DefaultAWDLActionFrame,
    tlvs::{
        data_path::DataPathStateTLV,
        dns_sd::{DefaultArpaTLV, DefaultServiceParametersTLV, DefaultServiceResponseTLV},
        sync_elect::{ChannelSequenceTLV, DefaultSyncTreeTLV},
    },
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use scroll::{Pread, Pwrite};

macro_rules! bench_read {
    ($bench_name:ident, $type_name:ty) => {
        fn $bench_name(bytes: &[u8]) {
            let _ = black_box(bytes).pread::<$type_name>(0).unwrap();
        }
    };
    ($bench_name:ident, $type_name:ty, $ctx:ty) => {
        fn $bench_name(bytes: &[u8], ctx: &$ctx) {
            let _ = black_box(bytes).pread_with::<$type_name>(0, ctx).unwrap();
        }
    };
}
macro_rules! bench_write {
    ($bench_name:ident, $type_name:ty, $buffer_length:expr) => {
        fn $bench_name(data: $type_name) {
            let mut buf = [0x00u8; $buffer_length];
            buf.pwrite(data, 0).unwrap();
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

bench_read!(bench_read_af, DefaultAWDLActionFrame);
bench_write!(bench_write_af, DefaultAWDLActionFrame, 0x1fff);
bench_read!(
    bench_read_service_parmeters_tlv,
    DefaultServiceParametersTLV
);
bench_write!(
    bench_write_service_parameters_tlv,
    DefaultServiceParametersTLV,
    0xff
);
bench_read!(bench_read_channel_sequence_tlv, ChannelSequenceTLV);
bench_write!(bench_write_channel_sequence_tlv, ChannelSequenceTLV, 0xff);
bench_read!(bench_read_sync_tree_tlv, DefaultSyncTreeTLV);
bench_write!(bench_write_sync_tree_tlv, DefaultSyncTreeTLV, 0xff);
bench_read!(bench_read_service_response_tlv, DefaultServiceResponseTLV);
bench_write!(
    bench_write_service_response_tlv,
    DefaultServiceResponseTLV,
    0xff
);
bench_read!(bench_read_data_path_state_tlv, DataPathStateTLV);
bench_write!(bench_write_data_path_state_tlv, DataPathStateTLV, 0xff);
bench_read!(bench_read_arpa_tlv, DefaultArpaTLV);
bench_write!(bench_write_arpa_tlv, DefaultArpaTLV, 0xff);

fn criterion_benchmark(c: &mut Criterion) {
    let af_bytes = include_bytes!("../test_bins/mif.bin");
    register_bench_fn!(c, bench_read_af, af_bytes);
    let af = af_bytes.pread::<DefaultAWDLActionFrame>(0).unwrap();
    register_bench_fn!(c, bench_write_af, af.clone());

    let data_path_state_tlv_bytes = &include_bytes!("../test_bins/data_path_state_tlv.bin")[3..];
    register_bench_fn!(c, bench_read_data_path_state_tlv, data_path_state_tlv_bytes);
    let data_path_state_tlv = data_path_state_tlv_bytes
        .pread::<DataPathStateTLV>(0)
        .unwrap();
    register_bench_fn!(
        c,
        bench_write_data_path_state_tlv,
        data_path_state_tlv.clone()
    );

    let service_parameters_tlv_bytes =
        &include_bytes!("../test_bins/service_parameters_tlv.bin")[3..];
    register_bench_fn!(
        c,
        bench_read_service_parmeters_tlv,
        service_parameters_tlv_bytes
    );
    let service_parameters_tlv = service_parameters_tlv_bytes
        .pread::<DefaultServiceParametersTLV>(0)
        .unwrap();
    register_bench_fn!(
        c,
        bench_write_service_parameters_tlv,
        service_parameters_tlv.clone()
    );

    let sync_tree_tlv_bytes = &include_bytes!("../test_bins/sync_tree_tlv.bin")[3..];
    register_bench_fn!(c, bench_read_sync_tree_tlv, sync_tree_tlv_bytes);
    let sync_tree_tlv = sync_tree_tlv_bytes.pread::<DefaultSyncTreeTLV>(0).unwrap();
    register_bench_fn!(c, bench_write_sync_tree_tlv, sync_tree_tlv.clone());

    let service_response_tlv_bytes =
        &include_bytes!("../test_bins/service_response_tlv_txt.bin")[3..];
    register_bench_fn!(
        c,
        bench_read_service_response_tlv,
        service_response_tlv_bytes
    );
    let service_response_tlv = service_response_tlv_bytes
        .pread::<DefaultServiceResponseTLV>(0)
        .unwrap();
    register_bench_fn!(
        c,
        bench_write_service_response_tlv,
        service_response_tlv.clone()
    );

    let arpa_tlv_bytes = &include_bytes!("../test_bins/arpa_tlv.bin")[3..];
    register_bench_fn!(c, bench_read_arpa_tlv, arpa_tlv_bytes);
    let arpa_tlv = service_response_tlv_bytes
        .pread::<DefaultArpaTLV>(0)
        .unwrap();
    register_bench_fn!(c, bench_write_arpa_tlv, arpa_tlv.clone());

    let channel_sequence_tlv_bytes = &include_bytes!("../test_bins/channel_sequence_tlv.bin")[3..];
    register_bench_fn!(
        c,
        bench_read_channel_sequence_tlv,
        channel_sequence_tlv_bytes
    );
    let channel_sequence_tlv = channel_sequence_tlv_bytes
        .pread::<ChannelSequenceTLV>(0)
        .unwrap();
    register_bench_fn!(
        c,
        bench_write_channel_sequence_tlv,
        channel_sequence_tlv.clone()
    );
}
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
