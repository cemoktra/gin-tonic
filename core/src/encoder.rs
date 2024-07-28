use crate::message::EncodeMessage;

pub trait Encode {
    fn encode_float(&mut self, n: f32);
    fn encode_double(&mut self, n: f64);

    fn encode_varint(&mut self, n: u64);

    fn encode_int32(&mut self, n: i32);
    fn encode_int64(&mut self, n: i64);

    fn encode_uint32(&mut self, n: u32);
    fn encode_uint64(&mut self, n: u64);

    fn encode_sint32(&mut self, n: i32);
    fn encode_sint64(&mut self, n: i64);

    fn encode_fixed32(&mut self, n: u32);
    fn encode_fixed64(&mut self, n: u64);

    fn encode_sfixed32(&mut self, n: i32);
    fn encode_sfixed64(&mut self, n: i64);

    fn encode_bool(&mut self, b: bool);
    fn encode_bytes(&mut self, b: &[u8]);
    fn encode_string(&mut self, s: &str);

    fn encode_message(&mut self, msg: impl EncodeMessage);
}

#[inline]
pub fn zigzag_encode(from: i64) -> u64 {
    ((from << 1) ^ (from >> 63)) as u64
}

impl<T> Encode for T
where
    T: bytes::BufMut,
{
    fn encode_float(&mut self, n: f32) {
        self.put_f32_le(n)
    }

    fn encode_double(&mut self, n: f64) {
        self.put_f64_le(n)
    }

    fn encode_varint(&mut self, mut n: u64) {
        while n >= 0x80 {
            self.put_u8(0x80 | (n as u8));
            n >>= 7;
        }

        self.put_u8(n as u8);
    }

    fn encode_int32(&mut self, mut n: i32) {
        let negative = n < 0;
        let mut n = n as u32;

        if n <= 0x7F {
            return self.put_u8(n as u8);
        }

        loop {
            let mut b = (n as u8) & 0x7F;
            n >>= 7;
            if n != 0 {
                b |= 0x80;
            } else if negative {
                b |= 0b11110000;
            }
            self.put_u8(b);

            if n == 0 {
                break;
            }
        }

        if negative {
            self.put_slice(&[0xFF, 0xFF, 0xFF, 0xFF, 0x01]);
        }
    }

    fn encode_int64(&mut self, n: i64) {
        self.encode_varint(n as u64);
    }

    fn encode_uint32(&mut self, n: u32) {
        self.encode_varint(n as u64);
    }

    fn encode_uint64(&mut self, n: u64) {
        self.encode_varint(n);
    }

    fn encode_sint32(&mut self, n: i32) {
        self.encode_varint(zigzag_encode(n as i64));
    }

    fn encode_sint64(&mut self, n: i64) {
        self.encode_varint(zigzag_encode(n));
    }

    fn encode_fixed32(&mut self, n: u32) {
        self.put_u32_le(n);
    }

    fn encode_fixed64(&mut self, n: u64) {
        self.put_u64_le(n);
    }

    fn encode_sfixed32(&mut self, n: i32) {
        self.put_i32_le(n);
    }

    //162 => 81
    //174 => 87

    fn encode_sfixed64(&mut self, n: i64) {
        self.put_i64_le(n);
    }

    fn encode_bool(&mut self, b: bool) {
        self.put_u8(b as u8);
    }

    fn encode_bytes(&mut self, b: &[u8]) {
        self.encode_uint32(b.len() as u32);
        self.put_slice(b);
    }

    fn encode_string(&mut self, s: &str) {
        self.encode_bytes(s.as_bytes());
    }

    fn encode_message(&mut self, msg: impl EncodeMessage) {
        msg.encode(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{
        Fixed32, Fixed64, Int32, Int64, PbType, SFixed32, SFixed64, SInt32, SInt64, UInt32, UInt64,
    };

    use super::Encode;

    #[test]
    fn encode_float() {
        let mut buffer = bytes::BytesMut::with_capacity(4);
        buffer.encode_float(3.14);
        assert_eq!(3.14f32.size_hint(), 4);
        assert_eq!(*buffer, [0xc3, 0xf5, 0x48, 0x40]);
    }

    #[test]
    fn encode_double() {
        let mut buffer = bytes::BytesMut::with_capacity(8);
        buffer.encode_double(3.14);
        assert_eq!(3.14f64.size_hint(), 8);
        assert_eq!(*buffer, [0x1f, 0x85, 0xeb, 0x51, 0xb8, 0x1e, 0x09, 0x40]);
    }

    #[test]
    fn encode_int32() {
        let mut buffer = bytes::BytesMut::with_capacity(10);
        buffer.encode_int32(1);
        assert_eq!(Int32(1).size_hint(), 1);
        assert_eq!(*buffer, [0x01]);

        buffer.clear();
        buffer.encode_int32(1234);
        assert_eq!(Int32(1234).size_hint(), 2);
        assert_eq!(*buffer, [0xd2, 0x09]);

        buffer.clear();
        buffer.encode_int32(-1234);
        assert_eq!(Int32(-1234).size_hint(), 10);
        assert_eq!(
            *buffer,
            [0xae, 0xf6, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]
        );
    }

    #[test]
    fn encode_int64() {
        let mut buffer = bytes::BytesMut::with_capacity(10);
        buffer.encode_int64(1);
        assert_eq!(Int64(1).size_hint(), 1);
        assert_eq!(*buffer, [0x01]);

        buffer.clear();
        buffer.encode_int64(1234);
        assert_eq!(Int64(1234).size_hint(), 2);
        assert_eq!(*buffer, [0xd2, 0x09]);

        buffer.clear();
        buffer.encode_int64(-1234);
        assert_eq!(Int64(-1234).size_hint(), 10);
        assert_eq!(
            *buffer,
            [0xae, 0xf6, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01]
        );
    }

    #[test]
    fn encode_uint32() {
        let mut buffer = bytes::BytesMut::with_capacity(10);
        buffer.encode_uint32(1);
        assert_eq!(UInt32(1).size_hint(), 1);
        assert_eq!(*buffer, [0x01]);

        buffer.clear();
        buffer.encode_uint32(1234);
        assert_eq!(UInt32(1234).size_hint(), 2);
        assert_eq!(*buffer, [0xd2, 0x09]);
    }

    #[test]
    fn encode_uint64() {
        let mut buffer = bytes::BytesMut::with_capacity(10);
        buffer.encode_uint64(1);
        assert_eq!(UInt64(1).size_hint(), 1);
        assert_eq!(*buffer, [0x01]);

        buffer.clear();
        buffer.encode_uint64(1234);
        assert_eq!(UInt64(1234).size_hint(), 2);
        assert_eq!(*buffer, [0xd2, 0x09]);
    }

    #[test]
    fn encode_sint32() {
        let mut buffer = bytes::BytesMut::with_capacity(10);
        buffer.encode_sint32(1);
        assert_eq!(SInt32(1).size_hint(), 1);
        assert_eq!(*buffer, [0x02]);

        buffer.clear();
        buffer.encode_sint32(1234);
        assert_eq!(SInt32(1234).size_hint(), 2);
        assert_eq!(*buffer, [0xa4, 0x13]);

        buffer.clear();
        buffer.encode_sint32(-1234);
        assert_eq!(SInt32(-1234).size_hint(), 2);
        assert_eq!(*buffer, [0xa3, 0x13]);
    }

    #[test]
    fn encode_sint64() {
        let mut buffer = bytes::BytesMut::with_capacity(8);
        buffer.encode_sint64(1);
        assert_eq!(SInt64(1).size_hint(), 1);
        assert_eq!(*buffer, [0x02]);

        buffer.clear();
        buffer.encode_sint64(1234);
        assert_eq!(SInt64(1234).size_hint(), 2);
        assert_eq!(*buffer, [0xa4, 0x13]);

        buffer.clear();
        buffer.encode_sint64(-1234);
        assert_eq!(SInt64(-1234).size_hint(), 2);
        assert_eq!(*buffer, [0xa3, 0x13]);
    }

    #[test]
    fn encode_fixed32() {
        let mut buffer = bytes::BytesMut::with_capacity(4);
        buffer.encode_fixed32(1);
        assert_eq!(Fixed32(1).size_hint(), 4);
        assert_eq!(*buffer, [0x01, 0x00, 0x00, 0x00]);

        buffer.clear();
        buffer.encode_fixed32(1234);
        assert_eq!(Fixed32(1234).size_hint(), 4);
        assert_eq!(*buffer, [0xd2, 0x04, 0x00, 0x00]);
    }

    #[test]
    fn encode_fixed64() {
        let mut buffer = bytes::BytesMut::with_capacity(8);
        buffer.encode_fixed64(1);
        assert_eq!(Fixed64(1).size_hint(), 8);
        assert_eq!(*buffer, [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        buffer.clear();
        buffer.encode_fixed64(1234);
        assert_eq!(Fixed64(1234).size_hint(), 8);
        assert_eq!(*buffer, [0xd2, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn encode_sfixed32() {
        let mut buffer = bytes::BytesMut::with_capacity(4);
        buffer.encode_sfixed32(1);
        assert_eq!(SFixed32(1).size_hint(), 4);
        assert_eq!(*buffer, [0x01, 0x00, 0x00, 0x00]);

        buffer.clear();
        buffer.encode_sfixed32(1234);
        assert_eq!(SFixed32(1234).size_hint(), 4);
        assert_eq!(*buffer, [0xd2, 0x04, 0x00, 0x00]);

        buffer.clear();
        buffer.encode_sfixed32(-1234);
        assert_eq!(SFixed32(-1234).size_hint(), 4);
        assert_eq!(*buffer, [0x2e, 0xfb, 0xff, 0xff]);
    }

    #[test]
    fn encode_sfixed64() {
        let mut buffer = bytes::BytesMut::with_capacity(8);
        buffer.encode_sfixed64(1);
        assert_eq!(SFixed64(1).size_hint(), 8);
        assert_eq!(*buffer, [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        buffer.clear();
        buffer.encode_sfixed64(1234);
        assert_eq!(SFixed64(1234).size_hint(), 8);
        assert_eq!(*buffer, [0xd2, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        buffer.clear();
        buffer.encode_sfixed64(-1234);
        assert_eq!(SFixed64(-1234).size_hint(), 8);
        assert_eq!(*buffer, [0x2e, 0xfb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    }

    #[test]
    fn encode_bool() {
        let mut buffer = bytes::BytesMut::with_capacity(4);
        buffer.encode_bool(false);
        assert_eq!(false.size_hint(), 1);
        assert_eq!(*buffer, [0x00]);

        buffer.clear();
        buffer.encode_bool(true);
        assert_eq!(true.size_hint(), 1);
        assert_eq!(*buffer, [0x01]);
    }

    #[test]
    fn encode_string() {
        let mut buffer = bytes::BytesMut::with_capacity(8);
        let data = String::new();
        buffer.encode_string(&data);
        assert_eq!(PbType::size_hint(&data), 1);
        assert_eq!(*buffer, [0x00]);

        buffer.clear();
        let data = String::from("hello");
        buffer.encode_string(&data);
        assert_eq!(PbType::size_hint(&data), 6);
        assert_eq!(*buffer, [0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f]);
    }
}
