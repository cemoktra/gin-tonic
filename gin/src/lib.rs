mod codec;
mod codegen;
mod impls;
pub mod protobuf;
mod traits;

pub use codec::GinCodec;
pub use codegen::{BuildEnvironment, Compiler};
pub use protobuf::{Error, FromWire, IntoWire};

pub mod export {
    pub use integer_encoding::VarInt;
    pub use prost;
}

pub use gin_tonic_derive::{Enumeration, Message, OneOf};
