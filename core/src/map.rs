use std::{collections::HashMap, hash::Hash};

use crate::{
    Decode, Encode, Map, Scalar, Tag, encoder::SizeHint, error::ProtoError,
    wire_types::WIRE_TYPE_LENGTH_ENCODED,
};

pub struct KeyValuePairView<'p, RustKey, RustValue> {
    pub key: &'p RustKey,
    pub value: &'p RustValue,
}

impl<'p, RustKey, RustValue> KeyValuePairView<'p, RustKey, RustValue> {
    fn size_hint<ProtobufKey, ProtobufValue>(&self) -> usize
    where
        RustKey: Scalar<ProtobufKey>,
        RustValue: Scalar<ProtobufValue>,
    {
        let mut hint = SizeHint::default();
        self.encode(&mut hint);
        hint.size()
    }

    fn encode<ProtobufKey, ProtobufValue>(&self, encoder: &mut impl Encode)
    where
        RustKey: Scalar<ProtobufKey>,
        RustValue: Scalar<ProtobufValue>,
    {
        let tag = Tag::from_parts(1u32, <RustKey as Scalar<ProtobufKey>>::WIRE_TYPE);
        encoder.encode_tag(tag);
        <RustKey as Scalar<ProtobufKey>>::encode(self.key, encoder);

        let tag = Tag::from_parts(2u32, <RustValue as Scalar<ProtobufValue>>::WIRE_TYPE);
        encoder.encode_tag(tag);
        <RustValue as Scalar<ProtobufValue>>::encode(self.value, encoder);
    }
}

impl<RustKey, ProtobufKey, RustValue, ProtobufValue> Map<ProtobufKey, ProtobufValue>
    for HashMap<RustKey, RustValue>
where
    RustKey: Scalar<ProtobufKey> + Hash + Eq,
    RustValue: Scalar<ProtobufValue>,
{
    #[inline]
    fn encode(&self, field_number: u32, encoder: &mut impl Encode) {
        for (key, value) in self.iter() {
            encoder.encode_tag(Tag::from_parts(field_number, WIRE_TYPE_LENGTH_ENCODED));

            let pair = KeyValuePairView { key, value };

            encoder.encode_uint64(pair.size_hint() as u64);
            pair.encode(encoder);
        }
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation)]
    fn decode<'buf>(decoder: &mut impl Decode, map: &mut Self) -> Result<(), ProtoError>
    where
        Self: Sized,
    {
        let size = decoder.decode_uint64()? as usize;
        let mut entry = decoder.sub_decoder(size);

        let tag1 = entry.decode_tag()?;
        let (key, value) = if tag1.field_number() == 1 {
            let key = <RustKey as Scalar<ProtobufKey>>::decode(&mut entry)?;
            entry.decode_tag()?;
            let value = <RustValue as Scalar<ProtobufValue>>::decode(&mut entry)?;
            (key, value)
        } else {
            let value = <RustValue as Scalar<ProtobufValue>>::decode(&mut entry)?;
            entry.decode_tag()?;
            let key = <RustKey as Scalar<ProtobufKey>>::decode(&mut entry)?;
            (key, value)
        };

        map.insert(key, value);
        Ok(())
    }
}
