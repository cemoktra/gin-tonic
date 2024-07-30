// not using the constants as testing against protobuf pal would get difficult then
#![allow(clippy::approx_constant)]
use gin_tonic_derive::{Enumeration, Message, OneOf};
use std::collections::HashMap;

mod primitives {
    mod required {
        use gin_tonic_derive::Message;

        #[derive(Debug, Message, PartialEq)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(tag = 1, proto = "int32")]
            int32: i32,
            #[gin(tag = 2, proto = "int64")]
            int64: i64,
            #[gin(tag = 3, proto = "uint32")]
            uint32: u32,
            #[gin(tag = 4, proto = "uint64")]
            uint64: u64,
            #[gin(tag = 5, proto = "sint32")]
            sint32: i32,
            #[gin(tag = 6, proto = "sint64")]
            sint64: i64,
            #[gin(tag = 7, proto = "fixed32")]
            fixed32: u32,
            #[gin(tag = 8, proto = "fixed64")]
            fixed64: u64,
            #[gin(tag = 9, proto = "sfixed32")]
            sfixed32: i32,
            #[gin(tag = 10, proto = "sfixed64")]
            sfixed64: i64,
            #[gin(tag = 11, proto = "string")]
            string: String,
            #[gin(tag = 12, proto = "bool")]
            boolean: bool,
            #[gin(tag = 13, proto = "float")]
            float: f32,
            #[gin(tag = 14, proto = "double")]
            double: f64,
        }

        #[test]
        fn encode_decode() {
            use gin_tonic_core::types::PbType;

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

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }
    }

    mod optional {
        use gin_tonic_derive::Message;

        #[derive(Debug, Message, PartialEq, Default)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(tag = 1, cardinality = "optional", proto = "int32")]
            int32: Option<i32>,
            #[gin(tag = 2, cardinality = "optional", proto = "int64")]
            int64: Option<i64>,
            #[gin(tag = 3, cardinality = "optional", proto = "uint32")]
            uint32: Option<u32>,
            #[gin(tag = 4, cardinality = "optional", proto = "uint64")]
            uint64: Option<u64>,
            #[gin(tag = 5, cardinality = "optional", proto = "sint32")]
            sint32: Option<i32>,
            #[gin(tag = 6, cardinality = "optional", proto = "sint64")]
            sint64: Option<i64>,
            #[gin(tag = 7, cardinality = "optional", proto = "fixed32")]
            fixed32: Option<u32>,
            #[gin(tag = 8, cardinality = "optional", proto = "fixed64")]
            fixed64: Option<u64>,
            #[gin(tag = 9, cardinality = "optional", proto = "sfixed32")]
            sfixed32: Option<i32>,
            #[gin(tag = 10, cardinality = "optional", proto = "sfixed64")]
            sfixed64: Option<i64>,
            #[gin(tag = 11, cardinality = "optional", proto = "string")]
            string: Option<String>,
            #[gin(tag = 12, cardinality = "optional", proto = "bool")]
            boolean: Option<bool>,
            #[gin(tag = 13, cardinality = "optional", proto = "float")]
            float: Option<f32>,
            #[gin(tag = 14, cardinality = "optional", proto = "double")]
            double: Option<f64>,
        }

        #[test]
        fn encode_decode_some() {
            use gin_tonic_core::types::PbType;

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

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_none() {
            use gin_tonic_core::types::PbType;

            let test = Test::default();

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }
    }

    mod repeated {
        use gin_tonic_derive::Message;

        #[derive(Debug, Message, PartialEq, Default)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(tag = 1, cardinality = "repeated", proto = "int32")]
            int32: Vec<i32>,
            #[gin(tag = 2, cardinality = "repeated", proto = "int64")]
            int64: Vec<i64>,
            #[gin(tag = 3, cardinality = "repeated", proto = "uint32")]
            uint32: Vec<u32>,
            #[gin(tag = 4, cardinality = "repeated", proto = "uint64")]
            uint64: Vec<u64>,
            #[gin(tag = 5, cardinality = "repeated", proto = "sint32")]
            sint32: Vec<i32>,
            #[gin(tag = 6, cardinality = "repeated", proto = "sint64")]
            sint64: Vec<i64>,
            #[gin(tag = 7, cardinality = "repeated", proto = "fixed32")]
            fixed32: Vec<u32>,
            #[gin(tag = 8, cardinality = "repeated", proto = "fixed64")]
            fixed64: Vec<u64>,
            #[gin(tag = 9, cardinality = "repeated", proto = "sfixed32")]
            sfixed32: Vec<i32>,
            #[gin(tag = 10, cardinality = "repeated", proto = "sfixed64")]
            sfixed64: Vec<i64>,
            #[gin(tag = 11, cardinality = "repeated", proto = "string")]
            string: Vec<String>,
            #[gin(tag = 12, cardinality = "repeated", proto = "bool")]
            boolean: Vec<bool>,
            #[gin(tag = 13, cardinality = "repeated", proto = "float")]
            float: Vec<f32>,
            #[gin(tag = 14, cardinality = "repeated", proto = "double")]
            double: Vec<f64>,
        }

        #[test]
        fn encode_decode_some() {
            use gin_tonic_core::types::PbType;

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

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_none() {
            use gin_tonic_core::types::PbType;

            let test = Test::default();

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }
    }
}

mod messages {
    use gin_tonic_derive::Message;

    #[derive(Debug, Message, PartialEq, Clone)]
    #[gin(root = "crate")]
    struct Nested {
        #[gin(tag = 1, proto = "int32")]
        int32: i32,
    }

    mod required {
        use gin_tonic_derive::Message;

        #[derive(Debug, Message, PartialEq)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(tag = 1, kind = "message")]
            nested_1: super::Nested,
            #[gin(tag = 2, kind = "message")]
            nested_2: super::Nested,
            #[gin(tag = 3, kind = "message")]
            nested_3: super::Nested,
            #[gin(tag = 4, kind = "message")]
            nested_4: super::Nested,
        }

        #[test]
        fn encode_decode() {
            use gin_tonic_core::types::PbType;

            let test = Test {
                nested_1: super::Nested { int32: 123 },
                nested_2: super::Nested { int32: 123 },
                nested_3: super::Nested { int32: 123 },
                nested_4: super::Nested { int32: 123 },
            };

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }
    }

    mod optional {
        use gin_tonic_derive::Message;

        #[derive(Debug, Message, PartialEq, Default)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(tag = 1, cardinality = "optional", kind = "message")]
            nested_1: Option<super::Nested>,
            #[gin(tag = 2, cardinality = "optional", kind = "message")]
            nested_2: Option<super::Nested>,
            #[gin(tag = 3, cardinality = "optional", kind = "message")]
            nested_3: Option<super::Nested>,
            #[gin(tag = 4, cardinality = "optional", kind = "message")]
            nested_4: Option<super::Nested>,
        }

        #[test]
        fn encode_decode_some() {
            use gin_tonic_core::types::PbType;

            let test = Test {
                nested_1: Some(super::Nested { int32: 123 }),
                nested_2: Some(super::Nested { int32: 123 }),
                nested_3: Some(super::Nested { int32: 123 }),
                nested_4: Some(super::Nested { int32: 123 }),
            };

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_none() {
            use gin_tonic_core::types::PbType;

            let test = Test::default();

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }
    }

    mod repeated {
        use gin_tonic_derive::Message;

        #[derive(Debug, Message, PartialEq, Default)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(tag = 1, cardinality = "repeated", kind = "message")]
            nested_1: Vec<super::Nested>,
            #[gin(tag = 2, cardinality = "repeated", kind = "message")]
            nested_2: Vec<super::Nested>,
            #[gin(tag = 3, cardinality = "repeated", kind = "message")]
            nested_3: Vec<super::Nested>,
            #[gin(tag = 4, cardinality = "repeated", kind = "message")]
            nested_4: Vec<super::Nested>,
        }

        #[test]
        fn encode_decode_some() {
            use gin_tonic_core::types::PbType;

            let test = Test {
                nested_1: vec![super::Nested { int32: 123 }, super::Nested { int32: 123 }],
                nested_2: vec![super::Nested { int32: 123 }],
                nested_3: vec![super::Nested { int32: 123 }],
                nested_4: vec![super::Nested { int32: 123 }],
            };

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_none() {
            use gin_tonic_core::types::PbType;

            let test = Test::default();

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }
    }
}

mod map {
    mod required {
        use std::collections::HashMap;

        use gin_tonic_derive::Message;

        #[derive(Debug, Message, PartialEq, Default)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(tag = 1, kind = "map", proto_key = "uint32", proto_value = "string")]
            map_1: HashMap<u32, String>,
            #[gin(tag = 2, kind = "map", proto_key = "string", proto_value = "uint32")]
            map_2: HashMap<String, u32>,
        }

        #[test]
        fn encode_decode() {
            use gin_tonic_core::types::PbType;

            let mut map_1 = HashMap::new();
            map_1.insert(10, "ten".into());
            map_1.insert(20, "twenty".into());

            let mut map_2 = HashMap::new();
            map_2.insert("ten".into(), 10);
            map_2.insert("twenty".into(), 20);

            let test = Test { map_1, map_2 };

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }
    }

    mod optional {
        use std::collections::HashMap;

        use gin_tonic_derive::Message;

        #[derive(Debug, Message, PartialEq, Default)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(
                tag = 1,
                cardinality = "optional",
                kind = "map",
                proto_key = "uint32",
                proto_value = "string"
            )]
            map_1: Option<HashMap<u32, String>>,
            #[gin(
                tag = 2,
                cardinality = "optional",
                kind = "map",
                proto_key = "string",
                proto_value = "uint32"
            )]
            map_2: Option<HashMap<String, u32>>,
        }

        #[test]
        fn encode_decode_some() {
            use gin_tonic_core::types::PbType;

            let mut map_1 = HashMap::new();
            map_1.insert(10, "ten".into());
            map_1.insert(20, "twenty".into());

            let mut map_2 = HashMap::new();
            map_2.insert("ten".into(), 10);
            map_2.insert("twenty".into(), 20);

            let test = Test {
                map_1: Some(map_1),
                map_2: Some(map_2),
            };

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_none() {
            use gin_tonic_core::types::PbType;

            let test = Test::default();

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }
    }
}

mod one_of {
    use gin_tonic_derive::OneOf;

    #[derive(Debug, PartialEq, OneOf)]
    #[gin(root = "crate")]
    enum Choice {
        #[gin(tag = 1, proto = "float")]
        Float(f32),
        #[gin(tag = 2, proto = "double")]
        Double(f64),
    }

    #[derive(Debug, PartialEq, OneOf)]
    #[gin(root = "crate")]
    enum Outcome {
        #[gin(tag = 3, proto = "string")]
        Success(String),
        #[gin(tag = 4, proto = "uint32")]
        ErrorCode(u32),
    }

    mod required {
        use gin_tonic_derive::Message;

        #[derive(Debug, Message, PartialEq)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(tag = 0, kind = "one_of")]
            choice: super::Choice,
            #[gin(tag = 0, kind = "one_of")]
            outcome: super::Outcome,
        }

        #[test]
        fn encode_decode() {
            use gin_tonic_core::types::PbType;

            let test = Test {
                choice: super::Choice::Double(3.14),
                outcome: super::Outcome::Success("3.14".into()),
            };

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }
    }

    mod optional {
        use gin_tonic_derive::Message;

        #[derive(Debug, Message, PartialEq, Default)]
        #[gin(root = "crate")]
        struct Test {
            #[gin(tag = 0, cardinality = "optional", kind = "one_of")]
            choice: Option<super::Choice>,
            #[gin(tag = 0, cardinality = "optional", kind = "one_of")]
            outcome: Option<super::Outcome>,
        }
        #[test]
        fn encode_decode_some() {
            use gin_tonic_core::types::PbType;

            let test = Test {
                choice: Some(super::Choice::Double(3.14)),
                outcome: Some(super::Outcome::Success("3.14".into())),
            };
            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert!(actual_size > 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }

        #[test]
        fn encode_decode_none() {
            use gin_tonic_core::types::PbType;

            let test = Test::default();

            let size_hint = test.size_hint();
            let mut buffer = bytes::BytesMut::with_capacity(size_hint);
            test.encode(&mut buffer);

            let actual_size = buffer.len();
            assert_eq!(actual_size, 0);
            assert_eq!(actual_size, size_hint);

            let read = Test::decode(&mut buffer).unwrap();

            assert_eq!(test, read)
        }
    }
}

mod non_primitve {
    use gin_tonic_derive::Message;

    #[derive(Debug, Message, PartialEq)]
    #[gin(root = "crate")]
    struct Test {
        #[gin(tag = 1)]
        required_ip: std::net::Ipv4Addr,
        #[gin(tag = 2, cardinality = "optional")]
        optional_ip: Option<std::net::Ipv4Addr>,
        #[gin(tag = 3, cardinality = "repeated")]
        repeated_ip: Vec<std::net::Ipv4Addr>,
    }

    #[test]
    fn encode_decode_some() {
        use gin_tonic_core::types::PbType;

        let test = Test {
            required_ip: std::net::Ipv4Addr::LOCALHOST,
            optional_ip: Some(std::net::Ipv4Addr::LOCALHOST),
            repeated_ip: vec![std::net::Ipv4Addr::LOCALHOST, std::net::Ipv4Addr::LOCALHOST],
        };
        let size_hint = test.size_hint();
        let mut buffer = bytes::BytesMut::with_capacity(size_hint);
        test.encode(&mut buffer);

        let actual_size = buffer.len();
        assert!(actual_size > 0);
        assert_eq!(actual_size, size_hint);

        let read = Test::decode(&mut buffer).unwrap();

        assert_eq!(test, read)
    }

    #[test]
    fn encode_decode_none() {
        use gin_tonic_core::types::PbType;

        let test = Test {
            required_ip: std::net::Ipv4Addr::LOCALHOST,
            optional_ip: None,
            repeated_ip: vec![],
        };
        let size_hint = test.size_hint();
        let mut buffer = bytes::BytesMut::with_capacity(size_hint);
        test.encode(&mut buffer);

        let actual_size = buffer.len();
        assert!(actual_size > 0);
        assert_eq!(actual_size, size_hint);

        let read = Test::decode(&mut buffer).unwrap();

        assert_eq!(test, read)
    }
}

#[derive(Debug, Message)]
#[gin(root = "crate")]
struct Test {
    #[gin(tag = 1)]
    ip: std::net::Ipv4Addr,
    #[gin(tag = 2, cardinality = "optional", proto = "uint32")]
    port: Option<u32>,
    #[gin(tag = 3, cardinality = "repeated", proto = "string")]
    protocols: Vec<String>,
    #[gin(tag = 4, kind = "message")]
    nested: Nested,
    #[gin(tag = 5)]
    logging: Logging,
    #[gin(tag = 0, kind = "one_of")]
    one_of: OneOf,
    #[gin(tag = 8, kind = "map", proto_key = "uint32", proto_value = "string")]
    map: HashMap<u32, String>,
}

#[derive(Debug, Message)]
#[gin(root = "crate")]
struct Nested {
    #[gin(tag = 1, proto = "int32")]
    number: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Enumeration)]
#[gin(root = "crate")]
enum Logging {
    #[gin(tag = 1)]
    Human,
    #[gin(tag = 2)]
    Json,
}

#[derive(Clone, Debug, Eq, PartialEq, OneOf)]
#[gin(root = "crate")]
enum OneOf {
    #[gin(tag = 6, proto = "int32")]
    Num(i32),
    #[gin(tag = 7, proto = "string")]
    Str(String),
}

#[test]
fn pb_serde() {
    use gin_tonic_core::types::PbType;

    let test = Test {
        ip: std::net::Ipv4Addr::LOCALHOST,
        port: None,
        protocols: vec![],
        nested: Nested { number: -1 },
        logging: Logging::Human,
        one_of: OneOf::Num(123),
        map: HashMap::new(),
    };

    let size_hint = test.size_hint();
    let mut buffer = bytes::BytesMut::with_capacity(size_hint);
    test.encode(&mut buffer);

    let actual_size = buffer.len();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let test = Test::decode(&mut buffer).unwrap();

    assert_eq!(test.ip, std::net::Ipv4Addr::LOCALHOST);
    assert!(test.port.is_none());
    assert!(test.protocols.is_empty());
    assert_eq!(test.nested.number, -1);
    assert_eq!(test.logging, Logging::Human);
    match test.one_of {
        OneOf::Num(n) => assert_eq!(n, 123),
        _ => panic!("wrong one_of"),
    }
    assert!(test.map.is_empty());

    // first round with optional field set to Some
    let mut map = HashMap::new();
    map.insert(10, String::from("ten"));
    map.insert(20, String::from("twenty"));
    let test = Test {
        ip: std::net::Ipv4Addr::LOCALHOST,
        port: Some(8080),
        protocols: vec![String::from("tcp"), String::from("udp")],
        nested: Nested { number: 1 },
        logging: Logging::Json,
        one_of: OneOf::Str(String::from("hello")),
        map,
    };

    let size_hint = test.size_hint();
    let mut buffer = bytes::BytesMut::with_capacity(size_hint);
    test.encode(&mut buffer);

    let actual_size = buffer.len();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let test = Test::decode(&mut buffer).unwrap();

    assert_eq!(test.ip, std::net::Ipv4Addr::LOCALHOST);
    assert_eq!(test.port, Some(8080));
    assert_eq!(test.protocols.len(), 2);
    assert_eq!(test.nested.number, 1);
    assert_eq!(test.logging, Logging::Json);
    match test.one_of {
        OneOf::Str(s) => assert_eq!(s, "hello"),
        _ => panic!("wrong one_of"),
    }
    assert_eq!(test.map.len(), 2);
}

#[derive(Debug, Message)]
#[gin(root = "crate")]
struct ResultMessage {
    #[gin(tag = 0, kind = "one_of")]
    result: ResultOneOf,
}

#[derive(Clone, Debug, Eq, PartialEq, gin_tonic_derive::OneOf)]
#[gin(root = "crate")]
enum ResultOneOf {
    #[gin(tag = 1, proto = "int32")]
    Success(i32),
    #[gin(tag = 2, proto = "int32")]
    Error(i32),
}

// this is on protobuf layer identical to ResultMessage and ResultOneOn but simplify the Rust layer
#[derive(Clone, Debug, Eq, PartialEq, gin_tonic_derive::Message)]
#[gin(root = "crate")]
enum UnwrappedResultOneOf {
    #[gin(tag = 1, proto = "int32")]
    Success(i32),
    #[gin(tag = 2, proto = "int32")]
    Error(i32),
}

#[test]
fn one_of_unwrapping() {
    use gin_tonic_core::types::PbType;

    // wrapped to unwrapped
    let test = ResultMessage {
        result: ResultOneOf::Success(1),
    };

    let size_hint = test.size_hint();
    let mut buffer = bytes::BytesMut::with_capacity(size_hint);

    test.encode(&mut buffer);

    let actual_size = buffer.len();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let unwrapped = UnwrappedResultOneOf::decode(&mut buffer.clone()).unwrap();
    assert_eq!(unwrapped, UnwrappedResultOneOf::Success(1));

    let wrapped = ResultMessage::decode(&mut buffer).unwrap();
    assert_eq!(wrapped.result, ResultOneOf::Success(1));

    // unwrapped to wrapped
    let test = UnwrappedResultOneOf::Success(1);

    let size_hint = test.size_hint();
    let mut buffer = bytes::BytesMut::with_capacity(size_hint);

    test.encode(&mut buffer);

    let actual_size = buffer.len();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let unwrapped = UnwrappedResultOneOf::decode(&mut buffer.clone()).unwrap();
    assert_eq!(unwrapped, UnwrappedResultOneOf::Success(1));

    let wrapped = ResultMessage::decode(&mut buffer).unwrap();
    assert_eq!(wrapped.result, ResultOneOf::Success(1));
}
