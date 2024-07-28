use base64::prelude::*;
use bytes::{Bytes, BytesMut};

use crate::{
    message::{DecodeMessage, EncodeMessage},
    tag::Tag,
    types::{
        Fixed32, Fixed64, Int32, Int64, PbType, SFixed32, SFixed64, SInt32, SInt64, UInt32, UInt64,
    },
};

#[derive(Debug)]
struct Test {
    float: f32,
    double: f64,
    pos_i32: i32,
    neg_i32: i32,
    pos_i64: i64,
    neg_i64: i64,
    uint32: u32,
    uint64: u64,
    pos_sint32: i32,
    neg_sint32: i32,
    pos_sint64: i64,
    neg_sint64: i64,
    fixed_u32: u32,
    fixed_u64: u64,
    pos_sfixed32: i32,
    neg_sfixed32: i32,
    pos_sfixed64: i64,
    neg_sfixed64: i64,
    b: bool,
    empty: String,
    hello: String,
}

impl EncodeMessage for Test {
    fn size_hint(&self) -> usize {
        UInt32(u32::from_parts(1, f32::WIRE_TYPE)).size_hint()
            + self.float.size_hint()
            + UInt32(u32::from_parts(2, f64::WIRE_TYPE)).size_hint()
            + self.double.size_hint()
            + UInt32(u32::from_parts(3, Int32::WIRE_TYPE)).size_hint()
            + Int32(self.pos_i32).size_hint()
            + UInt32(u32::from_parts(4, Int32::WIRE_TYPE)).size_hint()
            + Int32(self.neg_i32).size_hint()
            + UInt32(u32::from_parts(5, Int64::WIRE_TYPE)).size_hint()
            + Int64(self.pos_i64).size_hint()
            + UInt32(u32::from_parts(6, Int64::WIRE_TYPE)).size_hint()
            + Int64(self.neg_i64).size_hint()
            + UInt32(u32::from_parts(7, UInt32::WIRE_TYPE)).size_hint()
            + UInt32(self.uint32).size_hint()
            + UInt32(u32::from_parts(8, UInt64::WIRE_TYPE)).size_hint()
            + UInt64(self.uint64).size_hint()
            + UInt32(u32::from_parts(9, SInt32::WIRE_TYPE)).size_hint()
            + SInt32(self.pos_sint32).size_hint()
            + UInt32(u32::from_parts(10, SInt32::WIRE_TYPE)).size_hint()
            + SInt32(self.neg_sint32).size_hint()
            + UInt32(u32::from_parts(11, SInt64::WIRE_TYPE)).size_hint()
            + SInt64(self.pos_sint64).size_hint()
            + UInt32(u32::from_parts(12, SInt64::WIRE_TYPE)).size_hint()
            + SInt64(self.neg_sint64).size_hint()
            + UInt32(u32::from_parts(13, Fixed32::WIRE_TYPE)).size_hint()
            + Fixed32(self.fixed_u32).size_hint()
            + UInt32(u32::from_parts(14, Fixed64::WIRE_TYPE)).size_hint()
            + Fixed64(self.fixed_u64).size_hint()
            + UInt32(u32::from_parts(15, SFixed32::WIRE_TYPE)).size_hint()
            + SFixed32(self.pos_sfixed32).size_hint()
            + UInt32(u32::from_parts(16, SFixed32::WIRE_TYPE)).size_hint()
            + SFixed32(self.neg_sfixed32).size_hint()
            + UInt32(u32::from_parts(17, SFixed64::WIRE_TYPE)).size_hint()
            + SFixed64(self.pos_sfixed64).size_hint()
            + UInt32(u32::from_parts(18, SFixed64::WIRE_TYPE)).size_hint()
            + SFixed64(self.neg_sfixed64).size_hint()
            + UInt32(u32::from_parts(19, bool::WIRE_TYPE)).size_hint()
            + self.b.size_hint()
            + UInt32(u32::from_parts(20, String::WIRE_TYPE)).size_hint()
            + self.empty.size_hint()
            + UInt32(u32::from_parts(21, String::WIRE_TYPE)).size_hint()
            + self.hello.size_hint()
    }

    fn encode(&self, encoder: &mut impl crate::encoder::Encode) {
        encoder.encode_uint32(u32::from_parts(1, f32::WIRE_TYPE));
        encoder.encode_float(self.float);

        encoder.encode_uint32(u32::from_parts(2, f64::WIRE_TYPE));
        encoder.encode_double(self.double);

        encoder.encode_uint32(u32::from_parts(3, Int32::WIRE_TYPE));
        encoder.encode_int32(self.pos_i32);

        encoder.encode_uint32(u32::from_parts(4, Int32::WIRE_TYPE));
        encoder.encode_int32(self.neg_i32);

        encoder.encode_uint32(u32::from_parts(5, Int64::WIRE_TYPE));
        encoder.encode_int64(self.pos_i64);

        encoder.encode_uint32(u32::from_parts(6, Int64::WIRE_TYPE));
        encoder.encode_int64(self.neg_i64);

        encoder.encode_uint32(u32::from_parts(7, UInt32::WIRE_TYPE));
        encoder.encode_uint32(self.uint32);

        encoder.encode_uint32(u32::from_parts(8, UInt64::WIRE_TYPE));
        encoder.encode_uint64(self.uint64);

        encoder.encode_uint32(u32::from_parts(9, SInt32::WIRE_TYPE));
        encoder.encode_sint32(self.pos_sint32);

        encoder.encode_uint32(u32::from_parts(10, SInt32::WIRE_TYPE));
        encoder.encode_sint32(self.neg_sint32);

        encoder.encode_uint32(u32::from_parts(11, SInt64::WIRE_TYPE));
        encoder.encode_sint64(self.pos_sint64);

        encoder.encode_uint32(u32::from_parts(12, SInt64::WIRE_TYPE));
        encoder.encode_sint64(self.neg_sint64);

        encoder.encode_uint32(u32::from_parts(13, Fixed32::WIRE_TYPE));
        encoder.encode_fixed32(self.fixed_u32);

        encoder.encode_uint32(u32::from_parts(14, Fixed64::WIRE_TYPE));
        encoder.encode_fixed64(self.fixed_u64);

        encoder.encode_uint32(u32::from_parts(15, SFixed32::WIRE_TYPE));
        encoder.encode_sfixed32(self.pos_sfixed32);

        encoder.encode_uint32(u32::from_parts(16, SFixed32::WIRE_TYPE));
        encoder.encode_sfixed32(self.neg_sfixed32);

        encoder.encode_uint32(u32::from_parts(17, SFixed64::WIRE_TYPE));
        encoder.encode_sfixed64(self.pos_sfixed64);

        encoder.encode_uint32(u32::from_parts(18, SFixed64::WIRE_TYPE));
        encoder.encode_sfixed64(self.neg_sfixed64);

        encoder.encode_uint32(u32::from_parts(19, bool::WIRE_TYPE));
        encoder.encode_bool(self.b);

        encoder.encode_uint32(u32::from_parts(20, String::WIRE_TYPE));
        encoder.encode_string(&self.empty);

        encoder.encode_uint32(u32::from_parts(21, String::WIRE_TYPE));
        encoder.encode_string(&self.hello);
    }
}

impl DecodeMessage for Test {
    fn decode(
        decoder: &mut impl crate::decoder::Decode,
    ) -> Result<Self, crate::decoder::DecodeError>
    where
        Self: Sized,
    {
        let mut float = None;
        let mut double = None;
        let mut pos_i32 = None;
        let mut neg_i32 = None;
        let mut pos_i64 = None;
        let mut neg_i64 = None;
        let mut uint32 = None;
        let mut uint64 = None;
        let mut pos_sint32 = None;
        let mut neg_sint32 = None;
        let mut pos_sint64 = None;
        let mut neg_sint64 = None;
        let mut fixed_u32 = None;
        let mut fixed_u64 = None;
        let mut pos_sfixed32 = None;
        let mut neg_sfixed32 = None;
        let mut pos_sfixed64 = None;
        let mut neg_sfixed64 = None;
        let mut b = None;
        let mut empty = None;
        let mut hello = None;

        while !decoder.eof() {
            let tag = decoder.decode_uint32()?;
            let field_number = tag.field_number();
            let wire_type = tag.wire_type();

            println!("tag = {tag}, field_number = {field_number}, wire_type = {wire_type}",);

            match field_number {
                1 => {
                    assert_eq!(5, wire_type);
                    float = Some(decoder.decode_float()?);
                }
                2 => {
                    assert_eq!(1, wire_type);
                    double = Some(decoder.decode_double()?);
                }
                3 => {
                    assert_eq!(0, wire_type);
                    pos_i32 = Some(decoder.decode_int32()?);
                }
                4 => {
                    assert_eq!(0, wire_type);
                    neg_i32 = Some(decoder.decode_int32()?);
                }
                5 => {
                    assert_eq!(0, wire_type);
                    pos_i64 = Some(decoder.decode_int64()?);
                }
                6 => {
                    assert_eq!(0, wire_type);
                    neg_i64 = Some(decoder.decode_int64()?);
                }
                7 => {
                    assert_eq!(0, wire_type);
                    uint32 = Some(decoder.decode_uint32()?);
                }
                8 => {
                    assert_eq!(0, wire_type);
                    uint64 = Some(decoder.decode_uint64()?);
                }
                9 => {
                    assert_eq!(0, wire_type);
                    pos_sint32 = Some(decoder.decode_sint32()?);
                }
                10 => {
                    assert_eq!(0, wire_type);
                    neg_sint32 = Some(decoder.decode_sint32()?);
                }
                11 => {
                    assert_eq!(0, wire_type);
                    pos_sint64 = Some(decoder.decode_sint64()?);
                }
                12 => {
                    assert_eq!(0, wire_type);
                    neg_sint64 = Some(decoder.decode_sint64()?);
                }
                13 => {
                    assert_eq!(5, wire_type);
                    fixed_u32 = Some(decoder.decode_fixed32()?);
                }
                14 => {
                    assert_eq!(1, wire_type);
                    fixed_u64 = Some(decoder.decode_fixed64()?);
                }
                15 => {
                    assert_eq!(5, wire_type);
                    pos_sfixed32 = Some(decoder.decode_sfixed32()?);
                }
                16 => {
                    assert_eq!(5, wire_type);
                    neg_sfixed32 = Some(decoder.decode_sfixed32()?);
                }
                17 => {
                    assert_eq!(1, wire_type);
                    pos_sfixed64 = Some(decoder.decode_sfixed64()?);
                }
                18 => {
                    assert_eq!(1, wire_type);
                    neg_sfixed64 = Some(decoder.decode_sfixed64()?);
                }
                19 => {
                    assert_eq!(0, wire_type);
                    b = Some(decoder.decode_bool()?);
                }
                20 => {
                    assert_eq!(2, wire_type);
                    empty = Some(decoder.decode_string()?);
                }
                21 => {
                    assert_eq!(2, wire_type);
                    hello = Some(decoder.decode_string()?);
                }

                _ => panic!("unexpect field number: {field_number}"),
            }
        }

        Ok(Self {
            float: float.unwrap(),
            double: double.unwrap(),
            pos_i32: pos_i32.unwrap(),
            neg_i32: neg_i32.unwrap(),
            pos_i64: pos_i64.unwrap(),
            neg_i64: neg_i64.unwrap(),
            uint32: uint32.unwrap(),
            uint64: uint64.unwrap(),
            pos_sint32: pos_sint32.unwrap(),
            neg_sint32: neg_sint32.unwrap(),
            pos_sint64: pos_sint64.unwrap(),
            neg_sint64: neg_sint64.unwrap(),
            fixed_u32: fixed_u32.unwrap(),
            fixed_u64: fixed_u64.unwrap(),
            pos_sfixed32: pos_sfixed32.unwrap(),
            neg_sfixed32: neg_sfixed32.unwrap(),
            pos_sfixed64: pos_sfixed64.unwrap(),
            neg_sfixed64: neg_sfixed64.unwrap(),
            b: b.unwrap(),
            empty: empty.unwrap(),
            hello: hello.unwrap(),
        })
    }
}

#[test]
fn encode() {
    let test = Test {
        float: 3.14,
        double: 3.14,
        pos_i32: 12345,
        neg_i32: -12345,
        pos_i64: 12345,
        neg_i64: -12345,
        uint32: 1,
        uint64: 1,
        pos_sint32: 12345,
        neg_sint32: -12345,
        pos_sint64: 12345,
        neg_sint64: -12345,
        fixed_u32: 123,
        fixed_u64: 123,
        pos_sfixed32: 12345,
        neg_sfixed32: -12345,
        pos_sfixed64: 12345,
        neg_sfixed64: -12345,
        b: false,
        empty: String::new(),
        hello: String::from("world"),
    };

    let size = test.size_hint();
    assert_eq!(121, size);

    let mut buffer = BytesMut::with_capacity(121);
    test.encode(&mut buffer);

    assert_eq!(
        "DcP1SEARH4XrUbgeCUAYuWAgx5//////////ASi5YDDHn/////////8BOAFAAUjywAFQ8cABWPLAAWDxwAFtewAAAHF7AAAAAAAAAH05MAAAhQHHz///iQE5MAAAAAAAAJEBx8////////+YAQCiAQCqAQV3b3JsZA==",
        BASE64_STANDARD.encode(&buffer)
    );
}

#[test]
fn decode() {
    let data = BASE64_STANDARD
        .decode(
            "DcP1SEARH4XrUbgeCUAYuWAgx5//////////ASi5YDDHn/////////8BOAFAAUjywAFQ8cABWPLAAWDxwAFtewAAAHF7AAAAAAAAAH05MAAAhQHHz///iQE5MAAAAAAAAJEBx8////////+YAQCiAQCqAQV3b3JsZA==",
        )
        .unwrap();
    let mut bytes = bytes::Bytes::from(data);

    let test = Test::decode(&mut bytes).unwrap();

    assert_eq!(test.float, 3.14f32);
    assert_eq!(test.double, 3.14f64);
    assert_eq!(test.pos_i32, 12345);
    assert_eq!(test.pos_i64, 12345);
    assert_eq!(test.neg_i32, -12345);
    assert_eq!(test.neg_i64, -12345);
    assert_eq!(test.uint32, 1);
    assert_eq!(test.uint64, 1);
    assert_eq!(test.pos_sint32, 12345);
    assert_eq!(test.pos_sint64, 12345);
    assert_eq!(test.neg_sint32, -12345);
    assert_eq!(test.neg_sint64, -12345);
    assert_eq!(test.fixed_u32, 123);
    assert_eq!(test.fixed_u64, 123);
    assert_eq!(test.pos_sfixed32, 12345);
    assert_eq!(test.pos_sfixed64, 12345);
    assert_eq!(test.neg_sfixed32, -12345);
    assert_eq!(test.neg_sfixed64, -12345);
    assert!(!test.b);
    assert!(test.empty.is_empty());
    assert_eq!(test.hello, "world");
}
