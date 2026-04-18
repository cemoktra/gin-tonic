use std::collections::BTreeMap;

use crate::{
    Decode, Encode, Message, Scalar, Tag,
    error::ProtoError,
    wire_types::{WIRE_TYPE_I32, WIRE_TYPE_I64, WIRE_TYPE_LENGTH_ENCODED, WIRE_TYPE_VARINT},
};

// i can treat any message as scalar type by encoding it as bytes
impl<T> Scalar<crate::scalars::Bytes> for T
where
    T: Message,
{
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl Encode) {
        <Self as Message>::encode(self, encoder);
    }

    fn decode(decoder: &mut impl Decode) -> Result<Self, ProtoError>
    where
        Self: Sized,
    {
        <Self as Message>::decode(decoder)
    }
}

#[derive(Debug)]
pub struct RawMessageView<'buf> {
    buffer: &'buf [u8],
    data: BTreeMap<Tag, Vec<std::ops::Range<usize>>>,
}

impl<'buf> RawMessageView<'buf> {
    pub fn iter(&self) -> impl Iterator<Item = (&Tag, &Vec<std::ops::Range<usize>>)> {
        self.data.iter()
    }

    pub fn tag_data(&self, tag: Tag) -> Box<dyn Iterator<Item = &'buf [u8]> + '_> {
        match self.data.get(&tag) {
            Some(positions) => Box::new(
                positions
                    .iter()
                    .map(|position| self.buffer(position.clone())),
            ),
            None => Box::new(
                std::iter::empty::<std::ops::Range<usize>>()
                    .map(|position| self.buffer(position.clone())),
            ),
        }
    }

    pub fn buffer(&self, position: std::ops::Range<usize>) -> &'buf [u8] {
        &self.buffer[position]
    }
}

impl<'buf> TryFrom<&'buf [u8]> for RawMessageView<'buf> {
    type Error = ProtoError;

    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error> {
        let mut data = BTreeMap::<Tag, Vec<std::ops::Range<usize>>>::default();
        let len = buffer.len();
        let mut pos = 0usize;

        while pos < len {
            let (tag, read) = varint_simd::decode::<u32>(&buffer[pos..])?;
            let tag = Tag::from(tag);
            pos += read;

            match tag.wire_type() {
                WIRE_TYPE_VARINT => {
                    let read = varint_simd::decode_len::<u64>(&buffer[pos..])?;
                    data.entry(tag).or_default().push(pos..pos + read);
                    pos += read;
                }
                WIRE_TYPE_LENGTH_ENCODED => {
                    let (size, read) = varint_simd::decode::<u32>(&buffer[pos..])?;
                    data.entry(tag)
                        .or_default()
                        .push(pos..pos + read + size as usize);
                    pos += read;
                    pos += size as usize;
                }
                WIRE_TYPE_I32 => {
                    data.entry(tag).or_default().push(pos..pos + 4);
                    pos += 4;
                }
                WIRE_TYPE_I64 => {
                    data.entry(tag).or_default().push(pos..pos + 8);
                    pos += 8;
                }
                n => panic!("unsupported wire type: {n}"),
            }
        }

        Ok(Self { buffer, data })
    }
}

#[cfg(test)]
mod test {

    use indexmap::IndexMap;

    use crate::{
        Map, Packed, RawMessageView, Scalar, Tag,
        decoder::Decoder,
        encoder::Encoder,
        error::ProtoError,
        scalars::{Int32, ProtoString, UInt64},
        wire_types::WIRE_TYPE_LENGTH_ENCODED,
    };

    mod shared {
        use super::*;

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub(super) struct Test1 {
            pub(super) a: i32,
        }

        impl crate::Message for Test1 {
            fn encode(&self, encoder: &mut impl crate::Encode) {
                let tag = Tag::from_parts(1u32, <i32 as Scalar<Int32>>::WIRE_TYPE);
                encoder.encode_tag(tag);
                Scalar::<Int32>::encode(&self.a, encoder);
            }

            fn decode_raw_message<'buf>(
                raw_message: RawMessageView<'buf>,
            ) -> Result<Self, crate::error::ProtoError>
            where
                Self: Sized,
            {
                let tag1 = Tag::from_parts(1, <i32 as Scalar<Int32>>::WIRE_TYPE);

                let a_bytes = raw_message
                    .tag_data(tag1)
                    .next()
                    .ok_or(ProtoError::MissingField(1))?;
                let mut a_decoder = Decoder::new(a_bytes);

                Ok(Self {
                    a: Scalar::<Int32>::decode(&mut a_decoder)?,
                })
            }
        }
    }

    #[test]
    fn message_1() {
        // message Test1 {
        //   int32 a = 1;
        // }

        let test1 = shared::Test1 { a: 150 };

        let size_hint = test1.size_hint();
        assert_eq!(size_hint, 3);

        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        test1.encode(&mut encoder);

        assert_eq!(size_hint, buffer.len());
        assert_eq!(b"\x08\x96\x01", &buffer[..size_hint]);

        let mut decoder = Decoder::new(&buffer);
        let test1_de = shared::Test1::decode(&mut decoder).unwrap();

        assert_eq!(test1, test1_de);
    }

    #[test]
    fn message_2() {
        // message Test2 {
        //   string b = 2;
        // }

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct Test2 {
            b: String,
        }

        impl crate::Message for Test2 {
            fn encode(&self, encoder: &mut impl crate::Encode) {
                let tag = Tag::from_parts(
                    2u32,
                    <String as Scalar<crate::scalars::ProtoString>>::WIRE_TYPE,
                );
                encoder.encode_tag(tag);
                Scalar::<crate::scalars::ProtoString>::encode(&self.b, encoder);
            }

            fn decode_raw_message<'buf>(
                raw_message: super::RawMessageView<'buf>,
            ) -> Result<Self, crate::error::ProtoError>
            where
                Self: Sized,
            {
                let tag1 = Tag::from_parts(
                    2,
                    <String as Scalar<crate::scalars::ProtoString>>::WIRE_TYPE,
                );

                let b_bytes = raw_message
                    .tag_data(tag1)
                    .next()
                    .ok_or(ProtoError::MissingField(2))?;
                let mut b_decoder = Decoder::new(b_bytes);

                Ok(Self {
                    b: Scalar::<crate::scalars::ProtoString>::decode(&mut b_decoder)?,
                })
            }
        }

        let test2 = Test2 {
            b: "protobuf".into(),
        };

        let size_hint = test2.size_hint();
        assert_eq!(size_hint, 10);

        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        test2.encode(&mut encoder);

        assert_eq!(size_hint, buffer.len());
        assert_eq!(
            b"\x12\x08\x70\x72\x6f\x74\x6f\x62\x75\x66",
            &buffer[..size_hint]
        );

        let mut decoder = Decoder::new(&buffer);
        let test2_de = Test2::decode(&mut decoder).unwrap();

        assert_eq!(test2, test2_de);
    }

    #[test]
    fn message_3() {
        // message Test3 {
        //   Test1 c = 3;
        // }

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub(crate) struct Test3 {
            pub(crate) c: shared::Test1,
        }

        impl crate::Message for Test3 {
            fn encode(&self, encoder: &mut impl crate::Encode) {
                let tag = Tag::from_parts(
                    3u32,
                    <shared::Test1 as Scalar<crate::scalars::Bytes>>::WIRE_TYPE,
                );
                encoder.encode_tag(tag);

                let size = self.c.size_hint();
                <u64 as Scalar<UInt64>>::encode(&(size as u64), encoder);
                <shared::Test1 as Scalar<crate::scalars::Bytes>>::encode(&self.c, encoder);
            }

            fn decode_raw_message<'buf>(
                raw_message: RawMessageView<'buf>,
            ) -> Result<Self, ProtoError>
            where
                Self: Sized,
            {
                let tag = Tag::from_parts(3u32, <Self as Scalar<crate::scalars::Bytes>>::WIRE_TYPE);

                let bytes = raw_message
                    .tag_data(tag)
                    .next()
                    .ok_or(ProtoError::MissingField(2))?;
                let mut decoder = Decoder::new(bytes);
                let _size = <u64 as Scalar<UInt64>>::decode(&mut decoder)?;

                Ok(Self {
                    c: <shared::Test1 as Scalar<crate::scalars::Bytes>>::decode(&mut decoder)?,
                })
            }
        }

        let test3 = Test3 {
            c: shared::Test1 { a: 150 },
        };

        let size_hint = test3.size_hint();
        assert_eq!(size_hint, 5);

        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        test3.encode(&mut encoder);

        assert_eq!(size_hint, buffer.len());
        assert_eq!(b"\x1a\x03\x08\x96\x01", &buffer[..size_hint]);

        let mut decoder = Decoder::new(&buffer);
        let test3_de = Test3::decode(&mut decoder).unwrap();

        assert_eq!(test3, test3_de);
    }

    #[test]
    fn message_4() {
        // message Test4 {
        //   string d = 4;
        //   repeated int32 e = 6;
        // }

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct Test4 {
            d: String,
            e: Vec<i32>,
        }

        impl crate::Message for Test4 {
            fn encode(&self, encoder: &mut impl crate::Encode) {
                let tag = Tag::from_parts(
                    4u32,
                    <String as Scalar<crate::scalars::ProtoString>>::WIRE_TYPE,
                );
                encoder.encode_tag(tag);
                encoder.encode_str(&self.d);

                <Vec<i32> as Packed<Int32>>::encode(&self.e, 6, encoder);
            }

            fn decode_raw_message<'buf>(
                raw_message: RawMessageView<'buf>,
            ) -> Result<Self, ProtoError>
            where
                Self: Sized,
            {
                let tag = Tag::from_parts(
                    4,
                    <String as Scalar<crate::scalars::ProtoString>>::WIRE_TYPE,
                );

                let bytes = raw_message
                    .tag_data(tag)
                    .next()
                    .ok_or(ProtoError::MissingField(2))?;
                let mut decoder = Decoder::new(bytes);
                let d = <String as Scalar<crate::scalars::ProtoString>>::decode(&mut decoder)?;

                let e = <Vec<i32> as Packed<Int32>>::decode(6, &raw_message)?;

                Ok(Self { d, e })
            }
        }

        let test4 = Test4 {
            d: "hello".into(),
            e: vec![1, 2, 3],
        };

        let size_hint = test4.size_hint();
        assert_eq!(size_hint, 12);

        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        test4.encode(&mut encoder);

        assert_eq!(size_hint, buffer.len());
        assert_eq!(
            b"\x22\x05\x68\x65\x6c\x6c\x6f\x32\x03\x01\x02\x03",
            &buffer[..size_hint]
        );

        let mut decoder = Decoder::new(&buffer);
        let test4_de = Test4::decode(&mut decoder).unwrap();

        assert_eq!(test4, test4_de);
    }

    #[test]
    fn message_6() {
        // message Test6 {
        //   map<string, int32> g = 7;
        // }

        #[derive(Debug, PartialEq, Eq)]
        struct Test6 {
            g: IndexMap<String, i32>,
        }

        impl crate::Message for Test6 {
            fn encode(&self, encoder: &mut impl crate::Encode) {
                let tag = Tag::from_parts(7u32, WIRE_TYPE_LENGTH_ENCODED);

                Map::<ProtoString, Int32>::encode(&self.g, tag, encoder);
            }

            fn decode_raw_message<'buf>(
                raw_message: RawMessageView<'buf>,
            ) -> Result<Self, ProtoError>
            where
                Self: Sized,
            {
                let tag = Tag::from_parts(7u32, WIRE_TYPE_LENGTH_ENCODED);

                let g = Map::<ProtoString, Int32>::decode(tag, &raw_message)?;

                Ok(Self { g })
            }
        }

        let mut g = IndexMap::new();
        g.insert(String::from("true"), 1);
        g.insert(String::from("false"), 0);
        let test6 = Test6 { g };

        let size_hint = test6.size_hint();
        assert_eq!(size_hint, 21);

        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        test6.encode(&mut encoder);

        assert_eq!(size_hint, buffer.len());
        assert_eq!(
            b"\x3a\x08\x0a\x04\x74\x72\x75\x65\x10\x01\x3a\x09\x0a\x05\x66\x61\x6c\x73\x65\x10\x00",
            &buffer[..size_hint]
        );

        let mut decoder = Decoder::new(&buffer);
        let test6_de = Test6::decode(&mut decoder).unwrap();

        assert_eq!(test6, test6_de);
    }
}
