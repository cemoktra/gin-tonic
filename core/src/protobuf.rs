pub mod reader;
pub mod tag;
pub mod wire;

mod scalar;
#[cfg(test)]
mod test;

use crate::protobuf::reader::TagReader;
use crate::protobuf::wire::{WireType, WireTypeView};
use std::collections::HashMap;

/// error enumeration for problems occuring when converting a [WireTypeView] into an actual type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("unexpected wire type")]
    UnexpectedWireType,
    #[error("invalid varint")]
    InvalidVarInt,
    #[error("field with number {0} is missing")]
    MissingField(u32),
    #[error(transparent)]
    Conversion(Box<dyn std::error::Error>),
    #[error("enum variant {0} is not known")]
    UnknownEnumVariant(u32),
    #[error("invalid oneof")]
    InvalidOneOf,
}

pub trait Message
where
    Self: Sized,
{
    // for serialization
    fn serialize(self, writer: &mut impl std::io::Write) -> Result<usize, Error>;
    fn size_hint(&self) -> usize;

    // for deserialization
    fn deserialize(buffer: &[u8]) -> Result<Self, Error> {
        let reader = TagReader::new(buffer);
        let mut field_map = HashMap::<u32, Vec<WireTypeView>>::new();

        for tag in reader {
            let (field_number, wire_type) = tag.into_parts();
            field_map.entry(field_number).or_default().push(wire_type);
        }

        Self::deserialize_tags(&mut field_map)
    }

    fn deserialize_tags(tag_map: &mut HashMap<u32, Vec<WireTypeView>>) -> Result<Self, Error>;
}

pub trait OneOf
where
    Self: Sized,
{
    fn serialize(self, writer: &mut impl std::io::Write) -> Result<usize, Error>;
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
    fn size_hint(&self, tag: u32) -> usize;
}
