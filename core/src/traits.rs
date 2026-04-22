use crate::{Tag, encoder::SizeHint, error::ProtoError};

pub trait Scalar<ProtobufType> {
    const WIRE_TYPE: u8;

    fn size_hint(&self) -> usize {
        let mut hint = SizeHint::default();
        self.encode(&mut hint);
        hint.size()
    }

    fn encode(&self, encoder: &mut impl Encode);
    fn decode(decoder: &mut impl Decode) -> Result<Self, ProtoError>
    where
        Self: Sized;

    /// helper to serialize type with field number
    fn encode_field(&self, field_number: u32, encoder: &mut impl Encode) {
        encoder.encode_tag(Tag::from_parts(field_number, Self::WIRE_TYPE));
        self.encode(encoder);
    }
}

pub trait Encode {
    #[inline]
    fn encode_tag(&mut self, tag: Tag) {
        self.encode_uint32(tag.into());
    }

    #[inline]
    fn encode_int32(&mut self, n: i32) {
        self.encode_uint64(n as u64);
    }
    #[inline]
    fn encode_int64(&mut self, n: i64) {
        self.encode_uint64(n as u64);
    }

    fn encode_sint32(&mut self, n: i32);
    fn encode_sint64(&mut self, n: i64);

    fn encode_uint32(&mut self, n: u32);
    fn encode_uint64(&mut self, n: u64);

    fn encode_sfixed32(&mut self, n: i32);
    fn encode_sfixed64(&mut self, n: i64);

    fn encode_fixed32(&mut self, n: u32);
    fn encode_fixed64(&mut self, n: u64);

    fn encode_float(&mut self, n: f32);
    fn encode_double(&mut self, n: f64);

    #[inline]
    fn encode_bool(&mut self, b: bool) {
        self.encode_uint32(if b { 1 } else { 0 });
    }

    fn encode_bytes(&mut self, b: &[u8]);

    #[inline]
    fn encode_str(&mut self, s: &str) {
        self.encode_bytes(s.as_bytes())
    }
    #[inline]
    fn encode_string(&mut self, s: String) {
        self.encode_bytes(s.as_bytes())
    }
}

#[allow(clippy::len_without_is_empty)]
pub trait Decode {
    fn buffer(&self) -> &[u8];
    fn len(&self) -> usize;
    fn position(&self) -> usize;
    fn advance(&mut self, size: usize);
    fn eof(&self) -> bool;
    fn sub_decoder(&mut self, size: usize) -> impl Decode;

    #[inline]
    fn decode_int32(&mut self) -> Result<i32, ProtoError> {
        let v = self.decode_uint64()?;
        #[allow(clippy::cast_possible_truncation)]
        Ok(v as _)
    }
    #[inline]
    fn decode_int64(&mut self) -> Result<i64, ProtoError> {
        let v = self.decode_uint64()?;
        Ok(v as _)
    }

    fn decode_sint32(&mut self) -> Result<i32, ProtoError>;
    fn decode_sint64(&mut self) -> Result<i64, ProtoError>;

    fn decode_uint32(&mut self) -> Result<u32, ProtoError>;
    fn decode_uint64(&mut self) -> Result<u64, ProtoError>;

    fn decode_sfixed32(&mut self) -> Result<i32, ProtoError>;
    fn decode_sfixed64(&mut self) -> Result<i64, ProtoError>;

    fn decode_fixed32(&mut self) -> Result<u32, ProtoError>;
    fn decode_fixed64(&mut self) -> Result<u64, ProtoError>;

    fn decode_float(&mut self) -> Result<f32, ProtoError>;
    fn decode_double(&mut self) -> Result<f64, ProtoError>;

    fn decode_bytes(&mut self) -> Result<Vec<u8>, ProtoError>;

    #[inline]
    fn decode_string(&mut self) -> Result<String, ProtoError> {
        Ok(String::from_utf8(self.decode_bytes()?)?)
    }

    #[inline]
    fn decode_bool(&mut self) -> Result<bool, ProtoError> {
        Ok(self.decode_uint32()? != 0)
    }

    #[inline]
    fn decode_tag(&mut self) -> Result<Tag, ProtoError> {
        Ok(Tag::from(self.decode_uint32()?))
    }
}

pub trait Message {
    fn message_size_hint(&self) -> usize {
        let mut hint = SizeHint::default();
        self.encode_message(&mut hint);
        hint.size()
    }

    fn encode_message(&self, encoder: &mut impl Encode);

    fn decode_message(decoder: &mut impl Decode) -> Result<Self, ProtoError>
    where
        Self: Sized;
}

pub trait PackableMarker<ProtobufType> {}

pub trait Packed<ProtobufType> {
    type Rust;

    fn size_hint(&self, field_number: u32) -> usize {
        let mut hint = SizeHint::default();
        self.encode(field_number, &mut hint);
        hint.size()
    }

    fn encode(&self, field_number: u32, encoder: &mut impl Encode);

    fn decode(decoder: &mut impl Decode, v: &mut Self) -> Result<(), ProtoError>
    where
        Self: Sized;
}

pub trait Unpacked<ProtobufType> {
    type Rust;

    fn size_hint(&self, tag: Tag) -> usize {
        let mut hint = SizeHint::default();
        self.encode(tag, &mut hint);
        hint.size()
    }

    fn encode(&self, tag: Tag, encoder: &mut impl Encode);
}

pub trait Map<ProtobufKey, ProtobufValue> {
    fn size_hint(&self, field_number: u32) -> usize {
        let mut hint = SizeHint::default();
        self.encode(field_number, &mut hint);
        hint.size()
    }

    fn encode(&self, field_number: u32, encoder: &mut impl Encode);

    fn decode(decoder: &mut impl Decode, m: &mut Self) -> Result<(), ProtoError>
    where
        Self: Sized;
}
