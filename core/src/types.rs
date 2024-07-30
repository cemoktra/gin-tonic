use crate::{
    decoder::{Decode, DecodeError},
    encoder::{Encode, SizeHint},
    WIRE_TYPE_I32, WIRE_TYPE_I64, WIRE_TYPE_LENGTH_ENCODED, WIRE_TYPE_VARINT,
};

pub const fn sizeof_varint32(v: u32) -> usize {
    match v {
        0x0..=0x7F => 1,
        0x80..=0x3FFF => 2,
        0x4000..=0x1FFFFF => 3,
        0x200000..=0xFFFFFFF => 4,
        _ => 5,
    }
}

pub const fn sizeof_varint64(v: u64) -> usize {
    const U32_MAX: u64 = u32::MAX as u64;
    const U32_OVER_MAX: u64 = U32_MAX + 1;
    match v {
        0x0..=U32_MAX => sizeof_varint32(v as u32),
        U32_OVER_MAX..=0x7FFFFFFFF => 5,
        0x0800000000..=0x3FFFFFFFFFF => 6,
        0x040000000000..=0x1FFFFFFFFFFFF => 7,
        0x02000000000000..=0xFFFFFFFFFFFFFF => 8,
        0x0100000000000000..=0x7FFFFFFFFFFFFFFF => 9,
        _ => 10,
    }
}

pub trait PbType {
    const WIRE_TYPE: u8;

    fn size_hint(&self) -> usize {
        let mut hint = SizeHint::default();
        self.encode(&mut hint);
        hint.size()
    }

    fn encode(&self, encoder: &mut impl Encode);
    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized;
}

pub trait PbOneOf {
    fn size_hint(&self) -> usize {
        let mut hint = SizeHint::default();
        self.encode(&mut hint);
        hint.size()
    }

    fn encode(&self, encoder: &mut impl Encode);
    fn decode(
        field_number: u32,
        wire_type: u8,
        decoder: &mut impl Decode,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;

    fn matches(field_number: u32) -> bool;
}

pub struct Int32(pub i32);
pub struct Int64(pub i64);
pub struct UInt32(pub u32);
pub struct UInt64(pub u64);
pub struct SInt32(pub i32);
pub struct SInt64(pub i64);
pub struct Fixed32(pub u32);
pub struct Fixed64(pub u64);
pub struct SFixed32(pub i32);
pub struct SFixed64(pub i64);

impl PbType for Int32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_int32(self.0)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_int32().map(Self)
    }
}

impl PbType for Int64 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_int64(self.0)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_int64().map(Self)
    }
}

impl PbType for UInt32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_uint32(self.0)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_uint32().map(Self)
    }
}

impl PbType for UInt64 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_uint64(self.0)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_uint64().map(Self)
    }
}

impl PbType for SInt32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_sint32(self.0)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_sint32().map(Self)
    }
}

impl PbType for SInt64 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_sint64(self.0)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_sint64().map(Self)
    }
}

impl PbType for Fixed32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_I32;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_fixed32(self.0)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_fixed32().map(Self)
    }
}

impl PbType for Fixed64 {
    const WIRE_TYPE: u8 = WIRE_TYPE_I64;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_fixed64(self.0)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_fixed64().map(Self)
    }
}

impl PbType for SFixed32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_I32;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_sfixed32(self.0)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_sfixed32().map(Self)
    }
}

impl PbType for SFixed64 {
    const WIRE_TYPE: u8 = WIRE_TYPE_I64;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_sfixed64(self.0)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_sfixed64().map(Self)
    }
}

impl PbType for f32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_I32;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_float(*self)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_float()
    }
}

impl PbType for f64 {
    const WIRE_TYPE: u8 = WIRE_TYPE_I64;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_double(*self)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_double()
    }
}

impl PbType for bool {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_bool(*self)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_bool()
    }
}

impl PbType for String {
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_str(&self)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_string()
    }
}

impl PbType for std::net::Ipv4Addr {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_uint32(self.to_bits())
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_uint32().map(std::net::Ipv4Addr::from_bits)
    }
}

impl PbType for std::path::PathBuf {
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_str(self.display().to_string().as_ref())
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_string().map(Into::into)
    }
}

#[cfg(feature = "uuid_string")]
impl PbType for uuid::Uuid {
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl Encode) {
        encoder.encode_string(self.as_simple().to_string().as_ref())
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder
            .decode_string()?
            .parse()
            .map_err(|err| DecodeError::Conversion(Box::new(err)))
    }
}

#[cfg(feature = "uuid_bytes")]
impl PbType for uuid::Uuid {
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl Encode) {
        let (high, low) = self.as_u64_pair();
        encoder.encode_uint32(2);
        encoder.encode_uint64(high);
        encoder.encode_uint64(low);
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let len = decoder.decode_uint32()?;
        if len != 2 {
            todo!("error for unvalid length")
        }
        let high = decoder.decode_uint64()?;
        let low = decoder.decode_uint64()?;
        Ok(Self::from_u64_pair(high, low))
    }
}
