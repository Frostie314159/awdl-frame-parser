#![feature(
    iter_array_chunks,
    iter_next_chunk,
    exact_size_is_empty,
    more_qualified_paths,
    array_chunks,
    doc_cfg
)]
#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

/// Structures related to AWDL action frames.
pub mod action_frame;
pub mod common;
pub mod tlvs;
