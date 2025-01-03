//! gin-tonic wrapepr for code generation (models and service)
#[cfg(feature = "tonic")]
mod codec;

#[cfg(feature = "generator")]
mod codegen;

#[cfg(feature = "tonic")]
pub use codec::GinCodec;

#[cfg(feature = "generator")]
pub use codegen::CompileConfig;

pub use gin_tonic_core;

#[cfg(feature = "derive")]
pub use gin_tonic_derive;

pub use gin_tonic_core::{types, types::PbOneOf, types::PbType, Decode, DecodeError, Encode, Tag};

#[cfg(feature = "derive")]
pub use gin_tonic_derive::{Enumeration, Message, OneOf};

// Re-export the alloc crate for use within derived code.
#[doc(hidden)]
pub extern crate alloc;

#[cfg(test)]
mod test;
