use std::hash::Hash;

use indexmap::IndexMap;

use crate::{
    Decode, Encode, Map, RawMessageView, Scalar, Tag, decoder::Decoder, encoder::SizeHint,
    error::ProtoError, wire_types::WIRE_TYPE_LENGTH_ENCODED,
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

pub struct KeyValuePairOwned<RustKey, RustValue> {
    pub key: RustKey,
    pub value: RustValue,
}

impl<RustKey, RustValue> KeyValuePairOwned<RustKey, RustValue> {
    pub fn decode<ProtobufKey, ProtobufValue>(buffer: &[u8]) -> Result<Self, ProtoError>
    where
        RustKey: Scalar<ProtobufKey>,
        RustValue: Scalar<ProtobufValue>,
    {
        let raw_message = RawMessageView::try_from(buffer)?;

        let tag = Tag::from_parts(1u32, <RustKey as Scalar<ProtobufKey>>::WIRE_TYPE);
        let bytes = raw_message
            .tag_data(tag)
            .next_back()
            .ok_or(ProtoError::MissingField(1))?;
        let mut decoder = Decoder::new(bytes);
        let key = <RustKey as Scalar<ProtobufKey>>::decode(&mut decoder)?;

        let tag = Tag::from_parts(2u32, <RustValue as Scalar<ProtobufValue>>::WIRE_TYPE);
        let bytes = raw_message
            .tag_data(tag)
            .next_back()
            .ok_or(ProtoError::MissingField(2))?;
        let mut decoder = Decoder::new(bytes);
        let value = <RustValue as Scalar<ProtobufValue>>::decode(&mut decoder)?;

        Ok(Self { key, value })
    }
}

impl<RustKey, ProtobufKey, RustValue, ProtobufValue> Map<ProtobufKey, ProtobufValue>
    for IndexMap<RustKey, RustValue>
where
    RustKey: Scalar<ProtobufKey> + Hash + Eq,
    RustValue: Scalar<ProtobufValue>,
{
    fn encode(&self, field_number: u32, encoder: &mut impl Encode) {
        for (key, value) in self.iter() {
            encoder.encode_tag(Tag::from_parts(field_number, WIRE_TYPE_LENGTH_ENCODED));

            let pair = KeyValuePairView { key, value };

            encoder.encode_uint64(pair.size_hint() as u64);
            pair.encode(encoder);
        }
    }

    fn decode<'buf>(
        field_number: u32,
        raw_message: &'buf RawMessageView<'buf>,
    ) -> Result<Self, ProtoError>
    where
        Self: Sized,
    {
        let mut map = Self::new();

        for buffer in raw_message.tag_data(Tag::from_parts(field_number, WIRE_TYPE_LENGTH_ENCODED))
        {
            let mut decoder = Decoder::new(buffer);
            let _size = decoder.decode_uint64()?;

            let pair = KeyValuePairOwned::<RustKey, RustValue>::decode::<ProtobufKey, ProtobufValue>(
                decoder.buffer(),
            )?;
            map.insert(pair.key, pair.value);
        }

        Ok(map)
    }
}
