#![allow(clippy::cast_possible_truncation)]

use std::string::FromUtf8Error;

use crate::{tag::Tag, types::PbType};

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
/// decoding protobuf can end up in errors which are handled in this enumeration
pub enum DecodeError {
    #[error("VarInt limitof 10 bytes reached")]
    /// VarInt limitof 10 bytes reached
    VarIntLimit,
    #[error("Unexpected wire type: expected {0}, actual {1}")]
    /// an unexpected wire type was read from the tag
    UnexpectedWireType(u8, u8),
    #[error("Unexpected field number {0}")]
    /// an unexpected/unhandled field number was read
    UnexpectedFieldNumber(u32),
    #[error("Unexpected enum variant {0}")]
    /// an unexpected enum variant was read
    UnexpectedEnumVariant(i32),
    #[error("Unexpected oneof variant {0}")]
    /// an unexpected oneof field number was read
    UnexpectedOneOfVariant(u32),
    #[error("Field number {0} is missing")]
    /// an expected field number was not read
    MissingField(u32),
    #[error("OneOf of field numbers {0:?} is missing")]
    /// no variants field number of the oneof was found
    MissingOneOf(Vec<u32>),
    #[error(transparent)]
    /// converting bytes into UTF-8 string failed
    Utf8(#[from] FromUtf8Error),
    #[error(transparent)]
    /// generic conversion error occured while converting the protobuf representation into an actual type
    Conversion(Box<dyn std::error::Error>),
}

/// main decode trait, currenlty implemented for anything that implements [bytes::Buf]
pub trait Decode {
    fn eof(&self) -> bool;

    fn decode_float(&mut self) -> Result<f32, DecodeError>;
    fn decode_double(&mut self) -> Result<f64, DecodeError>;

    fn decode_varint(&mut self) -> Result<u64, DecodeError>;

    fn decode_int32(&mut self) -> Result<i32, DecodeError>;
    fn decode_int64(&mut self) -> Result<i64, DecodeError>;

    fn decode_uint32(&mut self) -> Result<u32, DecodeError>;
    fn decode_uint64(&mut self) -> Result<u64, DecodeError>;

    fn decode_sint32(&mut self) -> Result<i32, DecodeError>;
    fn decode_sint64(&mut self) -> Result<i64, DecodeError>;

    fn decode_fixed32(&mut self) -> Result<u32, DecodeError>;
    fn decode_fixed64(&mut self) -> Result<u64, DecodeError>;

    fn decode_sfixed32(&mut self) -> Result<i32, DecodeError>;
    fn decode_sfixed64(&mut self) -> Result<i64, DecodeError>;

    fn decode_bool(&mut self) -> Result<bool, DecodeError>;
    fn decode_bytes(&mut self) -> Result<bytes::Bytes, DecodeError>;
    fn decode_string(&mut self) -> Result<String, DecodeError>;

    fn decode_nested<N>(&mut self) -> Result<N, DecodeError>
    where
        N: PbType;

    fn decode_type<T>(&mut self) -> Result<T, DecodeError>
    where
        T: PbType;

    fn decode_packed<M, F>(&mut self, buffer: &mut Vec<M>, decode_fn: F) -> Result<(), DecodeError>
    where
        M: Clone,
        F: Fn(&mut Self) -> Result<M, DecodeError>;

    fn decode_map_element<K, V, KF, VF>(
        &mut self,
        decode_key_fn: KF,
        decode_key_fn: VF,
    ) -> Result<Option<(K, V)>, DecodeError>
    where
        KF: Fn(&mut Self) -> Result<K, DecodeError>,
        VF: Fn(&mut Self) -> Result<V, DecodeError>;
}

#[inline]
fn zigzag_decode(from: u64) -> i64 {
    ((from >> 1) ^ (-((from & 1) as i64)) as u64) as i64
}

impl<T> Decode for T
where
    T: bytes::Buf,
{
    #[inline]
    fn eof(&self) -> bool {
        !self.has_remaining()
    }

    #[inline]
    fn decode_float(&mut self) -> Result<f32, DecodeError> {
        Ok(self.get_f32_le())
    }

    #[inline]
    fn decode_double(&mut self) -> Result<f64, DecodeError> {
        Ok(self.get_f64_le())
    }

    #[inline]
    fn decode_varint(&mut self) -> Result<u64, DecodeError> {
        let b = self.get_u8();
        if b & 0x80 == 0 {
            return Ok(b as u64);
        }

        let mut varint: u64 = b as u64 & !0x80;
        let mut bitpos = 7;

        for _ in 1..10 {
            let b = self.get_u8();
            let u = b & !0x80;
            varint |= (u as u64) << bitpos;
            bitpos += 7;
            if b & 0x80 == 0 {
                return Ok(varint);
            }
        }

        Err(DecodeError::VarIntLimit)
    }

    #[inline]
    fn decode_int32(&mut self) -> Result<i32, DecodeError> {
        self.decode_varint().map(|u| u as i32)
    }

    #[inline]
    fn decode_int64(&mut self) -> Result<i64, DecodeError> {
        self.decode_varint().map(|u| u as i64)
    }

    #[inline]
    fn decode_uint32(&mut self) -> Result<u32, DecodeError> {
        self.decode_varint().map(|u| u as u32)
    }

    #[inline]
    fn decode_uint64(&mut self) -> Result<u64, DecodeError> {
        self.decode_varint()
    }

    #[inline]
    fn decode_sint32(&mut self) -> Result<i32, DecodeError> {
        self.decode_varint().map(|u| zigzag_decode(u) as i32)
    }

    #[inline]
    fn decode_sint64(&mut self) -> Result<i64, DecodeError> {
        self.decode_varint().map(zigzag_decode)
    }

    #[inline]
    fn decode_fixed32(&mut self) -> Result<u32, DecodeError> {
        Ok(self.get_u32_le())
    }

    #[inline]
    fn decode_fixed64(&mut self) -> Result<u64, DecodeError> {
        Ok(self.get_u64_le())
    }

    #[inline]
    fn decode_sfixed32(&mut self) -> Result<i32, DecodeError> {
        Ok(self.get_i32_le())
    }

    #[inline]
    fn decode_sfixed64(&mut self) -> Result<i64, DecodeError> {
        Ok(self.get_i64_le())
    }

    #[inline]
    fn decode_bool(&mut self) -> Result<bool, DecodeError> {
        Ok(self.get_u8() != 0)
    }

    #[inline]
    fn decode_bytes(&mut self) -> Result<bytes::Bytes, DecodeError> {
        let len = self.decode_uint32()?;
        Ok(self.copy_to_bytes(len as usize))
    }

    #[inline]
    fn decode_string(&mut self) -> Result<String, DecodeError> {
        let bytes = self.decode_bytes()?;
        Ok(String::from_utf8(bytes.to_vec())?)
    }

    #[inline]
    fn decode_type<M>(&mut self) -> Result<M, DecodeError>
    where
        M: PbType,
    {
        M::decode(self)
    }

    #[inline]
    fn decode_packed<M, F>(&mut self, buffer: &mut Vec<M>, decode_fn: F) -> Result<(), DecodeError>
    where
        M: Clone,
        F: Fn(&mut Self) -> Result<M, DecodeError>,
    {
        let len = self.decode_uint32()? as usize;
        if len == 0 {
            return Ok(());
        }
        let remaining_before = self.remaining();

        loop {
            buffer.push(decode_fn(self)?);
            if remaining_before - self.remaining() == len {
                break;
            }
        }

        Ok(())
    }

    #[inline]
    fn decode_map_element<K, V, KF, VF>(
        &mut self,
        decode_key_fn: KF,
        decode_value_fn: VF,
    ) -> Result<Option<(K, V)>, DecodeError>
    where
        KF: Fn(&mut Self) -> Result<K, DecodeError>,
        VF: Fn(&mut Self) -> Result<V, DecodeError>,
    {
        let mut key = None;
        let mut value = None;

        let len = self.decode_uint32()? as usize;
        let remaining_before = self.remaining();

        loop {
            let tag = self.decode_uint32()?;
            let field_number = tag.field_number();

            match field_number {
                1 => {
                    key = Some(decode_key_fn(self)?);
                }
                2 => {
                    value = Some(decode_value_fn(self)?);
                }
                _ => {}
            }

            if remaining_before - self.remaining() == len {
                break;
            }
        }

        if let (Some(key), Some(value)) = (key, value) {
            Ok(Some((key, value)))
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn decode_nested<N>(&mut self) -> Result<N, DecodeError>
    where
        N: PbType,
    {
        let len = self.decode_uint32()? as usize;
        let remaining_before = self.remaining();

        let result = {
            let mut nested_buf = &self.chunk()[0..len];
            N::decode(&mut nested_buf)
        }?;
        self.advance(len);
        if remaining_before - self.remaining() != len {
            // TODO: error?
            panic!(
                "read different amount of bytes: len = {len}, read = {}",
                remaining_before - self.remaining()
            );
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    // not using the constants as testing against protobuf pal would get difficult then
    #![allow(clippy::approx_constant)]

    use super::Decode;

    #[test]
    fn decode_float() {
        let mut buffer = bytes::Bytes::from_static(&[0xc3, 0xf5, 0x48, 0x40]);
        let value = buffer.decode_float().unwrap();
        assert_eq!(value, 3.14);
    }

    #[test]
    fn decode_double() {
        let mut buffer =
            bytes::Bytes::from_static(&[0x1f, 0x85, 0xeb, 0x51, 0xb8, 0x1e, 0x09, 0x40]);
        let value = buffer.decode_double().unwrap();
        assert_eq!(value, 3.14);
    }

    #[test]
    fn decode_int32() {
        let mut buffer = bytes::Bytes::from_static(&[0x01]);
        let value = buffer.decode_int32().unwrap();
        assert_eq!(value, 1);

        let mut buffer = bytes::Bytes::from_static(&[0xd2, 0x09]);
        let value = buffer.decode_int32().unwrap();
        assert_eq!(value, 1234);

        let mut buffer = bytes::Bytes::from_static(&[
            0xae, 0xf6, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
        ]);
        let value = buffer.decode_int32().unwrap();
        assert_eq!(value, -1234);
    }

    #[test]
    fn decode_int64() {
        let mut buffer = bytes::Bytes::from_static(&[0x01]);
        let value = buffer.decode_int64().unwrap();
        assert_eq!(value, 1);

        let mut buffer = bytes::Bytes::from_static(&[0xd2, 0x09]);
        let value = buffer.decode_int64().unwrap();
        assert_eq!(value, 1234);

        let mut buffer = bytes::Bytes::from_static(&[
            0xae, 0xf6, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01,
        ]);
        let value = buffer.decode_int64().unwrap();
        assert_eq!(value, -1234);
    }

    #[test]
    fn decode_uint32() {
        let mut buffer = bytes::Bytes::from_static(&[0x01]);
        let value = buffer.decode_uint32().unwrap();
        assert_eq!(value, 1);

        let mut buffer = bytes::Bytes::from_static(&[0xd2, 0x09]);
        let value = buffer.decode_uint32().unwrap();
        assert_eq!(value, 1234);
    }

    #[test]
    fn decode_uint64() {
        let mut buffer = bytes::Bytes::from_static(&[0x01]);
        let value = buffer.decode_uint64().unwrap();
        assert_eq!(value, 1);

        let mut buffer = bytes::Bytes::from_static(&[0xd2, 0x09]);
        let value = buffer.decode_uint64().unwrap();
        assert_eq!(value, 1234);
    }

    #[test]
    fn decode_sint32() {
        let mut buffer = bytes::Bytes::from_static(&[0x02]);
        let value = buffer.decode_sint32().unwrap();
        assert_eq!(value, 1);

        let mut buffer = bytes::Bytes::from_static(&[0xa4, 0x13]);
        let value = buffer.decode_sint32().unwrap();
        assert_eq!(value, 1234);

        let mut buffer = bytes::Bytes::from_static(&[0xa3, 0x13]);
        let value = buffer.decode_sint32().unwrap();
        assert_eq!(value, -1234);
    }

    #[test]
    fn decode_sin64() {
        let mut buffer = bytes::Bytes::from_static(&[0x02]);
        let value = buffer.decode_sint64().unwrap();
        assert_eq!(value, 1);

        let mut buffer = bytes::Bytes::from_static(&[0xa4, 0x13]);
        let value = buffer.decode_sint64().unwrap();
        assert_eq!(value, 1234);

        let mut buffer = bytes::Bytes::from_static(&[0xa3, 0x13]);
        let value = buffer.decode_sint64().unwrap();
        assert_eq!(value, -1234);
    }

    #[test]
    fn decode_fixed32() {
        let mut buffer = bytes::Bytes::from_static(&[0x01, 0x00, 0x00, 0x00]);
        let value = buffer.decode_fixed32().unwrap();
        assert_eq!(value, 1);

        let mut buffer = bytes::Bytes::from_static(&[0xd2, 0x04, 0x00, 0x00]);
        let value = buffer.decode_fixed32().unwrap();
        assert_eq!(value, 1234);
    }

    #[test]
    fn decode_fixed64() {
        let mut buffer =
            bytes::Bytes::from_static(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let value = buffer.decode_fixed64().unwrap();
        assert_eq!(value, 1);

        let mut buffer =
            bytes::Bytes::from_static(&[0xd2, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let value = buffer.decode_fixed64().unwrap();
        assert_eq!(value, 1234);
    }

    #[test]
    fn decode_sfixed32() {
        let mut buffer = bytes::Bytes::from_static(&[0x01, 0x00, 0x00, 0x00]);
        let value = buffer.decode_sfixed32().unwrap();
        assert_eq!(value, 1);

        let mut buffer = bytes::Bytes::from_static(&[0xd2, 0x04, 0x00, 0x00]);
        let value = buffer.decode_sfixed32().unwrap();
        assert_eq!(value, 1234);

        let mut buffer = bytes::Bytes::from_static(&[0x2e, 0xfb, 0xff, 0xff]);
        let value = buffer.decode_sfixed32().unwrap();
        assert_eq!(value, -1234);
    }

    #[test]
    fn decode_sfixed64() {
        let mut buffer =
            bytes::Bytes::from_static(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let value = buffer.decode_sfixed64().unwrap();
        assert_eq!(value, 1);

        let mut buffer =
            bytes::Bytes::from_static(&[0xd2, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        let value = buffer.decode_sfixed64().unwrap();
        assert_eq!(value, 1234);

        let mut buffer =
            bytes::Bytes::from_static(&[0x2e, 0xfb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        let value = buffer.decode_sfixed64().unwrap();
        assert_eq!(value, -1234);
    }

    #[test]
    fn decode_bool() {
        let mut buffer = bytes::Bytes::from_static(&[0x00]);
        let value = buffer.decode_bool().unwrap();
        assert!(!value);

        let mut buffer = bytes::Bytes::from_static(&[0x01]);
        let value = buffer.decode_bool().unwrap();
        assert!(value);
    }

    #[test]
    fn decode_string() {
        let mut buffer = bytes::Bytes::from_static(&[0x00]);
        let value = buffer.decode_string().unwrap();
        assert!(value.is_empty());

        let mut buffer = bytes::Bytes::from_static(&[0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f]);
        let value = buffer.decode_string().unwrap();
        assert_eq!(value.as_str(), "hello");
    }
}
