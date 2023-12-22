#![feature(
    iter_array_chunks,
    iter_next_chunk,
    iter_advance_by,
    iterator_try_collect,
    exact_size_is_empty,
    more_qualified_paths,
    array_chunks,
    doc_cfg,
    slice_as_chunks,
    slice_flatten,
    iter_intersperse,
    debug_closure_helpers
)]
#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

/// Structures related to AWDL action frames.
pub mod action_frame;
/// Structures, which are shared between multiple TLVs and frame types.
pub mod common;
//pub mod data_frame;
/// Every TLV currently understood.
pub mod tlvs;
pub use heapless;
