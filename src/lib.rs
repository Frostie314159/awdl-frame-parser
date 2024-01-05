#![no_std]
#![forbid(unsafe_code)]
#![feature(
    iter_next_chunk,
    array_chunks,
    slice_as_chunks,
    type_alias_impl_trait,
    debug_closure_helpers
)]

/* #[cfg(test)] */
extern crate alloc;

/// Structures related to AWDL action frames.
pub mod action_frame;
/// Structures, which are shared between multiple TLVs and frame types.
pub mod common;
//pub mod data_frame;
/// Every TLV currently understood.
pub mod tlvs;
