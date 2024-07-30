//mod codec;
//mod codegen;

// pub use codec::GinCodec;
// pub use codegen::CompileConfig;

pub use gin_tonic_core;
pub use gin_tonic_derive;

pub use gin_tonic_core::{types, types::PbOneOf, types::PbType, Decode, DecodeError, Encode, Tag};
pub use gin_tonic_derive::{Enumeration, Message, OneOf};

pub mod export {
    // pub use gin_tonic_core::VarInt;
    pub use smallvec::SmallVec;
}

// Re-export the alloc crate for use within derived code.
#[doc(hidden)]
pub extern crate alloc;

#[cfg(test)]
mod test;
