use std::io::{Result, Write};

/// protobuf VARINT type
pub trait VarInt
where
    Self: Sized,
{
    fn decode_slice(buffer: &[u8]) -> Result<(Self, usize)>;

    fn encode_writer(self, writer: &mut impl Write) -> Result<()>;
}

impl VarInt for u64 {
    fn decode_slice(buffer: &[u8]) -> Result<(Self, usize)> {
        let mut read = 0;
        let mut decoded = 0;

        for b in buffer {
            decoded |= u64::from(*b & 0x7F) << (read * 7);
            read += 1;

            if *b <= 0x7F {
                return if read == 9 && *b >= 0x02 {
                    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, ""))
                } else {
                    Ok((decoded, read))
                };
            }
        }

        Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, ""))
    }

    fn encode_writer(mut self, writer: &mut impl Write) -> std::io::Result<()> {
        loop {
            if self < 0x80 {
                writer.write(&[self as u8])?;
                return Ok(());
            }

            writer.write(&[((self & 0x7F) | 0x80) as u8])?;
            self >>= 7;
        }
    }
}

// groups are not supported
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum WireType {
    VarInt = 0,
    I64 = 1,
    LEN = 2,
    I32 = 5,
}

pub trait Tag {
    fn field_number(&self) -> u64;
    fn wire_type(&self) -> Option<WireType>;
}

impl Tag for u64 {
    fn field_number(&self) -> u64 {
        self >> 3
    }

    fn wire_type(&self) -> Option<WireType> {
        let wire_type = self & 0b111;

        match wire_type {
            0 => Some(WireType::VarInt),
            1 => Some(WireType::I64),
            2 => Some(WireType::LEN),
            5 => Some(WireType::I32),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::protobuf::{Tag, VarInt};

    #[test]
    fn encode_decode_varint() {
        let mut buffer = Vec::new();

        let var = 150u64;

        var.encode_writer(&mut buffer).unwrap();
        assert_eq!([0x96, 0x1], buffer[0..2]);

        let (var, read) = u64::decode_slice(buffer.as_slice()).unwrap();

        assert_eq!(150, var);
        assert_eq!(2, read);
    }

    #[test]
    fn tag() {
        let tag = 0x8;
        assert_eq!(Some(super::WireType::VarInt), tag.wire_type());
        assert_eq!(1, tag.field_number());
    }

    #[test]
    fn decode_tag_and_varint() {
        let buffer = [0x08, 0x96, 0x01];

        let (tag, read) = u64::decode_slice(buffer.as_slice()).unwrap();
        assert_eq!(read, 1);
        assert_eq!(Some(super::WireType::VarInt), tag.wire_type());
        assert_eq!(1, tag.field_number());

        let (var, read) = u64::decode_slice(&buffer[1..]).unwrap();
        assert_eq!(150, var);
        assert_eq!(2, read);
    }
}
