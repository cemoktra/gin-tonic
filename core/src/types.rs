use crate::{
    decoder::{Decode, DecodeError},
    encoder::{zigzag_encode, Encode},
    WIRE_TYPE_I32, WIRE_TYPE_I64, WIRE_TYPE_LENGTH_ENCODED, WIRE_TYPE_VARINT,
};

const fn sizeof_varint32(v: u32) -> usize {
    match v {
        0x0..=0x7F => 1,
        0x80..=0x3FFF => 2,
        0x4000..=0x1FFFFF => 3,
        0x200000..=0xFFFFFFF => 4,
        _ => 5,
    }
}

const fn sizeof_varint64(v: u64) -> usize {
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

pub trait PbType<T = ()> {
    const WIRE_TYPE: u8;

    fn size_hint(&self) -> usize;

    fn encode(self, encoder: &mut impl Encode);
    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized;
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

    fn size_hint(&self) -> usize {
        if self.0 >= 0 {
            sizeof_varint32(self.0 as _)
        } else {
            10
        }
    }

    fn encode(self, encoder: &mut impl Encode) {
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

    fn size_hint(&self) -> usize {
        sizeof_varint64(self.0 as _)
    }

    fn encode(self, encoder: &mut impl Encode) {
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

    fn size_hint(&self) -> usize {
        sizeof_varint32(self.0 as _)
    }

    fn encode(self, encoder: &mut impl Encode) {
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

    fn size_hint(&self) -> usize {
        sizeof_varint64(self.0 as _)
    }

    fn encode(self, encoder: &mut impl Encode) {
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

    fn size_hint(&self) -> usize {
        sizeof_varint32(zigzag_encode(self.0 as _) as _)
    }

    fn encode(self, encoder: &mut impl Encode) {
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

    fn size_hint(&self) -> usize {
        sizeof_varint64(zigzag_encode(self.0) as _)
    }

    fn encode(self, encoder: &mut impl Encode) {
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

    fn size_hint(&self) -> usize {
        std::mem::size_of::<i32>()
    }

    fn encode(self, encoder: &mut impl Encode) {
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

    fn size_hint(&self) -> usize {
        std::mem::size_of::<u64>()
    }

    fn encode(self, encoder: &mut impl Encode) {
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

    fn size_hint(&self) -> usize {
        std::mem::size_of::<i32>()
    }

    fn encode(self, encoder: &mut impl Encode) {
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

    fn size_hint(&self) -> usize {
        std::mem::size_of::<i64>()
    }

    fn encode(self, encoder: &mut impl Encode) {
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

    fn size_hint(&self) -> usize {
        std::mem::size_of::<f32>()
    }

    fn encode(self, encoder: &mut impl Encode) {
        encoder.encode_float(self)
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

    fn size_hint(&self) -> usize {
        std::mem::size_of::<f64>()
    }

    fn encode(self, encoder: &mut impl Encode) {
        encoder.encode_double(self)
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

    fn size_hint(&self) -> usize {
        1
    }

    fn encode(self, encoder: &mut impl Encode) {
        encoder.encode_bool(self)
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

    fn size_hint(&self) -> usize {
        sizeof_varint32(self.len() as u32) + self.as_bytes().len()
    }

    fn encode(self, encoder: &mut impl Encode) {
        encoder.encode_string(&self)
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        decoder.decode_string()
    }
}
