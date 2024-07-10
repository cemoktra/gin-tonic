use crate::{Error, FromWire, TagReader, WireTypeView};

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
