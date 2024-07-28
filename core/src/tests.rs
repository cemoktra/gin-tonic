use base64::prelude::*;
use bytes::{Bytes, BytesMut};

use crate::{
    decode_field,
    decoder::{Decode, DecodeError},
    encode_field,
    encoder::Encode,
    size_hint, size_hint_wrapped,
    tag::Tag,
    types::{
        sizeof_varint64, Fixed32, Fixed64, Int32, Int64, PbType, SFixed32, SFixed64, SInt32,
        SInt64, UInt32, UInt64,
    },
    WIRE_TYPE_LENGTH_ENCODED, WIRE_TYPE_VARINT,
};

#[derive(Debug, PartialEq)]
enum TestEnum {
    A,
    B,
}

impl PbType for TestEnum {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn size_hint(&self) -> usize {
        match self {
            TestEnum::A => sizeof_varint64(1),
            TestEnum::B => sizeof_varint64(2),
        }
    }

    fn encode(self, encoder: &mut impl Encode) {
        match self {
            TestEnum::A => encoder.encode_uint64(1),
            TestEnum::B => encoder.encode_uint64(2),
        }
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match decoder.decode_uint64()? {
            1 => Ok(Self::A),
            2 => Ok(Self::B),
            _ => todo!("enum variant error"),
        }
    }
}

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
    e: TestEnum,
}

impl PbType for Test {
    fn size_hint(&self) -> usize {
        size_hint!(1, f32, self.float)
            + size_hint!(2, f64, self.double)
            + size_hint_wrapped!(3, Int32, Int32, self.pos_i32)
            + size_hint_wrapped!(4, Int32, Int32, self.neg_i32)
            + size_hint_wrapped!(5, Int64, Int64, self.pos_i64)
            + size_hint_wrapped!(6, Int64, Int64, self.neg_i64)
            + size_hint_wrapped!(7, UInt32, UInt32, self.uint32)
            + size_hint_wrapped!(8, UInt64, UInt64, self.uint64)
            + size_hint_wrapped!(9, SInt32, SInt32, self.pos_sint32)
            + size_hint_wrapped!(10, SInt32, SInt32, self.neg_sint32)
            + size_hint_wrapped!(11, SInt64, SInt64, self.pos_sint64)
            + size_hint_wrapped!(12, SInt64, SInt64, self.neg_sint64)
            + size_hint_wrapped!(13, Fixed32, Fixed32, self.fixed_u32)
            + size_hint_wrapped!(14, Fixed64, Fixed64, self.fixed_u64)
            + size_hint_wrapped!(15, SFixed32, SFixed32, self.pos_sfixed32)
            + size_hint_wrapped!(16, SFixed32, SFixed32, self.neg_sfixed32)
            + size_hint_wrapped!(17, SFixed64, SFixed64, self.pos_sfixed64)
            + size_hint_wrapped!(18, SFixed64, SFixed64, self.neg_sfixed64)
            + size_hint!(19, bool, self.b)
            + size_hint!(20, String, self.empty)
            + size_hint!(21, String, self.hello)
            + size_hint!(22, TestEnum, self.e)
    }

    fn encode(self, encoder: &mut impl crate::encoder::Encode) {
        encode_field!(1, f32, self.float, encoder, Encode::encode_float);
        encode_field!(2, f64, self.double, encoder, Encode::encode_double);
        encode_field!(3, Int32, self.pos_i32, encoder, Encode::encode_int32);
        encode_field!(4, Int32, self.neg_i32, encoder, Encode::encode_int32);
        encode_field!(5, Int64, self.pos_i64, encoder, Encode::encode_int64);
        encode_field!(6, Int64, self.neg_i64, encoder, Encode::encode_int64);
        encode_field!(7, UInt32, self.uint32, encoder, Encode::encode_uint32);
        encode_field!(8, UInt64, self.uint64, encoder, Encode::encode_uint64);
        encode_field!(9, SInt32, self.pos_sint32, encoder, Encode::encode_sint32);
        encode_field!(10, SInt32, self.neg_sint32, encoder, Encode::encode_sint32);
        encode_field!(11, SInt64, self.pos_sint64, encoder, Encode::encode_sint64);
        encode_field!(12, SInt64, self.neg_sint64, encoder, Encode::encode_sint64);
        encode_field!(13, Fixed32, self.fixed_u32, encoder, Encode::encode_fixed32);
        encode_field!(14, Fixed64, self.fixed_u64, encoder, Encode::encode_fixed64);
        encode_field!(
            15,
            SFixed32,
            self.pos_sfixed32,
            encoder,
            Encode::encode_sfixed32
        );
        encode_field!(
            16,
            SFixed32,
            self.neg_sfixed32,
            encoder,
            Encode::encode_sfixed32
        );
        encode_field!(
            17,
            SFixed64,
            self.pos_sfixed64,
            encoder,
            Encode::encode_sfixed64
        );
        encode_field!(
            18,
            SFixed64,
            self.neg_sfixed64,
            encoder,
            Encode::encode_sfixed64
        );
        encode_field!(19, bool, self.b, encoder, Encode::encode_bool);
        encode_field!(20, String, &self.empty, encoder, Encode::encode_string);
        encode_field!(21, String, &self.hello, encoder, Encode::encode_string);
        encode_field!(22, TestEnum, self.e, encoder, Encode::encode_type);
    }

    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
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
        let mut e = None;

        while !decoder.eof() {
            let tag = decoder.decode_uint32()?;
            let field_number = tag.field_number();
            let wire_type = tag.wire_type();

            match field_number {
                1 => decode_field!(f32, float, wire_type, decoder, Decode::decode_float),
                2 => decode_field!(f64, double, wire_type, decoder, Decode::decode_double),
                3 => decode_field!(Int32, pos_i32, wire_type, decoder, Decode::decode_int32),
                4 => decode_field!(Int32, neg_i32, wire_type, decoder, Decode::decode_int32),
                5 => decode_field!(Int64, pos_i64, wire_type, decoder, Decode::decode_int64),
                6 => decode_field!(Int64, neg_i64, wire_type, decoder, Decode::decode_int64),
                7 => decode_field!(UInt32, uint32, wire_type, decoder, Decode::decode_uint32),
                8 => decode_field!(UInt64, uint64, wire_type, decoder, Decode::decode_uint64),
                9 => decode_field!(
                    SInt32,
                    pos_sint32,
                    wire_type,
                    decoder,
                    Decode::decode_sint32
                ),
                10 => decode_field!(
                    SInt32,
                    neg_sint32,
                    wire_type,
                    decoder,
                    Decode::decode_sint32
                ),
                11 => decode_field!(
                    SInt64,
                    pos_sint64,
                    wire_type,
                    decoder,
                    Decode::decode_sint64
                ),
                12 => decode_field!(
                    SInt64,
                    neg_sint64,
                    wire_type,
                    decoder,
                    Decode::decode_sint64
                ),
                13 => decode_field!(
                    Fixed32,
                    fixed_u32,
                    wire_type,
                    decoder,
                    Decode::decode_fixed32
                ),
                14 => decode_field!(
                    Fixed64,
                    fixed_u64,
                    wire_type,
                    decoder,
                    Decode::decode_fixed64
                ),
                15 => decode_field!(
                    SFixed32,
                    pos_sfixed32,
                    wire_type,
                    decoder,
                    Decode::decode_sfixed32
                ),
                16 => decode_field!(
                    SFixed32,
                    neg_sfixed32,
                    wire_type,
                    decoder,
                    Decode::decode_sfixed32
                ),
                17 => decode_field!(
                    SFixed64,
                    pos_sfixed64,
                    wire_type,
                    decoder,
                    Decode::decode_sfixed64
                ),
                18 => decode_field!(
                    SFixed64,
                    neg_sfixed64,
                    wire_type,
                    decoder,
                    Decode::decode_sfixed64
                ),
                19 => decode_field!(bool, b, wire_type, decoder, Decode::decode_bool),
                20 => decode_field!(String, empty, wire_type, decoder, Decode::decode_string),
                21 => decode_field!(String, hello, wire_type, decoder, Decode::decode_string),
                22 => decode_field!(TestEnum, e, wire_type, decoder, TestEnum::decode),
                n => return Err(DecodeError::UnexpectedFieldNumber(n)),
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
            e: e.unwrap(),
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
        e: TestEnum::A,
    };

    let size = test.size_hint();
    assert_eq!(124, size);

    let mut buffer = BytesMut::with_capacity(size);
    test.encode(&mut buffer);

    assert_eq!(
        "DcP1SEARH4XrUbgeCUAYuWAgx5//////////ASi5YDDHn/////////8BOAFAAUjywAFQ8cABWPLAAWDxwAFtewAAAHF7AAAAAAAAAH05MAAAhQHHz///iQE5MAAAAAAAAJEBx8////////+YAQCiAQCqAQV3b3JsZLABAQ==",
        BASE64_STANDARD.encode(&buffer)
    );
}

#[test]
fn decode() {
    let data = BASE64_STANDARD
        .decode(
            "DcP1SEARH4XrUbgeCUAYuWAgx5//////////ASi5YDDHn/////////8BOAFAAUjywAFQ8cABWPLAAWDxwAFtewAAAHF7AAAAAAAAAH05MAAAhQHHz///iQE5MAAAAAAAAJEBx8////////+YAQCiAQCqAQV3b3JsZLABAQ==",
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
    assert_eq!(test.e, TestEnum::A);
}
