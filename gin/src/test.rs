use gin_tonic_derive::{Enumeration, Message};
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
    mod repeated {}
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
    // #[gin(tag = 0, kind = "one_of")]
    // one_of: OneOf,
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

// #[derive(Clone, Debug, Eq, PartialEq, gin_tonic_derive::OneOf)]
// #[gin(root = "crate")]
// enum OneOf {
//     #[gin(tag = 6)]
//     Num(i32),
//     #[gin(tag = 7)]
//     Str(String),
// }

#[test]
fn pb_serde() {
    use gin_tonic_core::types::PbType;

    let test = Test {
        ip: std::net::Ipv4Addr::LOCALHOST,
        port: None,
        protocols: vec![],
        nested: Nested { number: -1 },
        logging: Logging::Human,
        // one_of: OneOf::Num(123),
        map: HashMap::new(),
    };

    println!("{}", test.ip.to_bits());

    let size_hint = test.size_hint();
    let mut buffer = bytes::BytesMut::with_capacity(size_hint);
    test.encode(&mut buffer);

    for b in &buffer {
        print!("{b:02x}");
    }
    println!();

    let actual_size = buffer.len();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let test = Test::decode(&mut buffer).unwrap();

    assert_eq!(test.ip, std::net::Ipv4Addr::LOCALHOST);
    assert!(test.port.is_none());
    assert!(test.protocols.is_empty());
    assert_eq!(test.nested.number, -1);
    assert_eq!(test.logging, Logging::Human);
    // match test.one_of {
    //     OneOf::Num(n) => assert_eq!(n, 123),
    //     _ => panic!("wrong one_of"),
    // }
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
        //     one_of: OneOf::Str(String::from("hello")),
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
    // match test.one_of {
    //     OneOf::Str(s) => assert_eq!(s, "hello"),
    //     _ => panic!("wrong one_of"),
    // }
    assert_eq!(test.map.len(), 2);
}

// #[derive(Debug, Message)]
// #[gin(root = "crate")]
// struct ResultMessage {
//     #[gin(tag = 0, kind = "one_of")]
//     result: ResultOneOf,
// }

// #[derive(Clone, Debug, Eq, PartialEq, gin_tonic_derive::OneOf)]
// #[gin(root = "crate")]
// enum ResultOneOf {
//     #[gin(tag = 1)]
//     Success(i32),
//     #[gin(tag = 2)]
//     Error(i32),
// }

// // this is on protobuf layer identical to ResultMessage and ResultOneOn but simplify the Rust layer
// #[derive(Clone, Debug, Eq, PartialEq, gin_tonic_derive::OneOf)]
// #[gin(root = "crate")]
// enum UnwrappedResultOneOf {
//     #[gin(tag = 1)]
//     Success(i32),
//     #[gin(tag = 2)]
//     Error(i32),
// }

// #[test]
// fn one_of_unwrapping() {
//     use gin_tonic_core::Message;

//     // wrapped to unwrapped
//     let test = ResultMessage {
//         result: ResultOneOf::Success(1),
//     };

//     let size_hint = gin_tonic_core::Message::size_hint(&test);
//     let mut buffer = bytes::BytesMut::with_capacity(size_hint);

//     test.serialize(&mut buffer);

//     let actual_size = buffer.len();
//     assert!(actual_size > 0);
//     assert_eq!(actual_size, size_hint);

//     let (unwrapped, _) = UnwrappedResultOneOf::deserialize(&buffer).unwrap();
//     assert_eq!(unwrapped, UnwrappedResultOneOf::Success(1));

//     let (wrapped, _) = ResultMessage::deserialize(&buffer).unwrap();
//     assert_eq!(wrapped.result, ResultOneOf::Success(1));

//     // unwrapped to wrapped
//     let test = UnwrappedResultOneOf::Success(1);

//     let size_hint = gin_tonic_core::Message::size_hint(&test);
//     let mut buffer = bytes::BytesMut::with_capacity(size_hint);

//     test.serialize(&mut buffer);
//     let actual_size = buffer.len();
//     assert!(actual_size > 0);
//     assert_eq!(actual_size, size_hint);

//     let (unwrapped, _) = UnwrappedResultOneOf::deserialize(&buffer).unwrap();
//     assert_eq!(unwrapped, UnwrappedResultOneOf::Success(1));

//     let (wrapped, _) = ResultMessage::deserialize(&buffer).unwrap();
//     assert_eq!(wrapped.result, ResultOneOf::Success(1));
// }
