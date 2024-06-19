mod codec;
mod codegen;
mod impls;
mod traits;

pub use codec::GinCodec;
pub use codegen::{BuildEnvironment, Compiler};

pub use gin_tonic_core;
pub use gin_tonic_derive;

pub use gin_tonic_core::{Error, FromWire, IntoWire};
pub use gin_tonic_derive::{Enumeration, Message, OneOf};

pub mod export {
    pub use integer_encoding::VarInt;
    pub use prost;
}
