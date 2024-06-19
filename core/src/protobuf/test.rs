use crate::protobuf::reader::TagReader;
use crate::protobuf::wire::WireType;
use crate::protobuf::{Error, FromWire, IntoWire, Message, WireTypeView};
use integer_encoding::VarInt;
use std::collections::HashMap;
use std::io::Write;

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

struct Nested {
    // 1
    number: i32,
}

struct Test {
    // 1
    ip: std::net::Ipv4Addr,
    // 2
    port: Option<u32>,
    // 3
    protocols: Vec<String>,
    // 4
    nested: Nested,
    // 5
    logging: Logging,
    // 6 + 7
    oneof: Oneof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Logging {
    Human = 1,
    Json = 2,
}

enum Oneof {
    Num(i32),
    Str(String),
}

impl Message for Oneof {
    fn serialize(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;

        match self {
            Oneof::Num(n) => {
                let wire_type = n.into_wire();
                written += wire_type.serialize(6, writer)?;
            }
            Oneof::Str(s) => {
                let wire_type = s.into_wire();
                written += wire_type.serialize(7, writer)?;
            }
        }

        Ok(written)
    }

    fn deserialize_tags(tag_map: &mut HashMap<u32, Vec<WireTypeView>>) -> Result<Self, Error> {
        if let Some(types) = tag_map.remove(&6) {
            let n = i32::from_wire(types.into_iter().nth(0).ok_or(Error::InvalidOneOf)?)?;
            Ok(Self::Num(n))
        } else if let Some(types) = tag_map.remove(&7) {
            let s = String::from_wire(types.into_iter().nth(0).ok_or(Error::InvalidOneOf)?)?;
            Ok(Self::Str(s))
        } else {
            Err(Error::InvalidOneOf)
        }
    }

    fn size_hint(&self) -> usize {
        match self {
            Oneof::Num(n) => n.size_hint(6),
            Oneof::Str(s) => s.size_hint(7),
        }
    }
}

// this will be implemented via derive macro
impl IntoWire for Logging {
    fn into_wire(self) -> WireType {
        match self {
            Logging::Human => 1u32.into_wire(),
            Logging::Json => 2u32.into_wire(),
        }
    }

    fn size_hint(&self, tag: u32) -> usize {
        tag.required_space()
            + match self {
                Logging::Human => 1u32.required_space(),
                Logging::Json => 2u32.required_space(),
            }
    }
}

// this will be implemented via derive macro
impl FromWire for Logging {
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let n = u32::from_wire(wire)?;
        match n {
            1 => Ok(Self::Human),
            2 => Ok(Self::Json),
            n => Err(Error::UnknownEnumVariant(n)),
        }
    }
}

// this is a test implementation and should be replaced with a derive macro later
impl Message for Test {
    fn serialize(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;

        // normal serialization
        let wire_type = self.ip.into_wire();
        written += wire_type.serialize(1, writer)?;

        // optional serialization
        if let Some(port) = self.port {
            let wire_type = port.into_wire();
            written += wire_type.serialize(2, writer)?;
        }

        // vector serialization
        for item in self.protocols {
            let wire_type = item.into_wire();
            written += wire_type.serialize(3, writer)?;
        }

        // nested serialization
        let wire_type = self.nested.into_wire();
        written += wire_type.serialize(4, writer)?;

        // enum serialization
        let wire_type = self.logging.into_wire();
        written += wire_type.serialize(5, writer)?;

        // oneof serialization
        written += self.oneof.serialize(writer)?;

        Ok(written)
    }

    fn deserialize_tags(tag_map: &mut HashMap<u32, Vec<WireTypeView>>) -> Result<Self, Error> {
        // normal deserialization
        let ip = tag_map
            .remove(&1)
            .ok_or(Error::MissingField(1))?
            .into_iter()
            .nth(0)
            .ok_or(Error::MissingField(1))?;
        let ip = std::net::Ipv4Addr::from_wire(ip)?;

        // optional deserialization
        let port = tag_map
            .remove(&2)
            .map(|wire| u32::from_wire(wire.into_iter().nth(0).ok_or(Error::MissingField(2))?))
            .transpose()?;

        // vector deserialization
        let mut protocols = vec![];
        if let Some(wires) = tag_map.remove(&3) {
            for wire in wires {
                protocols.push(String::from_wire(wire)?)
            }
        }

        // nested deserialization
        let nested = tag_map
            .remove(&4)
            .ok_or(Error::MissingField(4))?
            .into_iter()
            .nth(0)
            .ok_or(Error::MissingField(4))?;
        let nested = Nested::from_wire(nested)?;

        // enum deserialization
        let logging = tag_map
            .remove(&5)
            .ok_or(Error::MissingField(5))?
            .into_iter()
            .nth(0)
            .ok_or(Error::MissingField(5))?;
        let logging = Logging::from_wire(logging)?;

        // oneof deserialization
        let oneof = Oneof::deserialize_tags(tag_map)?;

        Ok(Self {
            ip,
            port,
            protocols,
            nested,
            logging,
            oneof,
        })
    }

    fn size_hint(&self) -> usize {
        let ip_size = self.ip.size_hint(1);
        let port_size = self.port.map(|port| port.size_hint(2)).unwrap_or_default();
        let protocols_size: usize = self.protocols.iter().map(|item| item.size_hint(3)).sum();
        let nested_size = IntoWire::size_hint(&self.nested, 4);
        let nested_size = 4u32.required_space() + nested_size.required_space() + nested_size;
        let logging_size = self.logging.size_hint(5);
        let oneof_size = Message::size_hint(&self.oneof);

        ip_size + port_size + protocols_size + nested_size + logging_size + oneof_size
    }
}

// this is a test implementation and should be replaced with a derive macro later
impl Message for Nested {
    fn serialize(self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;

        let wire_type = self.number.into_wire();
        written += wire_type.serialize(1, writer)?;

        Ok(written)
    }

    fn deserialize_tags(tag_map: &mut HashMap<u32, Vec<WireTypeView>>) -> Result<Self, Error> {
        let number = tag_map
            .remove(&1)
            .ok_or(Error::MissingField(1))?
            .into_iter()
            .nth(0)
            .ok_or(Error::MissingField(1))?;
        let number = i32::from_wire(number)?;

        Ok(Self { number })
    }

    fn size_hint(&self) -> usize {
        let number_size = self.number.size_hint(1);

        number_size
    }
}

mod wire_impl {
    use crate::protobuf::wire::{WireType, WireTypeView};
    use crate::protobuf::{Error, FromWire, IntoWire};
    use integer_encoding::VarInt;

    impl FromWire for std::net::Ipv4Addr {
        fn from_wire(wire: WireTypeView) -> Result<Self, Error> {
            let ip = match wire {
                WireTypeView::VarInt(data) => {
                    let (ip, _) = u32::decode_var(data).ok_or(Error::InvalidVarInt)?;
                    ip
                }
                WireTypeView::FixedI32(data) => {
                    let array: [u8; 4] = data.try_into().expect("I32 is always 4 bytes");
                    u32::from_be_bytes(array)
                }
                _ => return Err(Error::UnexpectedWireType),
            };

            Ok(std::net::Ipv4Addr::from(ip))
        }
    }

    impl IntoWire for std::net::Ipv4Addr {
        fn into_wire(self) -> WireType {
            let mut data = [0u8; 10];
            let ip: u32 = self.into();
            let size = ip.encode_var(&mut data);
            WireType::VarInt(data, size)
        }

        fn size_hint(&self, tag: u32) -> usize {
            let ip: u32 = (*self).into();
            ip.required_space() + tag.required_space()
        }
    }
}

#[test]
fn basic_serde() {
    // first round with optional field set to None
    let test = Test {
        ip: std::net::Ipv4Addr::LOCALHOST,
        port: None,
        protocols: vec![],
        nested: Nested { number: -1 },
        logging: Logging::Human,
        oneof: Oneof::Num(123),
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
    match test.oneof {
        Oneof::Num(n) => assert_eq!(n, 123),
        _ => panic!("wrong oneof"),
    }

    // first round with optional field set to Some
    let test = Test {
        ip: std::net::Ipv4Addr::LOCALHOST,
        port: Some(8080),
        protocols: vec![String::from("tcp"), String::from("udp")],
        nested: Nested { number: -1 },
        logging: Logging::Json,
        oneof: Oneof::Str(String::from("hello")),
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
    match test.oneof {
        Oneof::Str(s) => assert_eq!(s, "hello"),
        _ => panic!("wrong oneof"),
    }
}
