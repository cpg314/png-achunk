// Avoid code examples in README.md to be compiled when running doctests.
#![cfg(not(doctest))]
#![doc = include_str!("../README.md")]

mod chunk;
pub use chunk::{Chunk, ChunkType};

pub mod decode;
pub mod encode;

pub use decode::Decoder;
pub use encode::Encoder;
