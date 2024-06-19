use integer_encoding::VarInt;

pub struct TagReader<'a> {
    position: usize,
    buffer: &'a [u8],
}

impl<'a> TagReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            position: 0,
            buffer,
        }
    }
}

impl<'a> Iterator for TagReader<'a> {
    type Item = Tag;

    fn next(&mut self) -> Option<Self::Item> {
        let (tag, read) = Tag::parse(&self.buffer[self.position..])?;
        self.position += read;
        Some(tag)
    }
}

#[derive(Clone, Debug)]
pub struct Tag {
    field_number: u32,
    wire_type: WireType,
}

impl Tag {
    pub fn field_number(&self) -> u32 {
        self.field_number
    }

    pub fn wire_type(&self) -> &WireType {
        &self.wire_type
    }

    pub fn parse(buf: &[u8]) -> Option<(Self, usize)> {
        let (tag, tag_read) = u32::decode_var(buf)?;

        let field_number = tag >> 3;
        let wire_type = tag & 0b111;

        let (wire_type, read) = match wire_type {
            0 => {
                let mut data = [0u8; 10];
                let (_data, read) = u64::decode_var(&buf[tag_read..])?;
                data[0..read].copy_from_slice(&buf[tag_read..tag_read + read]);

                (WireType::VarInt(data), read)
            }
            1 => {
                let mut data = [0u8; 8];
                data.copy_from_slice(&buf[tag_read..9]);
                (WireType::I64(data), 8)
            }
            2 => {
                let (len, read) = u32::decode_var(&buf[tag_read..])?;
                let len = len as usize;
                let offset = tag_read + read;
                let data = buf[offset..offset + len].to_vec();

                (WireType::LEN(data), read + len)
            }
            3 => (WireType::SGroup, 0),
            4 => (WireType::EGroup, 0),
            5 => {
                let mut data = [0u8; 4];
                data.copy_from_slice(&buf[tag_read..5]);
                (WireType::I32(data), 4)
            }
            _ => return None,
        };

        Some((
            Self {
                field_number,
                wire_type,
            },
            tag_read + read,
        ))
    }
}

/// groups are not supported
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub enum WireType {
    VarInt([u8; 10]),
    I64([u8; 8]),
    SGroup,
    EGroup,
    LEN(Vec<u8>),
    I32([u8; 4]),
}

#[cfg(test)]
mod test {
    use crate::protobuf::{TagReader, WireType};
    use integer_encoding::VarInt;

    #[test]
    fn proto3_compliance() {
        // https://github.com/protocolbuffers/protoscope/blob/main/testdata/proto3.pb
        let buffer = std::fs::read("proto3.pb").unwrap();
        let mut reader = TagReader::new(&buffer);

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 31);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(&data[..], [0x65, 0xad, 0x02]);
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 32);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(&data[..], [0xca, 0x01, 0xae, 0x02]);
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 33);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(&data[..], [0xcb, 0x01, 0xaf, 0x02]);
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 34);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(&data[..], [0xcc, 0x01, 0xb0, 0x02]);
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 35);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(&data[..], [0x9a, 0x03, 0xe2, 0x04]);
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 36);
        if let WireType::LEN(data) = tag.wire_type() {
            let mut sub_reader = TagReader::new(&data);

            let tag = sub_reader.next().unwrap();
            assert_eq!(tag.field_number(), 51);
            assert_eq!(*tag.wire_type(), WireType::EGroup);

            let tag = sub_reader.next().unwrap();
            assert_eq!(tag.field_number(), 76);
            assert_eq!(*tag.wire_type(), WireType::EGroup);

            assert!(sub_reader.next().is_none());
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 37);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(&data[..], [0xcf, 0x00, 0x00, 0x00, 0x33, 0x01, 0x00, 0x00]);
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 38);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(
                &data[..],
                [
                    0xd0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x01, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00
                ]
            );
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 39);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(&data[..], [0xd1, 0x00, 0x00, 0x00, 0x35, 0x01, 0x00, 0x00]);
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 40);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(
                &data[..],
                [
                    0xd2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x01, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00
                ]
            );
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 41);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(&data[..], [0x00, 0x00, 0x53, 0x43, 0x00, 0x80, 0x9b, 0x43]);
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 42);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(
                &data[..],
                [
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x6a, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x80, 0x73, 0x40
                ]
            );
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 43);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(&data[..], [0x01, 0x00]);
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 44);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(String::from_utf8(data.clone()).unwrap(), "215");
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 44);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(String::from_utf8(data.clone()).unwrap(), "315");
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 45);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(String::from_utf8(data.clone()).unwrap(), "216");
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 45);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(String::from_utf8(data.clone()).unwrap(), "316");
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 48);
        if let WireType::LEN(data) = tag.wire_type() {
            let mut sub_reader = TagReader::new(&data);

            let tag = sub_reader.next().unwrap();
            assert_eq!(tag.field_number(), 1);
            if let WireType::VarInt(data) = tag.wire_type() {
                let (data, _) = u32::decode_var(data).unwrap();
                assert_eq!(218, data);
            }

            assert!(sub_reader.next().is_none());
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 48);
        if let WireType::LEN(data) = tag.wire_type() {
            let mut sub_reader = TagReader::new(&data);

            let tag = sub_reader.next().unwrap();
            assert_eq!(tag.field_number(), 1);
            if let WireType::VarInt(data) = tag.wire_type() {
                let (data, _) = u32::decode_var(data).unwrap();
                assert_eq!(318, data);
            }

            assert!(sub_reader.next().is_none());
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 49);
        if let WireType::LEN(data) = tag.wire_type() {
            let mut sub_reader = TagReader::new(&data);

            let tag = sub_reader.next().unwrap();
            assert_eq!(tag.field_number(), 1);
            if let WireType::VarInt(data) = tag.wire_type() {
                let (data, _) = u32::decode_var(data).unwrap();
                assert_eq!(219, data);
            }

            assert!(sub_reader.next().is_none());
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 49);
        if let WireType::LEN(data) = tag.wire_type() {
            let mut sub_reader = TagReader::new(&data);

            let tag = sub_reader.next().unwrap();
            assert_eq!(tag.field_number(), 1);
            if let WireType::VarInt(data) = tag.wire_type() {
                let (data, _) = u32::decode_var(data).unwrap();
                assert_eq!(319, data);
            }

            assert!(sub_reader.next().is_none());
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 50);
        if let WireType::LEN(data) = tag.wire_type() {
            let mut sub_reader = TagReader::new(&data);

            let tag = sub_reader.next().unwrap();
            assert_eq!(tag.field_number(), 1);
            if let WireType::VarInt(data) = tag.wire_type() {
                let (data, _) = u32::decode_var(data).unwrap();
                assert_eq!(220, data);
            }

            assert!(sub_reader.next().is_none());
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 50);
        if let WireType::LEN(data) = tag.wire_type() {
            let mut sub_reader = TagReader::new(&data);

            let tag = sub_reader.next().unwrap();
            assert_eq!(tag.field_number(), 1);
            if let WireType::VarInt(data) = tag.wire_type() {
                let (data, _) = u32::decode_var(data).unwrap();
                assert_eq!(320, data);
            }

            assert!(sub_reader.next().is_none());
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 51);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(&data[..], [0x02, 0x03]);
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 52);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(&data[..], [0x05, 0x06]);
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 54);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(String::from_utf8(data.clone()).unwrap(), "224");
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 54);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(String::from_utf8(data.clone()).unwrap(), "324");
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 55);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(String::from_utf8(data.clone()).unwrap(), "225");
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 55);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(String::from_utf8(data.clone()).unwrap(), "325");
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 57);
        if let WireType::LEN(data) = tag.wire_type() {
            let mut sub_reader = TagReader::new(&data);

            let tag = sub_reader.next().unwrap();
            assert_eq!(tag.field_number(), 1);
            if let WireType::VarInt(data) = tag.wire_type() {
                let (data, _) = u32::decode_var(data).unwrap();
                assert_eq!(227, data);
            }

            assert!(sub_reader.next().is_none());
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 57);
        if let WireType::LEN(data) = tag.wire_type() {
            let mut sub_reader = TagReader::new(&data);

            let tag = sub_reader.next().unwrap();
            assert_eq!(tag.field_number(), 1);
            if let WireType::VarInt(data) = tag.wire_type() {
                let (data, _) = u32::decode_var(data).unwrap();
                assert_eq!(327, data);
            }

            assert!(sub_reader.next().is_none());
        }

        let tag = reader.next().unwrap();
        assert_eq!(tag.field_number(), 114);
        if let WireType::LEN(data) = tag.wire_type() {
            assert_eq!(String::from_utf8(data.clone()).unwrap(), "604");
        }

        assert!(reader.next().is_none());
    }
}
