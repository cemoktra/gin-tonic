
use crate::{TagReader, IntoWire, FromWire, Error,WireType,WireTypeView};
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
                .next()
                .ok_or(Error::MissingField(1))?;
            let key = K::from_wire(key)?;

            let value = field_map
                .remove(&2)
                .ok_or(Error::MissingField(2))?
                .into_iter()
                .next()
                .ok_or(Error::MissingField(2))?;
            let value = V::from_wire(value)?;

            Ok((key, value))
        }
        _ => Err(Error::UnexpectedWireType),
    }
}
