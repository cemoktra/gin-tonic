use crate::{Error, FromWire, IntoWire, TagReader, WireType, WireTypeView};

/// convert a key-value pair into a [WireType]
#[inline(always)]
pub fn into_wire<K, V>(key: K, value: V) -> WireType
where
    K: IntoWire,
    V: IntoWire,
{
    let mut map_buffer = bytes::BytesMut::with_capacity(key.size_hint(1) + value.size_hint(2));

    let wire_type = key.into_wire();
    wire_type.serialize(1, &mut map_buffer);

    let wire_type = value.into_wire();
    wire_type.serialize(2, &mut map_buffer);

    WireType::LengthEncoded(map_buffer.freeze())
}

/// read a key-value pair from a [WireTypeView]
#[inline(always)]
pub fn from_wire<K, V>(wire_type: WireTypeView) -> Result<(K, V), Error>
where
    K: FromWire,
    V: FromWire,
{
    match wire_type {
        WireTypeView::LengthEncoded(data) => {
            let mut reader = TagReader::new(data);

            let key_tag = reader.next().ok_or(Error::MissingField(1))?;
            let value_tag = reader.next().ok_or(Error::MissingField(2))?;

            let (_, key) = key_tag.into_parts();
            let (_, value) = value_tag.into_parts();

            let key = K::from_wire(key)?;
            let value = V::from_wire(value)?;

            Ok((key, value))
        }
        _ => Err(Error::UnexpectedWireType),
    }
}
