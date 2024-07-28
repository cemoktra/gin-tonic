use std::string::FromUtf8Error;

use crate::types::PbType;

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("VarInt limitof 10 bytes reached")]
    VarIntLimit,
    #[error("Unexpected wire type: expected {0}, actual {1}")]
    UnexpectedWireType(u8, u8),
    #[error("Unexpected field number {0}")]
    UnexpectedFieldNumber(u32),
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    #[error(transparent)]
    Conversion(Box<dyn std::error::Error>),
}

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

    fn encode_type<T>(&mut self) -> Result<T, DecodeError>
    where
        T: PbType;
}

#[inline]
pub fn zigzag_decode(from: u64) -> i64 {
    ((from >> 1) ^ (-((from & 1) as i64)) as u64) as i64
}

impl<T> Decode for T
where
    T: bytes::Buf,
{
    fn eof(&self) -> bool {
        !self.has_remaining()
    }

    fn decode_float(&mut self) -> Result<f32, DecodeError> {
        Ok(self.get_f32_le())
    }

    fn decode_double(&mut self) -> Result<f64, DecodeError> {
        Ok(self.get_f64_le())
    }

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

    fn decode_int32(&mut self) -> Result<i32, DecodeError> {
        self.decode_varint().map(|u| u as i32)
    }

    fn decode_int64(&mut self) -> Result<i64, DecodeError> {
        self.decode_varint().map(|u| u as i64)
    }

    fn decode_uint32(&mut self) -> Result<u32, DecodeError> {
        self.decode_varint().map(|u| u as u32)
    }

    fn decode_uint64(&mut self) -> Result<u64, DecodeError> {
        self.decode_varint()
    }

    fn decode_sint32(&mut self) -> Result<i32, DecodeError> {
        self.decode_varint().map(|u| zigzag_decode(u) as i32)
    }

    fn decode_sint64(&mut self) -> Result<i64, DecodeError> {
        self.decode_varint().map(|u| zigzag_decode(u))
    }

    fn decode_fixed32(&mut self) -> Result<u32, DecodeError> {
        Ok(self.get_u32_le())
    }

    fn decode_fixed64(&mut self) -> Result<u64, DecodeError> {
        Ok(self.get_u64_le())
    }

    fn decode_sfixed32(&mut self) -> Result<i32, DecodeError> {
        Ok(self.get_i32_le())
    }

    fn decode_sfixed64(&mut self) -> Result<i64, DecodeError> {
        Ok(self.get_i64_le())
    }

    fn decode_bool(&mut self) -> Result<bool, DecodeError> {
        Ok(self.get_u8() != 0)
    }

    fn decode_bytes(&mut self) -> Result<bytes::Bytes, DecodeError> {
        let len = self.decode_uint32()?;
        Ok(self.copy_to_bytes(len as usize))
    }

    fn decode_string(&mut self) -> Result<String, DecodeError> {
        let bytes = self.decode_bytes()?;
        Ok(String::from_utf8(bytes.to_vec())?)
    }

    fn encode_type<M>(&mut self) -> Result<M, DecodeError>
    where
        M: PbType,
    {
        M::decode(self)
    }
}

#[cfg(test)]
mod tests {
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
