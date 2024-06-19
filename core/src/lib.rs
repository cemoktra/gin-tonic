mod wire;
mod tags;

use std::collections::HashMap;
pub use wire::{
    nested::size_hint as nested_size_hint,
    map::{
        into_wire as map_into_wire,
        from_wire as map_from_wire,
    },
    wire_type::{WireType, WireTypeView},
    FromWire, IntoWire, Error
};
pub use tags::{
    Tag, reader::TagReader
};

//! trait for passing a struct as protobuf message
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
        let mut field_map = HashMap::<u32, Vec<WireTypeView>>::new();

        for tag in reader.by_ref() {
            let (field_number, wire_type) = tag.into_parts();
            field_map.entry(field_number).or_default().push(wire_type);
        }

        Ok((Self::deserialize_tags(&mut field_map)?, reader.position()))
    }

    fn deserialize_tags(tag_map: &mut HashMap<u32, Vec<WireTypeView>>) -> Result<Self, Error>;
}

//! special handling for one ofs
pub trait OneOf
    where
        Self: Sized,
{
    fn serialize(self, writer: &mut impl std::io::Write) -> Result<usize, Error>;
    fn deserialize(buffer: &[u8]) -> Result<Self, Error>;
    fn size_hint(&self) -> usize;
}
