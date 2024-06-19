pub mod codegen;
pub mod impls;
pub mod protobuf;
pub mod traits;

pub mod export {
    pub use integer_encoding::VarInt;
    pub use prost;
}

pub use gin_tonic_derive::{Enumeration, Message, OneOf};
