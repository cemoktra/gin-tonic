mod scalars {
    mod required {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};

        #[derive(Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1, scalar = "int32")]
            int32: i32,
            #[gin(id = 2, scalar = "int64")]
            int64: i64,
            #[gin(id = 3, scalar = "uint32")]
            uint32: u32,
            #[gin(id = 4, scalar = "uint64")]
            uint64: u64,
            #[gin(id = 5, scalar = "sint32")]
            sint32: i32,
            #[gin(id = 6, scalar = "sint64")]
            sint64: i64,
            #[gin(id = 7, scalar = "fixed32")]
            fixed32: u32,
            #[gin(id = 8, scalar = "fixed64")]
            fixed64: u64,
            #[gin(id = 9, scalar = "sfixed32")]
            sfixed32: i32,
            #[gin(id = 10, scalar = "sfixed64")]
            sfixed64: i64,
            #[gin(id = 11)]
            string: String,
            #[gin(id = 12)]
            boolean: bool,
            #[gin(id = 13)]
            float: f32,
            #[gin(id = 14)]
            double: f64,
        }

        // impl gin_tonic_core::Message for Test {
        //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
        //         Scalar::<Int32>::encode_field(&self.int32, 1, encoder);
        //         Scalar::<Int64>::encode_field(&self.int64, 2, encoder);
        //         Scalar::<UInt32>::encode_field(&self.uint32, 3, encoder);
        //         Scalar::<UInt64>::encode_field(&self.uint64, 4, encoder);
        //         Scalar::<SInt32>::encode_field(&self.sint32, 5, encoder);
        //         Scalar::<SInt64>::encode_field(&self.sint64, 6, encoder);
        //         Scalar::<Fixed32>::encode_field(&self.fixed32, 7, encoder);
        //         Scalar::<Fixed64>::encode_field(&self.fixed64, 8, encoder);
        //         Scalar::<SFixed32>::encode_field(&self.sfixed32, 9, encoder);
        //         Scalar::<SFixed64>::encode_field(&self.sfixed64, 10, encoder);
        //         Scalar::<ProtoString>::encode_field(&self.string, 11, encoder);
        //         Scalar::<Bool>::encode_field(&self.boolean, 12, encoder);
        //         Scalar::<Float>::encode_field(&self.float, 13, encoder);
        //         Scalar::<Double>::encode_field(&self.double, 14, encoder);
        //     }

        //     fn decode_raw_message<'buf>(
        //         raw_message: gin_tonic_core::RawMessageView<'buf>,
        //     ) -> Result<Self, ProtoError>
        //     where
        //         Self: Sized,
        //     {
        //         Ok(Self {
        //             int32: Scalar::<Int32>::decode_field(1, &raw_message)?,
        //             int64: Scalar::<Int64>::decode_field(2, &raw_message)?,
        //             uint32: Scalar::<UInt32>::decode_field(3, &raw_message)?,
        //             uint64: Scalar::<UInt64>::decode_field(4, &raw_message)?,
        //             sint32: Scalar::<SInt32>::decode_field(5, &raw_message)?,
        //             sint64: Scalar::<SInt64>::decode_field(6, &raw_message)?,
        //             fixed32: Scalar::<Fixed32>::decode_field(7, &raw_message)?,
        //             fixed64: Scalar::<Fixed64>::decode_field(8, &raw_message)?,
        //             sfixed32: Scalar::<SFixed32>::decode_field(9, &raw_message)?,
        //             sfixed64: Scalar::<SFixed64>::decode_field(10, &raw_message)?,
        //             string: Scalar::<ProtoString>::decode_field(11, &raw_message)?,
        //             boolean: Scalar::<Bool>::decode_field(12, &raw_message)?,
        //             float: Scalar::<Float>::decode_field(13, &raw_message)?,
        //             double: Scalar::<Double>::decode_field(14, &raw_message)?,
        //         })
        //     }
        // }

        #[test]
        fn encode_decode() {
            let test = Test {
                int32: -1234,
                int64: -1234,
                uint32: 1234,
                uint64: 1234,
                sint32: -1234,
                sint64: -1234,
                fixed32: 1234,
                fixed64: 1234,
                sfixed32: -1234,
                sfixed64: -1234,
                string: "protobuf".into(),
                boolean: true,
                double: 3.14,
                float: 3.14,
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            assert_eq!(
                &buffer,
                b"\x08\xae\xf6\xff\xff\xff\xff\xff\xff\xff\x01\x10\xae\xf6\xff\xff\xff\xff\xff\xff\xff\x01\x18\xd2\x09\x20\xd2\x09\x28\xa3\x13\x30\xa3\x13\x3d\xd2\x04\x00\x00\x41\xd2\x04\x00\x00\x00\x00\x00\x00\x4d\x2e\xfb\xff\xff\x51\x2e\xfb\xff\xff\xff\xff\xff\xff\x5a\x08\x70\x72\x6f\x74\x6f\x62\x75\x66\x60\x01\x6d\xc3\xf5\x48\x40\x71\x1f\x85\xeb\x51\xb8\x1e\x09\x40"
            );

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }
    }

    mod optional {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};

        #[derive(Default, Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1, scalar = "int32")]
            int32: Option<i32>,
            #[gin(id = 2, scalar = "int64")]
            int64: Option<i64>,
            #[gin(id = 3, scalar = "uint32")]
            uint32: Option<u32>,
            #[gin(id = 4, scalar = "uint64")]
            uint64: Option<u64>,
            #[gin(id = 5, scalar = "sint32")]
            sint32: Option<i32>,
            #[gin(id = 6, scalar = "sint64")]
            sint64: Option<i64>,
            #[gin(id = 7, scalar = "fixed32")]
            fixed32: Option<u32>,
            #[gin(id = 8, scalar = "fixed64")]
            fixed64: Option<u64>,
            #[gin(id = 9, scalar = "sfixed32")]
            sfixed32: Option<i32>,
            #[gin(id = 10, scalar = "sfixed64")]
            sfixed64: Option<i64>,
            #[gin(id = 11)]
            string: Option<String>,
            #[gin(id = 12)]
            boolean: Option<bool>,
            #[gin(id = 13)]
            float: Option<f32>,
            #[gin(id = 14)]
            double: Option<f64>,
        }

        // impl gin_tonic_core::Message for Test {
        //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
        //         if let Some(int32) = &self.int32 {
        //             Scalar::<Int32>::encode_field(int32, 1, encoder);
        //         }
        //         if let Some(int64) = &self.int64 {
        //             Scalar::<Int64>::encode_field(int64, 2, encoder);
        //         }
        //         if let Some(uint32) = &self.uint32 {
        //             Scalar::<UInt32>::encode_field(uint32, 3, encoder);
        //         }
        //         if let Some(uint64) = &self.uint64 {
        //             Scalar::<UInt64>::encode_field(uint64, 4, encoder);
        //         }
        //         if let Some(sint32) = &self.sint32 {
        //             Scalar::<SInt32>::encode_field(sint32, 5, encoder);
        //         }
        //         if let Some(sint64) = &self.sint64 {
        //             Scalar::<SInt64>::encode_field(sint64, 6, encoder);
        //         }
        //         if let Some(fixed32) = &self.fixed32 {
        //             Scalar::<Fixed32>::encode_field(fixed32, 7, encoder);
        //         }
        //         if let Some(fixed64) = &self.fixed64 {
        //             Scalar::<Fixed64>::encode_field(fixed64, 8, encoder);
        //         }
        //         if let Some(sfixed32) = &self.sfixed32 {
        //             Scalar::<SFixed32>::encode_field(sfixed32, 9, encoder);
        //         }
        //         if let Some(sfixed64) = &self.sfixed64 {
        //             Scalar::<SFixed64>::encode_field(sfixed64, 10, encoder);
        //         }
        //         if let Some(string) = &self.string {
        //             Scalar::<ProtoString>::encode_field(string, 11, encoder);
        //         }
        //         if let Some(boolean) = &self.boolean {
        //             Scalar::<Bool>::encode_field(boolean, 12, encoder);
        //         }
        //         if let Some(float) = &self.float {
        //             Scalar::<Float>::encode_field(float, 13, encoder);
        //         }
        //         if let Some(double) = &self.double {
        //             Scalar::<Double>::encode_field(double, 14, encoder);
        //         }
        //     }

        //     fn decode_raw_message<'buf>(
        //         raw_message: gin_tonic_core::RawMessageView<'buf>,
        //     ) -> Result<Self, gin_tonic_core::ProtoError>
        //     where
        //         Self: Sized,
        //     {
        //         let int32 = match Scalar::<Int32>::decode_field(1, &raw_message) {
        //             Ok(int32) => Some(int32),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let int64 = match Scalar::<Int64>::decode_field(2, &raw_message) {
        //             Ok(int64) => Some(int64),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let uint32 = match Scalar::<UInt32>::decode_field(3, &raw_message) {
        //             Ok(uint32) => Some(uint32),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let uint64 = match Scalar::<UInt64>::decode_field(4, &raw_message) {
        //             Ok(uint64) => Some(uint64),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let sint32 = match Scalar::<SInt32>::decode_field(5, &raw_message) {
        //             Ok(sint32) => Some(sint32),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let sint64 = match Scalar::<SInt64>::decode_field(6, &raw_message) {
        //             Ok(sint64) => Some(sint64),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let fixed32 = match Scalar::<Fixed32>::decode_field(7, &raw_message) {
        //             Ok(fixed32) => Some(fixed32),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let fixed64 = match Scalar::<Fixed64>::decode_field(8, &raw_message) {
        //             Ok(fixed64) => Some(fixed64),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let sfixed32 = match Scalar::<SFixed32>::decode_field(9, &raw_message) {
        //             Ok(sfixed32) => Some(sfixed32),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let sfixed64 = match Scalar::<SFixed64>::decode_field(10, &raw_message) {
        //             Ok(sfixed64) => Some(sfixed64),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let string = match Scalar::<ProtoString>::decode_field(11, &raw_message) {
        //             Ok(string) => Some(string),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let boolean = match Scalar::<Bool>::decode_field(12, &raw_message) {
        //             Ok(boolean) => Some(boolean),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let float = match Scalar::<Float>::decode_field(13, &raw_message) {
        //             Ok(float) => Some(float),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let double = match Scalar::<Double>::decode_field(14, &raw_message) {
        //             Ok(double) => Some(double),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };

        //         Ok(Self {
        //             int32,
        //             int64,
        //             uint32,
        //             uint64,
        //             sint32,
        //             sint64,
        //             fixed32,
        //             fixed64,
        //             sfixed32,
        //             sfixed64,
        //             string,
        //             boolean,
        //             float,
        //             double,
        //         })
        //     }
        // }

        #[test]
        fn encode_decode_some() {
            let test = Test {
                int32: Some(-1234),
                int64: Some(-1234),
                uint32: Some(1234),
                uint64: Some(1234),
                sint32: Some(-1234),
                sint64: Some(-1234),
                fixed32: Some(1234),
                fixed64: Some(1234),
                sfixed32: Some(-1234),
                sfixed64: Some(-1234),
                string: Some("protobuf".into()),
                boolean: Some(true),
                float: Some(3.14),
                double: Some(3.14),
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(
                &buffer,
                b"\x08\xae\xf6\xff\xff\xff\xff\xff\xff\xff\x01\x10\xae\xf6\xff\xff\xff\xff\xff\xff\xff\x01\x18\xd2\x09\x20\xd2\x09\x28\xa3\x13\x30\xa3\x13\x3d\xd2\x04\x00\x00\x41\xd2\x04\x00\x00\x00\x00\x00\x00\x4d\x2e\xfb\xff\xff\x51\x2e\xfb\xff\xff\xff\xff\xff\xff\x5a\x08\x70\x72\x6f\x74\x6f\x62\x75\x66\x60\x01\x6d\xc3\xf5\x48\x40\x71\x1f\x85\xeb\x51\xb8\x1e\x09\x40"
            );

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_none() {
            let test = Test::default();

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }
    }

    mod repeated {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};

        #[derive(Default, Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1, scalar = "int32")]
            int32: Vec<i32>,
            #[gin(id = 2, scalar = "int64")]
            int64: Vec<i64>,
            #[gin(id = 3, scalar = "uint32")]
            uint32: Vec<u32>,
            #[gin(id = 4, scalar = "uint64")]
            uint64: Vec<u64>,
            #[gin(id = 5, scalar = "sint32")]
            sint32: Vec<i32>,
            #[gin(id = 6, scalar = "sint64")]
            sint64: Vec<i64>,
            #[gin(id = 7, scalar = "fixed32")]
            fixed32: Vec<u32>,
            #[gin(id = 8, scalar = "fixed64")]
            fixed64: Vec<u64>,
            #[gin(id = 9, scalar = "sfixed32")]
            sfixed32: Vec<i32>,
            #[gin(id = 10, scalar = "sfixed64")]
            sfixed64: Vec<i64>,
            #[gin(id = 11)]
            string: Vec<String>,
            #[gin(id = 12)]
            boolean: Vec<bool>,
            #[gin(id = 13)]
            float: Vec<f32>,
            #[gin(id = 14)]
            double: Vec<f64>,
        }

        // impl gin_tonic_core::Message for Test {
        //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
        //         Packed::<Int32>::encode(&self.int32, 1, encoder);
        //         Packed::<Int64>::encode(&self.int64, 2, encoder);
        //         Packed::<UInt32>::encode(&self.uint32, 3, encoder);
        //         Packed::<UInt64>::encode(&self.uint64, 4, encoder);
        //         Packed::<SInt32>::encode(&self.sint32, 5, encoder);
        //         Packed::<SInt64>::encode(&self.sint64, 6, encoder);
        //         Packed::<Fixed32>::encode(&self.fixed32, 7, encoder);
        //         Packed::<Fixed64>::encode(&self.fixed64, 8, encoder);
        //         Packed::<SFixed32>::encode(&self.sfixed32, 9, encoder);
        //         Packed::<SFixed64>::encode(&self.sfixed64, 10, encoder);
        //         Unpacked::<ProtoString>::encode(
        //             &self.string,
        //             Tag::from_parts(11, WIRE_TYPE_LENGTH_ENCODED),
        //             encoder,
        //         );
        //         Packed::<Bool>::encode(&self.boolean, 12, encoder);
        //         Packed::<Float>::encode(&self.float, 13, encoder);
        //         Packed::<Double>::encode(&self.double, 14, encoder);
        //     }

        //     fn decode_raw_message<'buf>(
        //         raw_message: gin_tonic_core::RawMessageView<'buf>,
        //     ) -> Result<Self, ProtoError>
        //     where
        //         Self: Sized,
        //     {
        //         Ok(Self {
        //             int32: <Vec<i32> as Packed<Int32>>::decode(1, &raw_message)?,
        //             int64: <Vec<i64> as Packed<Int64>>::decode(2, &raw_message)?,
        //             uint32: <Vec<u32> as Packed<UInt32>>::decode(3, &raw_message)?,
        //             uint64: <Vec<u64> as Packed<UInt64>>::decode(4, &raw_message)?,
        //             sint32: <Vec<i32> as Packed<SInt32>>::decode(5, &raw_message)?,
        //             sint64: <Vec<i64> as Packed<SInt64>>::decode(6, &raw_message)?,
        //             fixed32: <Vec<u32> as Packed<Fixed32>>::decode(7, &raw_message)?,
        //             fixed64: <Vec<u64> as Packed<Fixed64>>::decode(8, &raw_message)?,
        //             sfixed32: <Vec<i32> as Packed<SFixed32>>::decode(9, &raw_message)?,
        //             sfixed64: <Vec<i64> as Packed<SFixed64>>::decode(10, &raw_message)?,
        //             string: <Vec<String> as Unpacked<ProtoString>>::decode(
        //                 Tag::from_parts(11, WIRE_TYPE_LENGTH_ENCODED),
        //                 &raw_message,
        //             )?,
        //             boolean: <Vec<bool> as Packed<Bool>>::decode(12, &raw_message)?,
        //             float: <Vec<f32> as Packed<Float>>::decode(13, &raw_message)?,
        //             double: <Vec<f64> as Packed<Double>>::decode(14, &raw_message)?,
        //         })
        //     }
        // }

        #[test]
        fn encode_decode_non_empty() {
            let test = Test {
                int32: vec![1, 2, -3],
                int64: vec![1, 2, -3],
                uint32: vec![1, 2, 3],
                uint64: vec![1, 2, 3],
                sint32: vec![1, 2, -3],
                sint64: vec![1, 2, -3],
                fixed32: vec![1, 2, 3],
                fixed64: vec![1, 2, 3],
                sfixed32: vec![1, 2, -3],
                sfixed64: vec![1, 2, -3],
                string: vec!["hello".into(), "world".into()],
                boolean: vec![true, false],
                float: vec![3.14, 3.14],
                double: vec![3.14, 3.14],
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(buffer, b"\x0a\x0c\x01\x02\xfd\xff\xff\xff\xff\xff\xff\xff\xff\x01\x12\x0c\x01\x02\xfd\xff\xff\xff\xff\xff\xff\xff\xff\x01\x1a\x03\x01\x02\x03\x22\x03\x01\x02\x03\x2a\x03\x02\x04\x05\x32\x03\x02\x04\x05\x3a\x0c\x01\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00\x42\x18\x01\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00\x4a\x0c\x01\x00\x00\x00\x02\x00\x00\x00\xfd\xff\xff\xff\x52\x18\x01\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\xfd\xff\xff\xff\xff\xff\xff\xff\x5a\x05\x68\x65\x6c\x6c\x6f\x5a\x05\x77\x6f\x72\x6c\x64\x62\x02\x01\x00\x6a\x08\xc3\xf5\x48\x40\xc3\xf5\x48\x40\x72\x10\x1f\x85\xeb\x51\xb8\x1e\x09\x40\x1f\x85\xeb\x51\xb8\x1e\x09\x40");

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_empty() {
            let test = Test::default();

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }
    }
}

mod nested {
    #[derive(Debug, PartialEq, gin_tonic_derive::Message)]
    #[gin(root = "crate")]
    struct Nested {
        #[gin(id = 1, scalar = "int32")]
        int32: i32,
    }

    // impl gin_tonic_core::Message for Nested {
    //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
    //         Scalar::<Int32>::encode_field(&self.int32, 1, encoder);
    //     }

    //     fn decode_raw_message<'buf>(
    //         raw_message: gin_tonic_core::RawMessageView<'buf>,
    //     ) -> Result<Self, gin_tonic_core::ProtoError>
    //     where
    //         Self: Sized,
    //     {
    //         Ok(Self {
    //             int32: Scalar::<Int32>::decode_field(1, &raw_message)?,
    //         })
    //     }
    // }

    mod required {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};

        #[derive(Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1)]
            nested_1: super::Nested,
            #[gin(id = 2)]
            nested_2: super::Nested,
            #[gin(id = 3)]
            nested_3: super::Nested,
            #[gin(id = 4)]
            nested_4: super::Nested,
        }

        // impl gin_tonic_core::Message for Test {
        //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
        //         Scalar::<Bytes>::encode_field(&self.nested_1, 1, encoder);
        //         Scalar::<Bytes>::encode_field(&self.nested_2, 2, encoder);
        //         Scalar::<Bytes>::encode_field(&self.nested_3, 3, encoder);
        //         Scalar::<Bytes>::encode_field(&self.nested_4, 4, encoder);
        //     }

        //     fn decode_raw_message<'buf>(
        //         raw_message: gin_tonic_core::RawMessageView<'buf>,
        //     ) -> Result<Self, gin_tonic_core::ProtoError>
        //     where
        //         Self: Sized,
        //     {
        //         Ok(Self {
        //             nested_1: Scalar::<Bytes>::decode_field(1, &raw_message)?,
        //             nested_2: Scalar::<Bytes>::decode_field(2, &raw_message)?,
        //             nested_3: Scalar::<Bytes>::decode_field(3, &raw_message)?,
        //             nested_4: Scalar::<Bytes>::decode_field(4, &raw_message)?,
        //         })
        //     }
        // }

        #[test]
        fn encode_decode() {
            let test = Test {
                nested_1: super::Nested { int32: 123 },
                nested_2: super::Nested { int32: 123 },
                nested_3: super::Nested { int32: 123 },
                nested_4: super::Nested { int32: 123 },
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(
                buffer,
                b"\x0a\x02\x08\x7b\x12\x02\x08\x7b\x1a\x02\x08\x7b\x22\x02\x08\x7b"
            );

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }
    }

    mod optional {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};

        #[derive(Default, Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1)]
            nested_1: Option<super::Nested>,
            #[gin(id = 2)]
            nested_2: Option<super::Nested>,
            #[gin(id = 3)]
            nested_3: Option<super::Nested>,
            #[gin(id = 4)]
            nested_4: Option<super::Nested>,
        }

        // impl gin_tonic_core::Message for Test {
        //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
        //         if let Some(nested_1) = &self.nested_1 {
        //             Scalar::<Bytes>::encode_field(nested_1, 1, encoder);
        //         }
        //         if let Some(nested_2) = &self.nested_2 {
        //             Scalar::<Bytes>::encode_field(nested_2, 2, encoder);
        //         }
        //         if let Some(nested_3) = &self.nested_3 {
        //             Scalar::<Bytes>::encode_field(nested_3, 3, encoder);
        //         }
        //         if let Some(nested_4) = &self.nested_4 {
        //             Scalar::<Bytes>::encode_field(nested_4, 4, encoder);
        //         }
        //     }

        //     fn decode_raw_message<'buf>(
        //         raw_message: gin_tonic_core::RawMessageView<'buf>,
        //     ) -> Result<Self, gin_tonic_core::ProtoError>
        //     where
        //         Self: Sized,
        //     {
        //         let nested_1 = match Scalar::<Bytes>::decode_field(1, &raw_message) {
        //             Ok(nested_1) => Some(nested_1),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let nested_2 = match Scalar::<Bytes>::decode_field(2, &raw_message) {
        //             Ok(nested_2) => Some(nested_2),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let nested_3 = match Scalar::<Bytes>::decode_field(3, &raw_message) {
        //             Ok(nested_3) => Some(nested_3),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };
        //         let nested_4 = match Scalar::<Bytes>::decode_field(4, &raw_message) {
        //             Ok(nested_4) => Some(nested_4),
        //             Err(ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };

        //         Ok(Self {
        //             nested_1,
        //             nested_2,
        //             nested_3,
        //             nested_4,
        //         })
        //     }
        // }

        #[test]
        fn encode_decode_some() {
            let test = Test {
                nested_1: Some(super::Nested { int32: 123 }),
                nested_2: Some(super::Nested { int32: 123 }),
                nested_3: Some(super::Nested { int32: 123 }),
                nested_4: Some(super::Nested { int32: 123 }),
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_none() {
            let test = Test::default();

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }
    }

    mod repeated {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};
        #[derive(Default, Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1)]
            nested_1: Vec<super::Nested>,
            #[gin(id = 2)]
            nested_2: Vec<super::Nested>,
            #[gin(id = 3)]
            nested_3: Vec<super::Nested>,
            #[gin(id = 4)]
            nested_4: Vec<super::Nested>,
        }

        // impl gin_tonic_core::Message for Test {
        //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
        //         Unpacked::<Bytes>::encode(
        //             &self.nested_1,
        //             Tag::from_parts(1, WIRE_TYPE_LENGTH_ENCODED),
        //             encoder,
        //         );
        //         Unpacked::<Bytes>::encode(
        //             &self.nested_2,
        //             Tag::from_parts(2, WIRE_TYPE_LENGTH_ENCODED),
        //             encoder,
        //         );
        //         Unpacked::<Bytes>::encode(
        //             &self.nested_3,
        //             Tag::from_parts(3, WIRE_TYPE_LENGTH_ENCODED),
        //             encoder,
        //         );
        //         Unpacked::<Bytes>::encode(
        //             &self.nested_4,
        //             Tag::from_parts(4, WIRE_TYPE_LENGTH_ENCODED),
        //             encoder,
        //         );
        //     }

        //     fn decode_raw_message<'buf>(
        //         raw_message: gin_tonic_core::RawMessageView<'buf>,
        //     ) -> Result<Self, gin_tonic_core::ProtoError>
        //     where
        //         Self: Sized,
        //     {
        //         Ok(Self {
        //             nested_1: <Vec<super::Nested> as Unpacked<Bytes>>::decode(
        //                 Tag::from_parts(1, WIRE_TYPE_LENGTH_ENCODED),
        //                 &raw_message,
        //             )?,
        //             nested_2: <Vec<super::Nested> as Unpacked<Bytes>>::decode(
        //                 Tag::from_parts(2, WIRE_TYPE_LENGTH_ENCODED),
        //                 &raw_message,
        //             )?,
        //             nested_3: <Vec<super::Nested> as Unpacked<Bytes>>::decode(
        //                 Tag::from_parts(3, WIRE_TYPE_LENGTH_ENCODED),
        //                 &raw_message,
        //             )?,
        //             nested_4: <Vec<super::Nested> as Unpacked<Bytes>>::decode(
        //                 Tag::from_parts(4, WIRE_TYPE_LENGTH_ENCODED),
        //                 &raw_message,
        //             )?,
        //         })
        //     }
        // }

        #[test]
        fn encode_decode_non_empty() {
            let test = Test {
                nested_1: vec![super::Nested { int32: 123 }, super::Nested { int32: 123 }],
                nested_2: vec![super::Nested { int32: 123 }],
                nested_3: vec![super::Nested { int32: 123 }],
                nested_4: vec![super::Nested { int32: 123 }],
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(
                buffer,
                b"\x0a\x02\x08\x7b\x0a\x02\x08\x7b\x12\x02\x08\x7b\x1a\x02\x08\x7b\x22\x02\x08\x7b"
            );

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_empty() {
            let test = Test::default();

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }
    }
}

mod map {
    mod required {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};
        use indexmap::IndexMap;

        #[derive(Default, Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1, key_scalar = "uint32")]
            map_1: IndexMap<u32, String>,
            #[gin(id = 2, value_scalar = "uint32")]
            map_2: IndexMap<String, u32>,
        }

        // impl gin_tonic_core::Message for Test {
        //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
        //         Map::<UInt32, ProtoString>::encode(&self.map_1, 1, encoder);
        //         Map::<ProtoString, UInt32>::encode(&self.map_2, 2, encoder);
        //     }

        //     fn decode_raw_message<'buf>(
        //         raw_message: gin_tonic_core::RawMessageView<'buf>,
        //     ) -> Result<Self, gin_tonic_core::ProtoError>
        //     where
        //         Self: Sized,
        //     {
        //         Ok(Self {
        //             map_1: Map::<UInt32, ProtoString>::decode(1, &raw_message)?,
        //             map_2: Map::<ProtoString, UInt32>::decode(2, &raw_message)?,
        //         })
        //     }
        // }

        #[test]
        fn encode_decode_non_empty() {
            let mut map_1 = IndexMap::new();
            map_1.insert(10, "ten".into());
            map_1.insert(20, "twenty".into());

            let mut map_2 = IndexMap::new();
            map_2.insert("ten".into(), 10);
            map_2.insert("twenty".into(), 20);

            let test = Test { map_1, map_2 };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(buffer,
                b"\x0a\x07\x08\x0a\x12\x03\x74\x65\x6e\x0a\x0a\x08\x14\x12\x06\x74\x77\x65\x6e\x74\x79\x12\x07\x0a\x03\x74\x65\x6e\x10\x0a\x12\x0a\x0a\x06\x74\x77\x65\x6e\x74\x79\x10\x14");

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_empty() {
            let map_1 = IndexMap::new();
            let map_2 = IndexMap::new();

            let test = Test { map_1, map_2 };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }
    }
}

mod enumeration {
    #[derive(Debug, PartialEq, gin_tonic_derive::Enumeration)]
    #[gin(root = "crate")]
    enum TestEnum {
        #[gin(id = 1)]
        A,
        #[gin(id = 2)]
        B,
    }

    // impl gin_tonic_core::PackableMarker<UInt32> for TestEnum {}

    // impl gin_tonic_core::Scalar<UInt32> for TestEnum {
    //     const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    //     fn encode(&self, encoder: &mut impl gin_tonic_core::Encode) {
    //         let value = match self {
    //             TestEnum::A => 1u32,
    //             TestEnum::B => 2u32,
    //         };

    //         Scalar::<UInt32>::encode(&value, encoder)
    //     }

    //     fn decode(
    //         decoder: &mut impl gin_tonic_core::Decode,
    //     ) -> Result<Self, gin_tonic_core::ProtoError>
    //     where
    //         Self: Sized,
    //     {
    //         let value = <u32 as Scalar<UInt32>>::decode(decoder)?;
    //         match value {
    //             1 => Ok(Self::A),
    //             2 => Ok(Self::B),
    //             n => Err(gin_tonic_core::ProtoError::UnknownEnumVariant(n)),
    //         }
    //     }
    // }

    mod required {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};

        #[derive(Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1, scalar = "uint32")]
            e: super::TestEnum,
        }

        // impl gin_tonic_core::Message for Test {
        //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
        //         self.e.encode_field(1, encoder);
        //     }

        //     fn decode_raw_message<'buf>(
        //         raw_message: gin_tonic_core::RawMessageView<'buf>,
        //     ) -> Result<Self, gin_tonic_core::ProtoError>
        //     where
        //         Self: Sized,
        //     {
        //         Ok(Self {
        //             e: <super::TestEnum as Scalar<UInt32>>::decode(&mut Decoder::new(
        //                 raw_message
        //                     .tag_data(Tag::from_parts(1, WIRE_TYPE_VARINT))
        //                     .rev()
        //                     .next()
        //                     .ok_or(gin_tonic_core::ProtoError::MissingField(1))?,
        //             ))?,
        //         })
        //     }
        // }

        #[test]
        fn encode_decode_a() {
            let test = Test {
                e: super::TestEnum::A,
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(buffer, b"\x08\x01");

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_b() {
            let test = Test {
                e: super::TestEnum::B,
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(buffer, b"\x08\x02");

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn decode_last_variant_wins() {
            let buffer = b"\x08\x01\x08\x02";

            let mut decoder = Decoder::new(&buffer[..]);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(super::TestEnum::B, read.e);
        }
    }

    mod optional {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};

        #[derive(Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1, scalar = "uint32")]
            e: Option<super::TestEnum>,
        }

        // impl gin_tonic_core::Message for Test {
        //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
        //         if let Some(e) = &self.e {
        //             e.encode_field(1, encoder);
        //         }
        //     }

        //     fn decode_raw_message<'buf>(
        //         raw_message: gin_tonic_core::RawMessageView<'buf>,
        //     ) -> Result<Self, gin_tonic_core::ProtoError>
        //     where
        //         Self: Sized,
        //     {
        //         let e = match <super::TestEnum as Scalar<UInt32>>::decode_field(1, &raw_message) {
        //             Ok(e) => Some(e),
        //             Err(gin_tonic_core::ProtoError::MissingField(_)) => None,
        //             Err(err) => return Err(err),
        //         };

        //         Ok(Self { e })
        //     }
        // }

        #[test]
        fn encode_decode_some() {
            let test = Test {
                e: Some(super::TestEnum::A),
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(buffer, b"\x08\x01");

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_none() {
            let test = Test { e: None };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }
    }

    mod repeated {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};

        #[derive(Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1, scalar = "uint32")]
            e: Vec<super::TestEnum>,
        }

        // impl gin_tonic_core::Message for Test {
        //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
        //         <Vec<super::TestEnum> as Packed<UInt32>>::encode(&self.e, 1, encoder);
        //     }

        //     fn decode_raw_message<'buf>(
        //         raw_message: gin_tonic_core::RawMessageView<'buf>,
        //     ) -> Result<Self, gin_tonic_core::ProtoError>
        //     where
        //         Self: Sized,
        //     {
        //         Ok(Self {
        //             e: <Vec<super::TestEnum> as Packed<UInt32>>::decode(1, &raw_message)?,
        //         })
        //     }
        // }

        #[test]
        fn encode_decode_non_empty() {
            let test = Test {
                e: vec![super::TestEnum::B, super::TestEnum::B, super::TestEnum::A],
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(buffer, b"\x0a\x03\x02\x02\x01");

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_empty() {
            let test = Test { e: vec![] };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }
    }
}

mod one_of {
    #[derive(Debug, PartialEq, gin_tonic_derive::OneOf)]
    #[gin(root = "crate")]
    enum OneOf {
        #[gin(id = 1)]
        A(String),
        #[gin(id = 2, scalar = "uint32")]
        B(u32),
    }

    //     impl gin_tonic_core::Message for OneOf {
    //         fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
    //             match self {
    //                 OneOf::A(value) => Scalar::<ProtoString>::encode_field(value, 1, encoder),
    //                 OneOf::B(value) => Scalar::<UInt32>::encode_field(value, 2, encoder),
    //             }
    //         }

    //         fn decode_raw_message<'buf>(
    //             raw_message: gin_tonic_core::RawMessageView<'buf>,
    //         ) -> Result<Self, gin_tonic_core::ProtoError>
    //         where
    //             Self: Sized,
    //         {
    //             let mut slf = None;

    //             if let Some(bytes) = raw_message
    //                 .tag_data(Tag::from_parts(
    //                     1,
    //                     <String as Scalar<ProtoString>>::WIRE_TYPE,
    //                 ))
    //                 .rev()
    //                 .next()
    //             {
    //                 slf = Some(Self::A(<String as Scalar<ProtoString>>::decode(
    //                     &mut Decoder::new(bytes),
    //                 )?));
    //             }

    //             if let Some(bytes) = raw_message
    //                 .tag_data(Tag::from_parts(2, <u32 as Scalar<UInt32>>::WIRE_TYPE))
    //                 .rev()
    //                 .next()
    //             {
    //                 slf = Some(Self::B(<u32 as Scalar<UInt32>>::decode(
    //                     &mut Decoder::new(bytes),
    //                 )?));
    //             }

    //             slf.ok_or(gin_tonic_core::ProtoError::MissingOneOf(&[1, 2]))
    //         }
    //     }

    #[derive(Debug, PartialEq, gin_tonic_derive::Message)]
    #[gin(root = "crate")]
    struct WrappedOneOf {
        #[gin(id = 1, oneof)]
        oneof: OneOf,
    }

    // impl gin_tonic_core::Message for WrappedOneOf {
    //     fn encode_message(&self, encoder: &mut impl gin_tonic_core::Encode) {
    //         self.oneof.encode_message(encoder);
    //     }

    //     fn decode_raw_message<'buf>(
    //         raw_message: gin_tonic_core::RawMessageView<'buf>,
    //     ) -> Result<Self, gin_tonic_core::ProtoError>
    //     where
    //         Self: Sized,
    //     {
    //         Ok(Self {
    //             oneof: OneOf::decode_raw_message(raw_message)?,
    //         })
    //     }
    // }

    mod wrapped {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};

        #[test]
        fn encode_decode_a() {
            let test = super::WrappedOneOf {
                oneof: super::OneOf::A(String::from("hello")),
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(buffer, b"\x0a\x05\x68\x65\x6c\x6c\x6f");

            let mut decoder = Decoder::new(&buffer);
            let read = super::WrappedOneOf::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_b() {
            let test = super::WrappedOneOf {
                oneof: super::OneOf::B(123),
            };

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(buffer, b"\x10\x7b");

            let mut decoder = Decoder::new(&buffer);
            let read = super::WrappedOneOf::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn decode_last_variant_wins() {
            let buffer = b"\x0a\x06\x53\x74\x72\x69\x6e\x67\x10\x7b";

            let mut decoder = Decoder::new(&buffer[..]);
            let read = super::WrappedOneOf::decode_message(&mut decoder).unwrap();

            match read.oneof {
                super::OneOf::A(_) => panic!("should end in B"),
                super::OneOf::B(_) => (),
            }
        }

        #[test]
        fn decode_last_vavalue_wins() {
            let buffer = b"\x10\x00\x10\x7b";

            let mut decoder = Decoder::new(&buffer[..]);
            let read = super::WrappedOneOf::decode_message(&mut decoder).unwrap();

            match read.oneof {
                super::OneOf::A(_) => panic!("should end in B"),
                super::OneOf::B(b) => assert_eq!(b, 123),
            }
        }
    }

    mod unwrapped {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};

        #[test]
        fn encode_decode_a() {
            let test = super::OneOf::A(String::from("hello"));

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(buffer, b"\x0a\x05\x68\x65\x6c\x6c\x6f");

            let mut decoder = Decoder::new(&buffer);
            let read = super::OneOf::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_b() {
            let test = super::OneOf::B(123);

            let size_hint = test.message_size_hint();
            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            test.encode_message(&mut encoder);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);
            assert_eq!(buffer, b"\x10\x7b");

            let mut decoder = Decoder::new(&buffer);
            let read = super::OneOf::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn decode_last_variant_wins() {
            let buffer = b"\x0a\x06\x53\x74\x72\x69\x6e\x67\x10\x7b";

            let mut decoder = Decoder::new(&buffer[..]);
            let read = super::OneOf::decode_message(&mut decoder).unwrap();

            match read {
                super::OneOf::A(_) => panic!("should end in B"),
                super::OneOf::B(_) => (),
            }
        }

        #[test]
        fn decode_last_vavalue_wins() {
            let buffer = b"\x10\x00\x10\x7b";

            let mut decoder = Decoder::new(&buffer[..]);
            let read = super::OneOf::decode_message(&mut decoder).unwrap();

            match read {
                super::OneOf::A(_) => panic!("should end in B"),
                super::OneOf::B(b) => assert_eq!(b, 123),
            }
        }
    }
}
