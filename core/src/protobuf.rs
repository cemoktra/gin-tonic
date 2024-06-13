pub mod reader;
pub mod tag;
pub mod wire;

mod scalar;
#[cfg(test)]
mod test;

use crate::protobuf::wire::{WireType, WireTypeView};

/// error enumeration for problems occuring when converting a [WireTypeView] into an actual type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("unexpected wire type")]
    UnexpectedWireType,
    #[error("invalid varint")]
    InvalidVarInt,
    #[error("field with number {0} is missing")]
    MissingField(u32),
    #[error(transparent)]
    Conversion(Box<dyn std::error::Error>),
}

pub trait ProtocolBuffer
where
    Self: Sized,
{
    fn serialize(&self, writer: &mut impl std::io::Write) -> Result<usize, Error>;
    fn deserialize(buffer: &[u8]) -> Result<Self, Error>;
    fn size_hint(&self) -> usize;
}

/// convert a [WireTypeView] into an actual Rust type
pub trait FromWire {
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized;
}

/// convert a Rust type into a [WireType]
pub trait IntoWire {
    fn into_wire(self) -> WireType;
    fn size_hint(&self) -> usize;
}
