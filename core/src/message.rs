use crate::{
    Decode, Encode, Message, Scalar, Tag, error::ProtoError, scalars::UInt64,
    wire_types::WIRE_TYPE_LENGTH_ENCODED,
};

// there is one slight difference between encoding an encapsulating message and treating a nested
// message as a scalar inside another message. the most outer message does not prepend a size while
// treating a message as scalar prepends the size of the length encoded block
impl<T> Scalar<crate::scalars::Bytes> for T
where
    T: Message,
{
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl Encode) {
        let size = self.message_size_hint();
        <u64 as Scalar<UInt64>>::encode(&(size as u64), encoder);

        <Self as Message>::encode_message(self, encoder);
    }

    fn encode_field(&self, field_number: u32, encoder: &mut impl Encode) {
        encoder.encode_tag(Tag::from_parts(
            field_number,
            <Self as Scalar<crate::scalars::Bytes>>::WIRE_TYPE,
        ));

        let size = self.message_size_hint();
        <u64 as Scalar<UInt64>>::encode(&(size as u64), encoder);

        self.encode_message(encoder);
    }

    #[allow(clippy::cast_possible_truncation)]
    fn decode(decoder: &mut impl Decode) -> Result<Self, ProtoError>
    where
        Self: Sized,
    {
        let size = <u64 as Scalar<UInt64>>::decode(decoder)? as usize;

        <Self as Message>::decode_message(&mut decoder.sub_decoder(size))
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use crate::{
        Decode, Map, Message, Packed, Scalar,
        decoder::Decoder,
        encoder::Encoder,
        error::ProtoError,
        scalars::{Int32, ProtoString},
    };

    mod shared {
        use crate::Decode;

        use super::*;

        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub(super) struct Test1 {
            pub(super) a: i32,
        }

        impl crate::Message for Test1 {
            fn encode_message(&self, encoder: &mut impl crate::Encode) {
                Scalar::<Int32>::encode_field(&self.a, 1, encoder);
            }

            fn decode_message<'buf>(
                decoder: &mut impl Decode,
            ) -> Result<Self, crate::error::ProtoError>
            where
                Self: Sized,
            {
                decoder.decode_tag()?;
                Ok(Self {
                    a: Scalar::<Int32>::decode(decoder)?,
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

        let size_hint = test1.message_size_hint();
        assert_eq!(size_hint, 3);

        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        test1.encode_message(&mut encoder);

        assert_eq!(size_hint, buffer.len());
        assert_eq!(b"\x08\x96\x01", &buffer[..size_hint]);

        let mut decoder = Decoder::new(&buffer);
        let test1_de = shared::Test1::decode_message(&mut decoder).unwrap();

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
            fn encode_message(&self, encoder: &mut impl crate::Encode) {
                Scalar::<crate::scalars::ProtoString>::encode_field(&self.b, 2, encoder);
            }

            fn decode_message<'buf>(
                decoder: &mut impl Decode,
            ) -> Result<Self, crate::error::ProtoError>
            where
                Self: Sized,
            {
                decoder.decode_tag()?;
                Ok(Self {
                    b: Scalar::<crate::scalars::ProtoString>::decode(decoder)?,
                })
            }
        }

        let test2 = Test2 {
            b: "protobuf".into(),
        };

        let size_hint = test2.message_size_hint();
        assert_eq!(size_hint, 10);

        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        test2.encode_message(&mut encoder);

        assert_eq!(size_hint, buffer.len());
        assert_eq!(
            b"\x12\x08\x70\x72\x6f\x74\x6f\x62\x75\x66",
            &buffer[..size_hint]
        );

        let mut decoder = Decoder::new(&buffer);
        let test2_de = Test2::decode_message(&mut decoder).unwrap();

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
            fn encode_message(&self, encoder: &mut impl crate::Encode) {
                Scalar::<crate::scalars::Bytes>::encode_field(&self.c, 3, encoder);
            }

            fn decode_message<'buf>(decoder: &mut impl Decode) -> Result<Self, ProtoError>
            where
                Self: Sized,
            {
                decoder.decode_tag()?;
                Ok(Self {
                    c: Scalar::<crate::scalars::Bytes>::decode(decoder)?,
                })
            }
        }

        let test3 = Test3 {
            c: shared::Test1 { a: 150 },
        };

        let size_hint = test3.message_size_hint();
        assert_eq!(size_hint, 5);

        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        test3.encode_message(&mut encoder);

        assert_eq!(size_hint, buffer.len());
        assert_eq!(b"\x1a\x03\x08\x96\x01", &buffer[..size_hint]);

        let mut decoder = Decoder::new(&buffer);
        let test3_de = Test3::decode_message(&mut decoder).unwrap();

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
            fn encode_message(&self, encoder: &mut impl crate::Encode) {
                Scalar::<crate::scalars::ProtoString>::encode_field(&self.d, 4, encoder);
                <Vec<i32> as Packed<Int32>>::encode(&self.e, 6, encoder);
            }

            fn decode_message<'buf>(decoder: &mut impl Decode) -> Result<Self, ProtoError>
            where
                Self: Sized,
            {
                decoder.decode_tag()?;
                let d = Scalar::<crate::scalars::ProtoString>::decode(decoder)?;
                decoder.decode_tag()?;
                let mut e = vec![];
                <Vec<i32> as Packed<Int32>>::decode(decoder, &mut e)?;
                Ok(Self { d, e })
            }
        }

        let test4 = Test4 {
            d: "hello".into(),
            e: vec![1, 2, 3],
        };

        let size_hint = test4.message_size_hint();
        assert_eq!(size_hint, 12);

        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        test4.encode_message(&mut encoder);

        assert_eq!(size_hint, buffer.len());
        assert_eq!(
            b"\x22\x05\x68\x65\x6c\x6c\x6f\x32\x03\x01\x02\x03",
            &buffer[..size_hint]
        );

        let mut decoder = Decoder::new(&buffer);
        let test4_de = Test4::decode_message(&mut decoder).unwrap();

        assert_eq!(test4, test4_de);
    }

    #[test]
    fn message_6() {
        // message Test6 {
        //   map<string, int32> g = 7;
        // }

        #[derive(Debug, PartialEq, Eq)]
        struct Test6 {
            g: HashMap<String, i32>,
        }

        impl crate::Message for Test6 {
            fn encode_message(&self, encoder: &mut impl crate::Encode) {
                Map::<ProtoString, Int32>::encode(&self.g, 7, encoder);
            }

            fn decode_message<'buf>(decoder: &mut impl Decode) -> Result<Self, ProtoError>
            where
                Self: Sized,
            {
                let mut g = HashMap::new();

                while !decoder.eof() {
                    decoder.decode_tag()?;

                    Map::<ProtoString, Int32>::decode(decoder, &mut g)?;
                }

                Ok(Self { g })
            }
        }

        let mut g = HashMap::new();
        g.insert(String::from("true"), 1);
        g.insert(String::from("false"), 0);
        let test6 = Test6 { g };

        let size_hint = test6.message_size_hint();
        assert_eq!(size_hint, 21);

        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        test6.encode_message(&mut encoder);

        assert_eq!(size_hint, buffer.len());

        let mut decoder = Decoder::new(&buffer);
        let test6_de = Test6::decode_message(&mut decoder).unwrap();

        assert_eq!(test6, test6_de);
    }
}
