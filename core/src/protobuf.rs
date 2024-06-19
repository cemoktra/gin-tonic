pub(crate) mod reader;
pub(crate) mod tag;
pub(crate) mod wire;

pub use wire::{WireType, WireTypeView};

mod scalar;
#[cfg(test)]
mod test;

use crate::protobuf::reader::TagReader;
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

pub mod nested {
    use crate::protobuf::IntoWire;
    use integer_encoding::VarInt;

    pub fn size_hint<T>(tag: u32, message: &T) -> usize
    where
        T: IntoWire,
    {
        let size = message.size_hint(tag);
        tag.required_space() + size.required_space() + size
    }
}

pub mod map {
    use crate::protobuf::reader::TagReader;
    use crate::protobuf::wire::{WireType, WireTypeView};
    use crate::protobuf::{Error, FromWire, IntoWire};
    use std::collections::HashMap;

    pub fn into_wire<K, V>(key: K, value: V) -> Result<WireType, Error>
    where
        K: IntoWire,
        V: IntoWire,
    {
        let mut map_buffer = Vec::with_capacity(key.size_hint(1) + value.size_hint(2));

        let wire_type = key.into_wire();
        wire_type.serialize(1, &mut map_buffer)?;

        let wire_type = value.into_wire();
        wire_type.serialize(2, &mut map_buffer)?;

        Ok(WireType::LengthEncoded(map_buffer))
    }

    pub fn from_wire<K, V>(wire_type: WireTypeView) -> Result<(K, V), Error>
    where
        K: FromWire,
        V: FromWire,
    {
        match wire_type {
            WireTypeView::LengthEncoded(data) => {
                let reader = TagReader::new(data);
                let mut field_map = HashMap::<u32, Vec<WireTypeView>>::new();

                for tag in reader {
                    let (field_number, wire_type) = tag.into_parts();
                    field_map.entry(field_number).or_default().push(wire_type);
                }

                let key = field_map
                    .remove(&1)
                    .ok_or(Error::MissingField(1))?
                    .into_iter()
                    .nth(0)
                    .ok_or(Error::MissingField(1))?;
                let key = K::from_wire(key)?;

                let value = field_map
                    .remove(&2)
                    .ok_or(Error::MissingField(2))?
                    .into_iter()
                    .nth(0)
                    .ok_or(Error::MissingField(2))?;
                let value = V::from_wire(value)?;

                Ok((key, value))
            }
            _ => return Err(Error::UnexpectedWireType),
        }
    }
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

impl From<Error> for tonic::Status {
    fn from(_value: Error) -> Self {
        tonic::Status::internal("TODO")
        // match value {
        //     Error::Io(_) => tonic::Status::
        //     Error::Utf8(_) => {}
        //     Error::UnexpectedWireType => {}
        //     Error::InvalidVarInt => {}
        //     Error::MissingField(_) => {}
        //     Error::Conversion(_) => {}
        //     Error::UnknownEnumVariant(_) => {}
        //     Error::InvalidOneOf => {}
        // }
    }
}
