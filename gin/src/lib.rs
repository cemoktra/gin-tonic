mod codec;
mod codegen;

pub use codec::GinCodec;
pub use codegen::CompileConfig;

pub use gin_tonic_core;
pub use gin_tonic_derive;

pub use gin_tonic_core::{Error, FromWire, IntoWire};
pub use gin_tonic_derive::{Enumeration, Message, OneOf};

pub mod export {
    pub use integer_encoding::VarInt;
}

// Re-export the alloc crate for use within derived code.
#[doc(hidden)]
pub extern crate alloc;
