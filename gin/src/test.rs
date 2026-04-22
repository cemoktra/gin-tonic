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

        #[test]
        #[allow(clippy::approx_constant)]
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

        #[test]
        #[allow(clippy::approx_constant)]
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

        #[test]
        #[allow(clippy::approx_constant)]
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
        use std::collections::HashMap;

        #[derive(Default, Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1, key_scalar = "uint32")]
            map_1: HashMap<u32, String>,
            #[gin(id = 2, value_scalar = "uint32")]
            map_2: HashMap<String, u32>,
        }

        #[test]
        fn encode_decode_non_empty() {
            let mut map_1 = HashMap::new();
            map_1.insert(10, "ten".into());
            map_1.insert(20, "twenty".into());

            let mut map_2 = HashMap::new();
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

            let mut decoder = Decoder::new(&buffer);
            let read = Test::decode_message(&mut decoder).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_empty() {
            let map_1 = HashMap::new();
            let map_2 = HashMap::new();

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

    mod required {
        use gin_tonic_core::{Message, decoder::Decoder, encoder::Encoder};

        #[derive(Debug, PartialEq, gin_tonic_derive::Message)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(id = 1, scalar = "uint32")]
            e: super::TestEnum,
        }

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

    #[derive(Debug, PartialEq, gin_tonic_derive::Message)]
    #[gin(root = "crate")]
    struct WrappedOneOf {
        #[gin(id = 1, oneof)]
        oneof: OneOf,
    }

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
