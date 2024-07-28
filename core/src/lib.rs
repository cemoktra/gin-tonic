mod decoder;
mod encoder;
mod macros;
mod message;
mod tag;
#[cfg(test)]
mod tests;
mod types;

pub const WIRE_TYPE_VARINT: u8 = 0;
pub const WIRE_TYPE_I64: u8 = 1;
pub const WIRE_TYPE_LENGTH_ENCODED: u8 = 2;
pub const WIRE_TYPE_I32: u8 = 5;

mod tags;
mod wire;

pub use bytes;
pub use tags::{reader::TagReader, Tag};
pub use wire::{
    map::from_wire as map_from_wire,
    nested::size_hint as nested_size_hint,
    varint::VarInt,
    wire_type::{WireType, WireTypeView},
    Error, FromWire, IntoWire,
};

/// trait for passing a struct as protobuf message
pub trait Message
where
    Self: Sized,
{
    // for serialization
    fn serialize(self, writer: &mut impl bytes::BufMut);
    fn size_hint(&self) -> usize;

    // for deserialization
    fn deserialize(buffer: &[u8]) -> Result<(Self, usize), Error> {
        let mut reader = TagReader::new(buffer);
        let slf = Self::deserialize_tags(reader.by_ref())?;

        Ok((slf, reader.position()))
    }

    fn deserialize_tags<'a>(tags: impl Iterator<Item = Tag<'a>>) -> Result<Self, Error>;
}

/// special handling for one ofs
pub trait OneOf: Message
where
    Self: Sized,
{
    // for deserialization
    fn matches_tag(tag: u32) -> bool;
    fn deserialize_wire(tag: u32, wire_type: WireTypeView) -> Result<Self, Error>;
}
