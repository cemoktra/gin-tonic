//! gin-tonic wrapepr for code generation (models and service)
mod codec;
mod codegen;

pub use codec::GinCodec;
pub use codegen::CompileConfig;

pub use gin_tonic_core;
pub use gin_tonic_derive;

pub use gin_tonic_core::{types, types::PbOneOf, types::PbType, Decode, DecodeError, Encode, Tag};
pub use gin_tonic_derive::{Enumeration, Message, OneOf};

// Re-export the alloc crate for use within derived code.
#[doc(hidden)]
pub extern crate alloc;

#[cfg(test)]
mod test;
