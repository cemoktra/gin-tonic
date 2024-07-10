use crate::{FromWire, IntoWire, Tag, TagReader, WireType, WireTypeView};
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

#[test]
fn wire_type_bool() {
    let value = true;

    let wire = value.into_wire();
    assert_eq!(wire.size_hint(1), 2);
    assert_eq!(value.size_hint(1), wire.size_hint(1));

    let wire_value = bool::from_wire(wire.as_view()).unwrap();
    assert_eq!(value, wire_value);

    let mut buffer = bytes::BytesMut::with_capacity(2);
    wire.serialize(1, &mut buffer);
    assert_eq!(buffer.len(), wire.size_hint(1));

    let (tag, wire) = Tag::deserialize(&buffer).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    let wire_value = bool::from_wire(wire).unwrap();
    assert_eq!(value, wire_value);
}

#[test]
fn wire_type_u32() {
    let value = 1234u32;

    let wire = value.into_wire();
    assert_eq!(wire.size_hint(1), 3);
    assert_eq!(value.size_hint(1), wire.size_hint(1));
    let wire_value = u32::from_wire(wire.as_view()).unwrap();
    assert_eq!(value, wire_value);

    let mut buffer = bytes::BytesMut::with_capacity(3);
    wire.serialize(1, &mut buffer);
    assert_eq!(buffer.len(), wire.size_hint(1));

    let (tag, wire) = Tag::deserialize(&buffer[0..]).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    let wire_value = u32::from_wire(wire).unwrap();
    assert_eq!(value, wire_value);
}

#[test]
fn wire_type_i32() {
    let value = -1234i32;

    let wire = value.into_wire();
    assert_eq!(wire.size_hint(1), 3);
    assert_eq!(value.size_hint(1), wire.size_hint(1));
    let wire_value = i32::from_wire(wire.as_view()).unwrap();
    assert_eq!(value, wire_value);

    let mut buffer = bytes::BytesMut::with_capacity(3);
    wire.serialize(1, &mut buffer);
    assert_eq!(buffer.len(), wire.size_hint(1));

    let (tag, wire) = Tag::deserialize(&buffer).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    let wire_value = i32::from_wire(wire).unwrap();
    assert_eq!(value, wire_value);
}

#[test]
fn wire_type_u64() {
    let value = 123456u64;

    let wire = value.into_wire();
    assert_eq!(wire.size_hint(1), 4);
    assert_eq!(value.size_hint(1), wire.size_hint(1));
    let wire_value = u64::from_wire(wire.as_view()).unwrap();
    assert_eq!(value, wire_value);

    let mut buffer = bytes::BytesMut::with_capacity(4);
    wire.serialize(1, &mut buffer);
    assert_eq!(buffer.len(), wire.size_hint(1));

    let (tag, wire) = Tag::deserialize(&buffer).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    let wire_value = u64::from_wire(wire).unwrap();
    assert_eq!(value, wire_value);
}

#[test]
fn wire_type_i64() {
    let value = -123456i64;

    let wire = value.into_wire();
    assert_eq!(wire.size_hint(1), 4);
    assert_eq!(value.size_hint(1), wire.size_hint(1));
    let wire_value = i64::from_wire(wire.as_view()).unwrap();
    assert_eq!(value, wire_value);

    let mut buffer = bytes::BytesMut::with_capacity(4);
    wire.serialize(1, &mut buffer);
    assert_eq!(buffer.len(), wire.size_hint(1));

    let (tag, wire) = Tag::deserialize(&buffer).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    let wire_value = i64::from_wire(wire).unwrap();
    assert_eq!(value, wire_value);
}

#[test]
fn wire_type_f32() {
    let value = std::f32::consts::PI;

    let wire = value.into_wire();
    assert_eq!(wire.size_hint(1), 5);
    assert_eq!(value.size_hint(1), wire.size_hint(1));
    let wire_value = f32::from_wire(wire.as_view()).unwrap();
    assert_eq!(value, wire_value);

    let mut buffer = bytes::BytesMut::with_capacity(5);
    wire.serialize(1, &mut buffer);
    assert_eq!(buffer.len(), wire.size_hint(1));

    let (tag, wire) = Tag::deserialize(&buffer).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    let wire_value = f32::from_wire(wire).unwrap();
    assert_eq!(value, wire_value);
}

#[test]
fn wire_type_f64() {
    let value = std::f64::consts::PI;

    let wire = value.into_wire();
    assert_eq!(wire.size_hint(1), 9);
    assert_eq!(value.size_hint(1), wire.size_hint(1));
    let wire_value = f64::from_wire(wire.as_view()).unwrap();
    assert_eq!(value, wire_value);

    let mut buffer = bytes::BytesMut::with_capacity(9);
    wire.serialize(1, &mut buffer);
    assert_eq!(buffer.len(), wire.size_hint(1));

    let (tag, wire) = Tag::deserialize(&buffer).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    let wire_value = f64::from_wire(wire).unwrap();
    assert_eq!(value, wire_value);
}

#[test]
fn wire_type_string() {
    let value = String::from("Test");

    let wire = value.clone().into_wire();
    assert_eq!(wire.size_hint(1), 6);
    assert_eq!(value.size_hint(1), wire.size_hint(1));
    let wire_value = String::from_wire(wire.as_view()).unwrap();
    assert_eq!(value, wire_value);

    let mut buffer = bytes::BytesMut::with_capacity(9);
    wire.serialize(1, &mut buffer);
    assert_eq!(buffer.len(), wire.size_hint(1));

    let (tag, wire) = Tag::deserialize(&buffer).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    let wire_value = String::from_wire(wire).unwrap();
    assert_eq!(value, wire_value);
}

#[test]
fn wire_type_sgroup() {
    let mut buffer = bytes::BytesMut::with_capacity(1);
    let wire = WireType::SGroup;
    wire.serialize(1, &mut buffer);
    assert_eq!(buffer.len(), wire.size_hint(1));

    let (tag, wire) = Tag::deserialize(&buffer).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    match wire {
        WireTypeView::SGroup => {}
        _ => panic!("wrong wire type"),
    }
}

#[test]
fn wire_type_egroup() {
    let mut buffer = bytes::BytesMut::with_capacity(1);
    let wire = WireType::EGroup;
    wire.serialize(1, &mut buffer);
    assert_eq!(buffer.len(), wire.size_hint(1));

    let (tag, wire) = Tag::deserialize(&buffer).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    match wire {
        WireTypeView::EGroup => {}
        _ => panic!("wrong wire type"),
    }
}

#[test]
fn wire_type_ipv4() {
    let value = std::net::Ipv4Addr::LOCALHOST;

    let wire = value.into_wire();
    assert_eq!(wire.size_hint(1), 6);
    assert_eq!(value.size_hint(1), wire.size_hint(1));
    let wire_value = std::net::Ipv4Addr::from_wire(wire.as_view()).unwrap();
    assert_eq!(value, wire_value);

    let mut buffer = bytes::BytesMut::with_capacity(6);
    wire.serialize(1, &mut buffer);
    assert_eq!(buffer.len(), wire.size_hint(1));

    let (tag, wire) = Tag::deserialize(&buffer).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    let wire_value = std::net::Ipv4Addr::from_wire(wire).unwrap();
    assert_eq!(value, wire_value);
}

/// test messages with manual Message implementation which would usually be derived
mod test_messages {
    use crate::{Error, FromWire, IntoWire, WireTypeView};
    use integer_encoding::VarInt;
    use std::collections::HashMap;

    #[derive(Debug, Clone, PartialEq)]
    pub(super) struct Nested {
        pub(super) whatever: f64,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub(super) struct Test {
        pub(super) map: HashMap<u32, bool>,
        pub(super) nested: Nested,
    }

    impl crate::Message for Nested {
        fn serialize(self, writer: &mut impl bytes::BufMut) {
            let wire_type = self.whatever.into_wire();
            wire_type.serialize(1, writer);
        }

        fn size_hint(&self) -> usize {
            self.whatever.size_hint(1)
        }

        fn deserialize_tags<'a>(tags: impl Iterator<Item = crate::Tag<'a>>) -> Result<Self, Error> {
            let mut whatever = None;

            for tag in tags {
                let (field_number, wire_type) = tag.into_parts();
                match field_number {
                    1 => {
                        whatever = Some(f64::from_wire(wire_type)?);
                    }
                    _ => todo!(),
                }
            }

            Ok(Self {
                whatever: whatever.ok_or(Error::MissingField(1))?,
            })
        }
    }

    impl crate::Message for Test {
        fn serialize(self, writer: &mut impl bytes::BufMut) {
            // prepare a decent size buffer
            let mut buffer = smallvec::SmallVec::<[u8; 512]>::new();
            buffer.resize(512, 0);

            for (key, value) in self.map {
                let size = key.size_hint(1) + value.size_hint(2);
                if size > buffer.len() {
                    buffer.resize(size, 0);
                }

                let mut buffer_ref = buffer.as_mut_slice();

                let wire_type = key.into_wire();
                wire_type.serialize(1, &mut buffer_ref);

                let wire_type = value.into_wire();
                wire_type.serialize(2, &mut buffer_ref);

                let wire_type = WireTypeView::LengthEncoded(&buffer[0..size]);
                wire_type.serialize(1, writer);
            }

            let wire_type = self.nested.into_wire();
            wire_type.serialize(2, writer);
        }

        fn size_hint(&self) -> usize {
            let map_size: usize = self
                .map
                .iter()
                .map(|(key, value)| {
                    let message_size = key.size_hint(1) + value.size_hint(2);
                    message_size + message_size.required_space() + 1.required_space()
                })
                .sum();

            let nested_size = crate::wire::nested::size_hint(2, &self.nested);

            map_size + nested_size
        }

        fn deserialize_tags<'a>(tags: impl Iterator<Item = crate::Tag<'a>>) -> Result<Self, Error> {
            let mut nested = None;
            let mut map = HashMap::new();

            for tag in tags {
                let (field_number, wire_type) = tag.into_parts();
                match field_number {
                    1 => {
                        let (key, value) = crate::map_from_wire(wire_type)?;
                        map.insert(key, value);
                    }
                    2 => {
                        nested = Some(Nested::from_wire(wire_type)?);
                    }
                    _ => todo!(),
                }
            }

            Ok(Self {
                map,
                nested: nested.ok_or(Error::MissingField(2))?,
            })
        }
    }
}

#[test]
fn wire_type_message() {
    let mut map = HashMap::new();
    map.insert(0, false);
    map.insert(1, true);
    let value = test_messages::Test {
        map,
        nested: test_messages::Nested {
            whatever: std::f64::consts::PI,
        },
    };

    let wire = value.clone().into_wire();
    assert_eq!(wire.size_hint(1), 25);
    assert_eq!(IntoWire::size_hint(&value, 1), wire.size_hint(1));
    let wire_value = test_messages::Test::from_wire(wire.as_view()).unwrap();
    assert_eq!(value, wire_value);

    let mut buffer = [0u8; 25];
    wire.serialize(1, &mut buffer.as_mut_slice());
    assert_eq!(buffer.len(), 25);

    let (tag, wire) = Tag::deserialize(&buffer).unwrap().0.into_parts();
    assert_eq!(tag, 1);
    let wire_value = test_messages::Test::from_wire(wire).unwrap();
    assert_eq!(value, wire_value);
}
