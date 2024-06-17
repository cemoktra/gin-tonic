use gin_tonic_core::{
    export::VarInt,
    protobuf::{Error, Message},
};
use std::collections::HashMap;

#[derive(Debug, gin_tonic_core::Message)]
struct Test {
    #[gin(tag = 1)]
    ip: std::net::Ipv4Addr,
    #[gin(tag = 2, cardinality = "optional")]
    port: Option<u32>,
    #[gin(tag = 3, cardinality = "repeated")]
    protocols: Vec<String>,
    #[gin(tag = 4, kind = "message")]
    nested: Nested,
    #[gin(tag = 5)]
    logging: Logging,
    #[gin(tag = 0, kind = "one_of")]
    one_of: OneOf,
    #[gin(tag = 8, kind = "map")]
    map: HashMap<u32, String>,
}

#[derive(Debug, gin_tonic_core::Message)]
struct Nested {
    #[gin(tag = 1)]
    number: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, gin_tonic_core::Enumeration)]
enum Logging {
    #[gin(tag = 1)]
    Human,
    #[gin(tag = 2)]
    Json,
}

#[derive(Clone, Debug, Eq, PartialEq, gin_tonic_core::OneOf)]
enum OneOf {
    #[gin(tag = 6)]
    Num(i32),
    #[gin(tag = 7)]
    Str(String),
}

#[test]
fn pb_serde() {
    let test = Test {
        ip: std::net::Ipv4Addr::LOCALHOST,
        port: None,
        protocols: vec![],
        nested: Nested { number: -1 },
        logging: Logging::Human,
        one_of: OneOf::Num(123),
        map: HashMap::new(),
    };

    let size_hint = Message::size_hint(&test);
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let test = Test::deserialize(&buffer).unwrap();

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
        nested: Nested { number: -1 },
        logging: Logging::Json,
        one_of: OneOf::Str(String::from("hello")),
        map,
    };
    let size_hint = Message::size_hint(&test);
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();

    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let test = Test::deserialize(&buffer).unwrap();

    assert_eq!(test.ip, std::net::Ipv4Addr::LOCALHOST);
    assert_eq!(test.port, Some(8080));
    assert_eq!(test.protocols.len(), 2);
    assert_eq!(test.nested.number, -1);
    assert_eq!(test.logging, Logging::Json);
    match test.one_of {
        OneOf::Str(s) => assert_eq!(s, "hello"),
        _ => panic!("wrong one_of"),
    }
    assert_eq!(test.map.len(), 2);
}

#[derive(Debug, gin_tonic_core::Message)]
struct ResultMessage {
    #[gin(tag = 0, kind = "one_of")]
    result: ResultOneOf,
}

#[derive(Clone, Debug, Eq, PartialEq, gin_tonic_core::OneOf)]
enum ResultOneOf {
    #[gin(tag = 1)]
    Success(i32),
    #[gin(tag = 2)]
    Error(i32),
}

// this is on protobuf layer identical to ResultMessage and ResultOneOn but simplify the Rust layer
#[derive(Clone, Debug, Eq, PartialEq, gin_tonic_core::Message)]
enum UnwrappedResultOneOf {
    #[gin(tag = 1)]
    Success(i32),
    #[gin(tag = 2)]
    Error(i32),
}

#[test]
fn one_of_unwrapping() {
    // wrapped to unwrapped
    let test = ResultMessage {
        result: ResultOneOf::Success(1),
    };

    let size_hint = Message::size_hint(&test);
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let unwrapped = UnwrappedResultOneOf::deserialize(&buffer).unwrap();
    assert_eq!(unwrapped, UnwrappedResultOneOf::Success(1));

    let wrapped = ResultMessage::deserialize(&buffer).unwrap();
    assert_eq!(wrapped.result, ResultOneOf::Success(1));

    // unwrapped to wrapped
    let test = UnwrappedResultOneOf::Success(1);

    let size_hint = Message::size_hint(&test);
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let unwrapped = UnwrappedResultOneOf::deserialize(&buffer).unwrap();
    assert_eq!(unwrapped, UnwrappedResultOneOf::Success(1));

    let wrapped = ResultMessage::deserialize(&buffer).unwrap();
    assert_eq!(wrapped.result, ResultOneOf::Success(1));
}
