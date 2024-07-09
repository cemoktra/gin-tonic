mod tags;
mod wire;

pub use tags::{reader::TagReader, Tag};
pub use wire::{
    map::{from_wire as map_from_wire, into_wire as map_into_wire},
    nested::size_hint as nested_size_hint,
    wire_type::{WireType, WireTypeView},
    Error, FromWire, IntoWire,
};

/// trait for passing a struct as protobuf message
pub trait Message
where
    Self: Sized,
{
    // for serialization
    fn serialize(self, writer: &mut impl std::io::Write) -> Result<usize, Error>;
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
