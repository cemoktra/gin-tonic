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
        let (tag, read) = u32::decode_var(buf)?;
        if read != 1 {
            return None;
        }

        let field_number = tag >> 3;
        let wire_type = tag & 0b111;

        let (wire_type, read) = match wire_type {
            0 => {
                let mut data = [0u8; 10];
                let (_data, read) = u64::decode_var(&buf[1..])?;
                data[0..read].copy_from_slice(&buf[1..1 + read]);

                (WireType::VarInt(data), read)
            }
            1 => {
                let mut data = [0u8; 8];
                data.copy_from_slice(&buf[1..9]);
                (WireType::I64(data), 8)
            }
            2 => {
                let (len, read) = u32::decode_var(&buf[1..])?;
                let len = len as usize;
                let data = buf[2..2 + len].to_vec();

                (WireType::LEN(data), read + len)
            }
            5 => {
                let mut data = [0u8; 4];
                data.copy_from_slice(&buf[1..5]);
                (WireType::I32(data), 4)
            }
            _ => return None,
        };

        Some((
            Self {
                field_number,
                wire_type,
            },
            read + 1,
        ))
    }
}

/// groups are not supported
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum WireType {
    VarInt([u8; 10]),
    I64([u8; 8]),
    LEN(Vec<u8>),
    I32([u8; 4]),
}

#[cfg(test)]
mod test {
    use crate::protobuf::{Tag, TagReader, WireType};
    use integer_encoding::VarInt;

    #[test]
    fn parse_varint_tag() {
        // field = 1
        // wire_type = VARINT with u64 = 150
        let buffer = [0x08, 0x96, 0x01];
        let (tag, _) = Tag::parse(&buffer).unwrap();

        assert_eq!(1, tag.field_number());
        match tag.wire_type() {
            WireType::VarInt(data) => {
                assert_eq!(150u64, u64::decode_var(data).unwrap().0);
            }
            _ => panic!("incorrect wire type"),
        }
    }

    #[test]
    fn parse_len_tag() {
        // field = 2
        // wire_type = LEN with [74 65 73 74 69 6e 67]
        let buffer = [0x12, 0x07, 0x74, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67];
        let (tag, _) = Tag::parse(&buffer).unwrap();

        assert_eq!(2, tag.field_number());
        match tag.wire_type() {
            WireType::LEN(data) => {
                let text = String::from_utf8(data.clone()).unwrap();
                assert_eq!(&text, "testing");
            }
            _ => panic!("incorrect wire type"),
        }
    }

    #[test]
    fn parse_multi_tags() {
        let buffer = [
            0x22, 0x05, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x28, 0x01, 0x28, 0x02, 0x28, 0x03,
        ];

        let mut tag_reader = TagReader::new(&buffer);

        let tag = tag_reader.next().unwrap();
        assert_eq!(4, tag.field_number);
        match tag.wire_type() {
            WireType::LEN(data) => {
                let text = String::from_utf8(data.clone()).unwrap();
                assert_eq!(&text, "hello");
            }
            _ => panic!("incorrect wire type"),
        }

        let tag = tag_reader.next().unwrap();
        assert_eq!(5, tag.field_number);
        println!("{tag:?}");
        match tag.wire_type() {
            WireType::VarInt(data) => {
                assert_eq!(1u32, u32::decode_var(data).unwrap().0);
            }
            _ => panic!("incorrect wire type"),
        }

        let tag = tag_reader.next().unwrap();
        assert_eq!(5, tag.field_number);
        println!("{tag:?}");
        match tag.wire_type() {
            WireType::VarInt(data) => {
                assert_eq!(2u32, u32::decode_var(data).unwrap().0);
            }
            _ => panic!("incorrect wire type"),
        }

        let tag = tag_reader.next().unwrap();
        assert_eq!(5, tag.field_number);
        println!("{tag:?}");
        match tag.wire_type() {
            WireType::VarInt(data) => {
                assert_eq!(3u32, u32::decode_var(data).unwrap().0);
            }
            _ => panic!("incorrect wire type"),
        }

        assert!(tag_reader.next().is_none());
    }
}
