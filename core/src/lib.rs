//! The gin-tonic-core library handles protobuf encode and decoding
mod decoder;
mod encoder;
pub mod macros;
mod tag;
#[cfg(test)]
mod tests;
pub mod types;

pub use decoder::{Decode, DecodeError};
pub use encoder::Encode;
pub use tag::Tag;

pub const WIRE_TYPE_VARINT: u8 = 0;
pub const WIRE_TYPE_I64: u8 = 1;
pub const WIRE_TYPE_LENGTH_ENCODED: u8 = 2;
pub const WIRE_TYPE_I32: u8 = 5;
