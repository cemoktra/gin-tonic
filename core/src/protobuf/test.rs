use crate::protobuf::{reader::TagReader, WireTypeView};
use gin_tonic_derive::{Enumeration, Message, OneOf};
use integer_encoding::VarInt;
use std::collections::HashMap;

#[test]
fn proto3_compliance() {
    // https://github.com/protocolbuffers/protoscope/blob/main/testdata/proto3.pb
    let buffer = std::fs::read("proto3.pb").unwrap();
    let mut reader = TagReader::new(&buffer);

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 31);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(&data[..], [0x65, 0xad, 0x02]);
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 32);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(&data[..], [0xca, 0x01, 0xae, 0x02]);
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 33);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(&data[..], [0xcb, 0x01, 0xaf, 0x02]);
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 34);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(&data[..], [0xcc, 0x01, 0xb0, 0x02]);
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 35);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(&data[..], [0x9a, 0x03, 0xe2, 0x04]);
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 36);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        let mut sub_reader = TagReader::new(data);

        let tag = sub_reader.next().unwrap();
        assert_eq!(tag.field_number(), 51);
        assert_eq!(*tag.wire_type(), WireTypeView::EGroup);

        let tag = sub_reader.next().unwrap();
        assert_eq!(tag.field_number(), 76);
        assert_eq!(*tag.wire_type(), WireTypeView::EGroup);

        assert!(sub_reader.next().is_none());
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 37);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(&data[..], [0xcf, 0x00, 0x00, 0x00, 0x33, 0x01, 0x00, 0x00]);
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 38);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(
            &data[..],
            [
                0xd0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x01, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00
            ]
        );
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 39);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(&data[..], [0xd1, 0x00, 0x00, 0x00, 0x35, 0x01, 0x00, 0x00]);
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 40);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(
            &data[..],
            [
                0xd2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x01, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00
            ]
        );
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 41);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(&data[..], [0x00, 0x00, 0x53, 0x43, 0x00, 0x80, 0x9b, 0x43]);
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 42);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(
            &data[..],
            [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x6a, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80,
                0x73, 0x40
            ]
        );
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 43);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(&data[..], [0x01, 0x00]);
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 44);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(String::from_utf8(data.to_vec()).unwrap(), "215");
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 44);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(String::from_utf8(data.to_vec()).unwrap(), "315");
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 45);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(String::from_utf8(data.to_vec()).unwrap(), "216");
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 45);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(String::from_utf8(data.to_vec()).unwrap(), "316");
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 48);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        let mut sub_reader = TagReader::new(data);

        let tag = sub_reader.next().unwrap();
        assert_eq!(tag.field_number(), 1);
        if let WireTypeView::VarInt(data) = tag.wire_type() {
            let (data, _) = u32::decode_var(data).unwrap();
            assert_eq!(218, data);
        }

        assert!(sub_reader.next().is_none());
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 48);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        let mut sub_reader = TagReader::new(data);

        let tag = sub_reader.next().unwrap();
        assert_eq!(tag.field_number(), 1);
        if let WireTypeView::VarInt(data) = tag.wire_type() {
            let (data, _) = u32::decode_var(data).unwrap();
            assert_eq!(318, data);
        }

        assert!(sub_reader.next().is_none());
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 49);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        let mut sub_reader = TagReader::new(data);

        let tag = sub_reader.next().unwrap();
        assert_eq!(tag.field_number(), 1);
        if let WireTypeView::VarInt(data) = tag.wire_type() {
            let (data, _) = u32::decode_var(data).unwrap();
            assert_eq!(219, data);
        }

        assert!(sub_reader.next().is_none());
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 49);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        let mut sub_reader = TagReader::new(data);

        let tag = sub_reader.next().unwrap();
        assert_eq!(tag.field_number(), 1);
        if let WireTypeView::VarInt(data) = tag.wire_type() {
            let (data, _) = u32::decode_var(data).unwrap();
            assert_eq!(319, data);
        }

        assert!(sub_reader.next().is_none());
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 50);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        let mut sub_reader = TagReader::new(data);

        let tag = sub_reader.next().unwrap();
        assert_eq!(tag.field_number(), 1);
        if let WireTypeView::VarInt(data) = tag.wire_type() {
            let (data, _) = u32::decode_var(data).unwrap();
            assert_eq!(220, data);
        }

        assert!(sub_reader.next().is_none());
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 50);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        let mut sub_reader = TagReader::new(data);

        let tag = sub_reader.next().unwrap();
        assert_eq!(tag.field_number(), 1);
        if let WireTypeView::VarInt(data) = tag.wire_type() {
            let (data, _) = u32::decode_var(data).unwrap();
            assert_eq!(320, data);
        }

        assert!(sub_reader.next().is_none());
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 51);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(&data[..], [0x02, 0x03]);
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 52);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(&data[..], [0x05, 0x06]);
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 54);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(String::from_utf8(data.to_vec()).unwrap(), "224");
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 54);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(String::from_utf8(data.to_vec()).unwrap(), "324");
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 55);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(String::from_utf8(data.to_vec()).unwrap(), "225");
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 55);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(String::from_utf8(data.to_vec()).unwrap(), "325");
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 57);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        let mut sub_reader = TagReader::new(data);

        let tag = sub_reader.next().unwrap();
        assert_eq!(tag.field_number(), 1);
        if let WireTypeView::VarInt(data) = tag.wire_type() {
            let (data, _) = u32::decode_var(data).unwrap();
            assert_eq!(227, data);
        }

        assert!(sub_reader.next().is_none());
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 57);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        let mut sub_reader = TagReader::new(data);

        let tag = sub_reader.next().unwrap();
        assert_eq!(tag.field_number(), 1);
        if let WireTypeView::VarInt(data) = tag.wire_type() {
            let (data, _) = u32::decode_var(data).unwrap();
            assert_eq!(327, data);
        }

        assert!(sub_reader.next().is_none());
    }

    let tag = reader.next().unwrap();
    assert_eq!(tag.field_number(), 114);
    if let WireTypeView::LengthEncoded(data) = tag.wire_type() {
        assert_eq!(String::from_utf8(data.to_vec()).unwrap(), "604");
    }

    assert!(reader.next().is_none());
}

#[derive(Debug, Message)]
#[gin(root = "crate")]
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

#[derive(Debug, Message)]
#[gin(root = "crate")]
struct Nested {
    #[gin(tag = 1)]
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
    #[gin(tag = 6)]
    Num(i32),
    #[gin(tag = 7)]
    Str(String),
}

#[test]
fn pb_serde() {
    use crate::protobuf::Message;

    let test = Test {
        ip: std::net::Ipv4Addr::LOCALHOST,
        port: None,
        protocols: vec![],
        nested: Nested { number: -1 },
        logging: Logging::Human,
        one_of: OneOf::Num(123),
        map: HashMap::new(),
    };

    let size_hint = crate::protobuf::Message::size_hint(&test);
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let (test, _) = Test::deserialize(&buffer).unwrap();

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
    let size_hint = crate::protobuf::Message::size_hint(&test);
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();

    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let (test, _) = Test::deserialize(&buffer).unwrap();

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

#[derive(Debug, Message)]
#[gin(root = "crate")]
struct ResultMessage {
    #[gin(tag = 0, kind = "one_of")]
    result: ResultOneOf,
}

#[derive(Clone, Debug, Eq, PartialEq, OneOf)]
#[gin(root = "crate")]
enum ResultOneOf {
    #[gin(tag = 1)]
    Success(i32),
    #[gin(tag = 2)]
    Error(i32),
}

// this is on protobuf layer identical to ResultMessage and ResultOneOn but simplify the Rust layer
#[derive(Clone, Debug, Eq, PartialEq, Message)]
#[gin(root = "crate")]
enum UnwrappedResultOneOf {
    #[gin(tag = 1)]
    Success(i32),
    #[gin(tag = 2)]
    Error(i32),
}

#[test]
fn one_of_unwrapping() {
    use crate::protobuf::Message;

    // wrapped to unwrapped
    let test = ResultMessage {
        result: ResultOneOf::Success(1),
    };

    let size_hint = crate::protobuf::Message::size_hint(&test);
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let (unwrapped, _) = UnwrappedResultOneOf::deserialize(&buffer).unwrap();
    assert_eq!(unwrapped, UnwrappedResultOneOf::Success(1));

    let (wrapped, _) = ResultMessage::deserialize(&buffer).unwrap();
    assert_eq!(wrapped.result, ResultOneOf::Success(1));

    // unwrapped to wrapped
    let test = UnwrappedResultOneOf::Success(1);

    let size_hint = crate::protobuf::Message::size_hint(&test);
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let (unwrapped, _) = UnwrappedResultOneOf::deserialize(&buffer).unwrap();
    assert_eq!(unwrapped, UnwrappedResultOneOf::Success(1));

    let (wrapped, _) = ResultMessage::deserialize(&buffer).unwrap();
    assert_eq!(wrapped.result, ResultOneOf::Success(1));
}
