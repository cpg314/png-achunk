#![doc = include_str!("../README.md")]

mod chunk;
pub use chunk::{Chunk, ChunkType};

pub mod decode;
pub mod encode;

pub use decode::Decoder;
pub use encode::Encoder;
