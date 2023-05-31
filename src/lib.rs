#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]
#![feature(exact_size_is_empty)]
#![feature(more_qualified_paths)]
#![feature(array_chunks)]
#![no_std]

extern crate alloc;

/// Structures related to AWDL action frames.
pub mod action_frame;
pub mod parser;
