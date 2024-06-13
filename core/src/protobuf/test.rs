use crate::protobuf::reader::TagReader;
use crate::protobuf::{Error, FromWire, IntoWire, ProtocolBuffer, WireTypeView};
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

struct Test {
    ip: std::net::Ipv4Addr,
    port: Option<u32>,
}

impl ProtocolBuffer for Test {
    fn serialize(&self, writer: &mut impl Write) -> Result<usize, Error> {
        let mut written = 0;

        let wire_type = self.ip.into_wire();
        written += wire_type.serialize(1, writer)?;

        if let Some(port) = self.port {
            let wire_type = port.into_wire();
            written += wire_type.serialize(2, writer)?;
        }

        Ok(written)
    }

    fn deserialize(buffer: &[u8]) -> Result<Self, Error> {
        let mut reader = TagReader::new(buffer);
        let mut field_map = HashMap::new();

        while let Some(tag) = reader.next() {
            let (field_number, wire_type) = tag.into_parts();
            field_map.insert(field_number, wire_type);
        }

        let ip = field_map.remove(&1).ok_or(Error::MissingField(1))?;
        let ip = std::net::Ipv4Addr::from_wire(ip)?;

        let port = field_map
            .remove(&2)
            .map(|wire| u32::from_wire(wire))
            .transpose()?;

        Ok(Self { ip, port })
    }

    fn size_hint(&self) -> usize {
        // each tag can take up to 10 bytes + the actual data
        0 + self.ip.size_hint()
            + 10
            + self
                .port
                .map(|port| port.size_hint() + 10)
                .unwrap_or_default()
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

        fn size_hint(&self) -> usize {
            let ip: u32 = (*self).into();
            ip.required_space()
        }
    }
}

#[test]
fn basic_serde() {
    // first round with optional field set to None
    let test = Test {
        ip: std::net::Ipv4Addr::LOCALHOST,
        port: None,
    };
    let size_hint = test.size_hint();
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();
    assert!(actual_size > 0);
    assert!(actual_size <= size_hint);

    let test = Test::deserialize(&buffer).unwrap();

    assert_eq!(test.ip, std::net::Ipv4Addr::LOCALHOST);
    assert!(test.port.is_none());

    // first round with optional field set to Some
    let test = Test {
        ip: std::net::Ipv4Addr::LOCALHOST,
        port: Some(8080),
    };
    let size_hint = test.size_hint();
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();
    assert!(actual_size > 0);
    assert!(actual_size <= size_hint);

    let test = Test::deserialize(&buffer).unwrap();

    assert_eq!(test.ip, std::net::Ipv4Addr::LOCALHOST);
    assert_eq!(test.port, Some(8080));
}
