#![allow(clippy::cast_possible_truncation)]

use crate::{
    tag::Tag,
    types::{sizeof_varint32, sizeof_varint64, PbType},
};

/// main encode trait, currently implement for anything that implements [bytes::BufMut] and [SizeHint]
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
    fn encode_str(&mut self, s: &str);
    fn encode_string(&mut self, s: String) {
        self.encode_str(&s)
    }

    fn encode_nested(&mut self, msg: &impl PbType)
    where
        Self: Sized,
    {
        let size = msg.size_hint();

        self.encode_uint32(size as _);
        msg.encode(self);
    }

    fn encode_type(&mut self, msg: &impl PbType);

    // TODO: encode_fn and size_fn are actually the same, but generics need to be adjusted
    fn encode_packed<M, F, FS>(&mut self, items: &[M], mut encode_fn: F, mut size_fn: FS)
    where
        M: Clone,
        F: FnMut(&mut Self, M),
        FS: FnMut(&mut SizeHint, M),
    {
        let mut hint = SizeHint::default();
        for item in items {
            size_fn(&mut hint, item.clone())
        }
        self.encode_uint32(hint.size() as _);
        for item in items {
            encode_fn(self, item.clone());
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn encode_map_element<K, V, FK, FV, FSK, FSV>(
        &mut self,
        key: K,
        value: V,
        key_wire_type: u8,
        value_wire_type: u8,
        mut key_encode_fn: FK,
        mut value_encode_fn: FV,
        mut key_size_fn: FSK,
        mut value_size_fn: FSV,
    ) where
        K: Clone,
        FK: FnMut(&mut Self, K),
        FSK: FnMut(&mut SizeHint, K),
        V: Clone,
        FV: FnMut(&mut Self, V),
        FSV: FnMut(&mut SizeHint, V),
    {
        let mut hint = SizeHint::default();

        hint.encode_uint32(u32::from_parts(1, key_wire_type));
        key_size_fn(&mut hint, key.clone());
        hint.encode_uint32(u32::from_parts(2, value_wire_type));
        value_size_fn(&mut hint, value.clone());

        self.encode_uint32(hint.size() as _);
        self.encode_uint32(u32::from_parts(1, key_wire_type));
        key_encode_fn(self, key.clone());
        self.encode_uint32(u32::from_parts(2, value_wire_type));
        value_encode_fn(self, value.clone());
    }
}

#[inline]
fn zigzag_encode(from: i64) -> u64 {
    ((from << 1) ^ (from >> 63)) as u64
}

impl<T> Encode for T
where
    T: bytes::BufMut,
{
    #[inline]
    fn encode_float(&mut self, n: f32) {
        self.put_f32_le(n)
    }

    #[inline]
    fn encode_double(&mut self, n: f64) {
        self.put_f64_le(n)
    }

    #[inline]
    fn encode_varint(&mut self, mut n: u64) {
        while n >= 0x80 {
            self.put_u8(0x80 | (n as u8));
            n >>= 7;
        }

        self.put_u8(n as u8);
    }

    #[inline]
    fn encode_int32(&mut self, n: i32) {
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

    #[inline]
    fn encode_int64(&mut self, n: i64) {
        self.encode_varint(n as u64);
    }

    #[inline]
    fn encode_uint32(&mut self, n: u32) {
        self.encode_varint(n as u64);
    }

    #[inline]
    fn encode_uint64(&mut self, n: u64) {
        self.encode_varint(n);
    }

    #[inline]
    fn encode_sint32(&mut self, n: i32) {
        self.encode_varint(zigzag_encode(n as i64));
    }

    #[inline]
    fn encode_sint64(&mut self, n: i64) {
        self.encode_varint(zigzag_encode(n));
    }

    #[inline]
    fn encode_fixed32(&mut self, n: u32) {
        self.put_u32_le(n);
    }

    #[inline]
    fn encode_fixed64(&mut self, n: u64) {
        self.put_u64_le(n);
    }

    #[inline]
    fn encode_sfixed32(&mut self, n: i32) {
        self.put_i32_le(n);
    }

    #[inline]
    fn encode_sfixed64(&mut self, n: i64) {
        self.put_i64_le(n);
    }

    #[inline]
    fn encode_bool(&mut self, b: bool) {
        self.put_u8(b as u8);
    }

    #[inline]
    fn encode_bytes(&mut self, b: &[u8]) {
        self.encode_uint32(b.len() as u32);
        self.put_slice(b);
    }

    #[inline]
    fn encode_str(&mut self, s: &str) {
        self.encode_bytes(s.as_bytes());
    }

    #[inline]
    fn encode_type(&mut self, ty: &impl PbType) {
        ty.encode(self)
    }
}

#[derive(Debug, Default)]
/// an [Encode] implementation that does not actually encode data but calculates the size it will need
pub struct SizeHint {
    size: usize,
}

impl SizeHint {
    pub fn clear(&mut self) {
        self.size = 0;
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Encode for SizeHint {
    #[inline]
    fn encode_float(&mut self, _n: f32) {
        self.size += std::mem::size_of::<f32>();
    }

    #[inline]
    fn encode_double(&mut self, _n: f64) {
        self.size += std::mem::size_of::<f64>();
    }

    #[inline]
    fn encode_varint(&mut self, n: u64) {
        self.size += sizeof_varint64(n);
    }

    #[inline]
    fn encode_int32(&mut self, n: i32) {
        if n < 0 {
            self.size += 10;
        } else {
            self.encode_varint(n as _);
        }
    }

    #[inline]
    fn encode_int64(&mut self, n: i64) {
        self.encode_varint(n as _);
    }

    #[inline]
    fn encode_uint32(&mut self, n: u32) {
        self.size += sizeof_varint32(n);
    }

    #[inline]
    fn encode_uint64(&mut self, n: u64) {
        self.encode_varint(n as _);
    }

    #[inline]
    fn encode_sint32(&mut self, n: i32) {
        self.size += sizeof_varint32(zigzag_encode(n as i64) as u32);
    }

    #[inline]
    fn encode_sint64(&mut self, n: i64) {
        self.encode_varint(zigzag_encode(n));
    }

    #[inline]
    fn encode_fixed32(&mut self, _n: u32) {
        self.size += std::mem::size_of::<u32>();
    }

    #[inline]
    fn encode_fixed64(&mut self, _n: u64) {
        self.size += std::mem::size_of::<u64>();
    }

    #[inline]
    fn encode_sfixed32(&mut self, _n: i32) {
        self.size += std::mem::size_of::<i32>();
    }

    #[inline]
    fn encode_sfixed64(&mut self, _n: i64) {
        self.size += std::mem::size_of::<i64>();
    }

    #[inline]
    fn encode_bool(&mut self, _b: bool) {
        self.size += 1;
    }

    #[inline]
    fn encode_bytes(&mut self, b: &[u8]) {
        self.encode_uint32(b.len() as u32);
        self.size += b.len();
    }

    #[inline]
    fn encode_str(&mut self, s: &str) {
        self.encode_bytes(s.as_bytes());
    }

    #[inline]
    fn encode_type(&mut self, msg: &impl PbType) {
        msg.encode(self)
    }
}

#[cfg(test)]
mod tests {
    // not using the constants as testing against protobuf pal would get difficult then
    #![allow(clippy::approx_constant)]

    use crate::{
        encoder::SizeHint,
        types::{
            Fixed32, Fixed64, Int32, Int64, PbType, SFixed32, SFixed64, SInt32, SInt64, UInt32,
            UInt64,
        },
        WIRE_TYPE_LENGTH_ENCODED, WIRE_TYPE_VARINT,
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
        buffer.encode_str(&data);
        assert_eq!(PbType::size_hint(&data), 1);
        assert_eq!(*buffer, [0x00]);

        buffer.clear();
        let data = String::from("hello");
        buffer.encode_str(&data);
        assert_eq!(PbType::size_hint(&data), 6);
        assert_eq!(*buffer, [0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f]);
    }

    #[test]
    fn encode_packed() {
        let mut buffer = bytes::BytesMut::with_capacity(8);
        let mut hint = SizeHint::default();

        let data = vec![1, 2, 3];
        buffer.encode_packed(&data, Encode::encode_uint32, Encode::encode_uint32);
        hint.encode_packed(&data, Encode::encode_uint32, Encode::encode_uint32);

        assert_eq!(hint.size(), 4);
        assert_eq!(*buffer, [0x03, 0x01, 0x02, 0x03]);

        buffer.clear();
        hint.clear();
        let data = vec![1234, 2345, 3456];
        buffer.encode_packed(&data, Encode::encode_uint32, Encode::encode_uint32);
        hint.encode_packed(&data, Encode::encode_uint32, Encode::encode_uint32);

        assert_eq!(hint.size(), 7);
        assert_eq!(*buffer, [0x06, 0xd2, 0x09, 0xa9, 0x12, 0x80, 0x1b]);
    }

    #[test]
    fn encode_map_element() {
        let mut buffer = bytes::BytesMut::with_capacity(8);
        let mut hint = SizeHint::default();

        let (key, value) = ("one", 1u32);

        hint.encode_map_element(
            key,
            value,
            WIRE_TYPE_LENGTH_ENCODED,
            WIRE_TYPE_VARINT,
            Encode::encode_str,
            Encode::encode_uint32,
            Encode::encode_str,
            Encode::encode_uint32,
        );
        buffer.encode_map_element(
            key,
            value,
            WIRE_TYPE_LENGTH_ENCODED,
            WIRE_TYPE_VARINT,
            Encode::encode_str,
            Encode::encode_uint32,
            Encode::encode_str,
            Encode::encode_uint32,
        );

        assert_eq!(hint.size(), 8);
        assert_eq!(*buffer, [0x07, 0x0a, 0x03, 0x6f, 0x6e, 0x65, 0x10, 0x01]);
    }
}
