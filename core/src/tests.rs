use std::collections::HashMap;

use base64::prelude::*;
use bytes::BytesMut;

use crate::{
    decode_field, decode_map, decode_nested, decode_vector,
    decoder::{Decode, DecodeError},
    encode_field, encode_map, encode_nested, encode_vector_packed, encode_vector_unpacked,
    encoder::Encode,
    tag::Tag,
    types::{
        Fixed32, Fixed64, Int32, Int64, PbType, SFixed32, SFixed64, SInt32, SInt64, UInt32, UInt64,
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

    fn encode(&self, encoder: &mut impl Encode) {
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

#[derive(Debug, PartialEq)]
enum TestOneOf {
    Int(i32),
    Str(String),
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
    o: TestOneOf,
    packed: Vec<u32>,
    unpacked: Vec<u32>,
    nested: Nested,
}

impl PbType for Test {
    fn encode(&self, encoder: &mut impl crate::encoder::Encode) {
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
        encode_field!(20, String, &self.empty, encoder, Encode::encode_str);
        encode_field!(21, String, &self.hello, encoder, Encode::encode_str);
        encode_field!(22, TestEnum, &self.e, encoder, Encode::encode_type);
        match &self.o {
            TestOneOf::Int(i) => {
                encode_field!(23, Int32, *i, encoder, Encode::encode_int32);
            }
            TestOneOf::Str(s) => {
                encode_field!(24, String, s, encoder, Encode::encode_str);
            }
        };
        encode_vector_packed!(25, &self.packed, encoder, Encode::encode_uint32);
        encode_vector_unpacked!(26, UInt32, &self.unpacked, encoder, Encode::encode_uint32);
        encode_nested!(27, &self.nested, encoder, Encode::encode_nested);
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
        let mut o = None;
        let mut packed = vec![];
        let mut unpacked = vec![];
        let mut nested = None;

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
                23 => {
                    #[allow(unused_assignments)]
                    let mut one_of_inner = None;
                    decode_field!(
                        Int32,
                        one_of_inner,
                        wire_type,
                        decoder,
                        Decode::decode_int32
                    );
                    o = Some(TestOneOf::Int(
                        one_of_inner.ok_or(DecodeError::MissingField(23))?,
                    ))
                }
                24 => {
                    #[allow(unused_assignments)]
                    let mut one_of_inner = None;
                    decode_field!(
                        String,
                        one_of_inner,
                        wire_type,
                        decoder,
                        Decode::decode_string
                    );
                    o = Some(TestOneOf::Str(
                        one_of_inner.ok_or(DecodeError::MissingField(24))?,
                    ))
                }
                25 => {
                    decode_vector!(
                        UInt32,
                        &mut packed,
                        wire_type,
                        decoder,
                        Decode::decode_uint32
                    );
                }
                26 => {
                    decode_vector!(
                        UInt32,
                        &mut unpacked,
                        wire_type,
                        decoder,
                        Decode::decode_uint32
                    );
                }
                27 => {
                    decode_nested!(nested, wire_type, decoder, Decode::decode_nested);
                }
                n => return Err(DecodeError::UnexpectedFieldNumber(n)),
            }
        }

        Ok(Self {
            float: float.ok_or(DecodeError::MissingField(1))?,
            double: double.ok_or(DecodeError::MissingField(2))?,
            pos_i32: pos_i32.ok_or(DecodeError::MissingField(3))?,
            neg_i32: neg_i32.ok_or(DecodeError::MissingField(4))?,
            pos_i64: pos_i64.ok_or(DecodeError::MissingField(5))?,
            neg_i64: neg_i64.ok_or(DecodeError::MissingField(6))?,
            uint32: uint32.ok_or(DecodeError::MissingField(7))?,
            uint64: uint64.ok_or(DecodeError::MissingField(8))?,
            pos_sint32: pos_sint32.ok_or(DecodeError::MissingField(9))?,
            neg_sint32: neg_sint32.ok_or(DecodeError::MissingField(10))?,
            pos_sint64: pos_sint64.ok_or(DecodeError::MissingField(11))?,
            neg_sint64: neg_sint64.ok_or(DecodeError::MissingField(12))?,
            fixed_u32: fixed_u32.ok_or(DecodeError::MissingField(13))?,
            fixed_u64: fixed_u64.ok_or(DecodeError::MissingField(14))?,
            pos_sfixed32: pos_sfixed32.ok_or(DecodeError::MissingField(15))?,
            neg_sfixed32: neg_sfixed32.ok_or(DecodeError::MissingField(16))?,
            pos_sfixed64: pos_sfixed64.ok_or(DecodeError::MissingField(17))?,
            neg_sfixed64: neg_sfixed64.ok_or(DecodeError::MissingField(18))?,
            b: b.ok_or(DecodeError::MissingField(19))?,
            empty: empty.ok_or(DecodeError::MissingField(20))?,
            hello: hello.ok_or(DecodeError::MissingField(21))?,
            e: e.ok_or(DecodeError::MissingField(22))?,
            o: o.ok_or(DecodeError::MissingOneOf(vec![23, 24]))?,
            packed,
            unpacked,
            nested: nested.ok_or(DecodeError::MissingField(27))?,
        })
    }
}

#[derive(Debug)]
struct MapTest {
    pub map: HashMap<String, bool>,
}

impl PbType for MapTest {
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl Encode) {
        encode_map!(
            1,
            &self.map,
            WIRE_TYPE_LENGTH_ENCODED,
            WIRE_TYPE_VARINT,
            encoder,
            Encode::encode_str,
            Encode::encode_bool,
        )
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut map = HashMap::new();

        while !decoder.eof() {
            let tag = decoder.decode_uint32()?;
            let field_number = tag.field_number();
            let wire_type = tag.wire_type();

            match field_number {
                1 => {
                    decode_map!(
                        &mut map,
                        wire_type,
                        decoder,
                        Decode::decode_string,
                        Decode::decode_bool
                    );
                }
                n => return Err(DecodeError::UnexpectedFieldNumber(n)),
            }
        }

        Ok(Self { map })
    }
}

#[derive(Debug)]
struct Nested {
    number: i32,
}

impl PbType for Nested {
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl Encode) {
        encode_field!(1, SInt32, self.number, encoder, Encode::encode_sint32);
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut number = None;

        while !decoder.eof() {
            let tag = decoder.decode_uint32()?;
            let field_number = tag.field_number();
            let wire_type = tag.wire_type();

            println!("[CHILD] {field_number}:{wire_type}");

            match field_number {
                1 => decode_field!(SInt32, number, wire_type, decoder, Decode::decode_sint32),
                n => return Err(DecodeError::UnexpectedFieldNumber(n)),
            }
        }

        println!("number = {number:?}");

        Ok(Self {
            number: number.ok_or(DecodeError::MissingField(1))?,
        })
    }
}

// Test Message at https://www.protobufpal.com:
//
// syntax = "proto3";
// enum Enum {
//   ENUM_UNSPECIFIED = 0;
//   ENUM_A = 1;
//   ENUM_B = 2;
// }
// message Test {
//   float f32 = 1;
//   double f64 = 2;
//   int32 pos_i32 = 3;
//   int32 neg_i32 = 4;
//   int64 pos_i64 = 5;
//   int64 neg_i64 = 6;
//   uint32 u32 = 7;
//   uint64 u64 = 8;
//   sint32 pos_si32 = 9;
//   sint32 neg_si32 = 10;
//   sint64 pos_si64 = 11;
//   sint64 neg_si64 = 12;
//   fixed32 fixed_u32 = 13;
//   fixed64 fixed_u64 = 14;
//   sfixed32 pos_sfixed32 = 15;
//   sfixed32 neg_sfixed32 = 16;
//   sfixed64 pos_sfixed64 = 17;
//   sfixed64 neg_sfixed64 = 18;
//   bool b = 19;
//   string empty = 20;
//   string hello = 21;
//   Enum e = 22;
//   oneof o {
//     int32 i = 23;
//     string str = 24;
//   }
//   repeated uint32 packed = 25;
//   repeated uint32 unpacked = 26 [packed = false];
//   Nested nested = 27;
// }
// message Nested {
//   sint32 number = 1;
// }

// Test data at https://www.protobufpal.com:
// {
//   "f32": 3.14,
//   "f64": 3.14,
//   "pos_i32": 12345,
//   "neg_i32": -12345,
//   "pos_i64": 12345,
//   "neg_i64": -12345,
//   "u32": 1,
//   "u64": 1,
//   "pos_si32": 12345,
//   "neg_si32": -12345,
//   "pos_si64": 12345,
//   "neg_si64": -12345,
//   "fixed_u32": 123,
//   "fixed_u64": 123,
//   "pos_sfixed32": 12345,
//   "neg_sfixed32": -12345,
//   "pos_sfixed64": 12345,
//   "neg_sfixed64": -12345,
//   "b": false,
//   "empty": "",
//   "hello": "world",
//   "e": "A",
//   "o": {
//     "str": "oneof"
//   },
//   "packed": [1,2,3],
//   "unpacked": [1,2,3],
//   "nested": {
//     "number": 12345
//   }
// }

const PROTOBUF_PAL: &str = "DcP1SEARH4XrUbgeCUAYuWAgx5//////////ASi5YDDHn/////////8BOAFAAUjywAFQ8cABWPLAAWDxwAFtewAAAHF7AAAAAAAAAH05MAAAhQHHz///iQE5MAAAAAAAAJEBx8////////+YAQCiAQCqAQV3b3JsZLABAcIBBW9uZW9mygEDAQID0AEB0AEC0AED2gEECPLAAQ==";

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
        o: TestOneOf::Str(String::from("oneof")),
        packed: vec![1, 2, 3],
        unpacked: vec![1, 2, 3],
        nested: Nested { number: 12345 },
    };

    let size = test.size_hint();
    // ensure it has same size as protobuf pal
    assert_eq!(308 / 2, size);

    let mut buffer = BytesMut::with_capacity(size);
    test.encode(&mut buffer);

    for b in &buffer {
        print!("{:02x}", b);
    }
    println!();

    // ensure it is identical to protobuf pal
    assert_eq!(PROTOBUF_PAL, BASE64_STANDARD.encode(&buffer));
}

#[test]
fn decode() {
    // ensure we can read protobuf pal
    let data = BASE64_STANDARD.decode(PROTOBUF_PAL).unwrap();
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
    assert_eq!(test.o, TestOneOf::Str("oneof".into()));
    assert_eq!(test.packed, vec![1, 2, 3]);
    assert_eq!(test.unpacked, vec![1, 2, 3]);
    assert_eq!(test.nested.number, 12345);
}

#[test]
fn map_encode_decode() {
    let mut map = HashMap::new();
    map.insert(String::from("true"), true);
    map.insert(String::from("false"), false);

    let test = MapTest { map };

    let size = test.size_hint();
    // ensure it has same size as protobuf pal
    assert_eq!(21, size);

    let mut buffer = BytesMut::with_capacity(size);
    test.encode(&mut buffer);

    // re-read self encoded
    let read = MapTest::decode(&mut buffer).unwrap();
    assert!(read.map.get("true").unwrap());
    assert!(!read.map.get("false").unwrap());

    // read self protobuf pal, we cannot compare the base64 data here as it seems the sequence is different
    let data = BASE64_STANDARD
        .decode("CggKBHRydWUQAQoJCgVmYWxzZRAA")
        .unwrap();
    let mut bytes = bytes::Bytes::from(data);

    let read = MapTest::decode(&mut bytes).unwrap();
    assert!(read.map.get("true").unwrap());
    assert!(!read.map.get("false").unwrap());
}
