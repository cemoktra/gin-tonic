use crate::VarInt;

/// [WireTypeView] is used for reading messages without allocation
#[derive(Debug, Clone, PartialEq)]
pub enum WireTypeView<'a> {
    VarInt(&'a [u8]),
    FixedI64(&'a [u8]),
    SGroup,
    EGroup,
    LengthEncoded(&'a [u8]),
    FixedI32(&'a [u8]),
}

impl<'a> WireTypeView<'a> {
    /// serialize a [WireTypeView] to anything that implements [bytes::BufMut]
    #[inline(always)]
    pub fn serialize(&self, field_number: u32, writer: &mut impl bytes::BufMut) {
        let mut buffer = [0u8; 20];
        let tag = field_number << 3;

        match self {
            WireTypeView::VarInt(data) => {
                let tag_size = tag.encode_var(&mut buffer);
                writer.put_slice(&buffer[0..tag_size]);
                writer.put_slice(data);
            }
            WireTypeView::FixedI64(data) => {
                let tag = tag | 0b1;
                let tag_size = tag.encode_var(&mut buffer);
                writer.put_slice(&buffer[0..tag_size]);
                writer.put_slice(data);
            }
            WireTypeView::LengthEncoded(data) => {
                let tag = tag | 0b10;
                let len: u32 = data.len().try_into().expect("this is good");

                let tag_size = tag.encode_var(&mut buffer);
                let len_size = len.encode_var(&mut buffer[tag_size..]);

                writer.put_slice(&buffer[0..tag_size + len_size]);
                writer.put_slice(data);
            }
            WireTypeView::SGroup => {
                let tag = tag | 0b11;
                let tag_size = tag.encode_var(&mut buffer);
                writer.put_slice(&buffer[0..tag_size]);
            }
            WireTypeView::EGroup => {
                let tag = tag | 0b100;
                let tag_size = tag.encode_var(&mut buffer);
                writer.put_slice(&buffer[0..tag_size]);
            }

            WireTypeView::FixedI32(data) => {
                let tag = tag | 0b101;
                let tag_size = tag.encode_var(&mut buffer);
                writer.put_slice(&buffer[0..tag_size]);
                writer.put_slice(data);
            }
        }
    }

    pub fn size_hint(&self, tag: u32) -> usize {
        match self {
            WireTypeView::VarInt(data) => tag.required_space() + data.len(),
            WireTypeView::FixedI64(_) => tag.required_space() + 8,
            WireTypeView::SGroup => tag.required_space(),
            WireTypeView::EGroup => tag.required_space(),
            WireTypeView::LengthEncoded(data) => {
                let data_len = data.len();
                tag.required_space() + data_len.required_space() + data_len
            }
            WireTypeView::FixedI32(_) => tag.required_space() + 4,
        }
    }
}

/// [WireType] is used for writing messages
#[derive(Debug, Clone, PartialEq)]
pub enum WireType {
    VarInt([u8; 10], u8),
    FixedI64([u8; 8]),
    SGroup,
    EGroup,
    LengthEncoded(bytes::Bytes),
    FixedI32([u8; 4]),
}

impl WireType {
    /// serialize a [WireType] to anything that implements [bytes::BufMut]
    #[inline(always)]
    pub fn serialize(&self, field_number: u32, writer: &mut impl bytes::BufMut) {
        let mut tag_varint = [0u8; 10];
        let tag = field_number << 3;

        match self {
            WireType::VarInt(data, size) => {
                let tag_size = tag.encode_var(&mut tag_varint);
                writer.put_slice(&tag_varint[0..tag_size]);
                writer.put_slice(&data[0..*size as usize]);
            }
            WireType::FixedI64(data) => {
                let tag = tag | 0b1;
                let tag_size = tag.encode_var(&mut tag_varint);
                writer.put_slice(&tag_varint[0..tag_size]);
                writer.put_slice(data);
            }
            WireType::LengthEncoded(data) => {
                let mut len_varint = [0u8; 10];

                let tag = tag | 0b10;
                let tag_size = tag.encode_var(&mut tag_varint);

                writer.put_slice(&tag_varint[0..tag_size]);

                let len: u32 = data.len().try_into().expect("this is good");
                let len_size = len.encode_var(&mut len_varint);

                writer.put_slice(&len_varint[0..len_size]);
                writer.put_slice(data);
            }
            WireType::SGroup => {
                let tag = tag | 0b11;
                let tag_size = tag.encode_var(&mut tag_varint);
                writer.put_slice(&tag_varint[0..tag_size]);
            }
            WireType::EGroup => {
                let tag = tag | 0b100;
                let tag_size = tag.encode_var(&mut tag_varint);
                writer.put_slice(&tag_varint[0..tag_size]);
            }

            WireType::FixedI32(data) => {
                let tag = tag | 0b101;
                let tag_size = tag.encode_var(&mut tag_varint);
                writer.put_slice(&tag_varint[0..tag_size]);
                writer.put_slice(data);
            }
        }
    }

    pub fn size_hint(&self, tag: u32) -> usize {
        match self {
            WireType::VarInt(_, size) => tag.required_space() + *size as usize,
            WireType::FixedI64(_) => tag.required_space() + 8,
            WireType::SGroup => tag.required_space(),
            WireType::EGroup => tag.required_space(),
            WireType::LengthEncoded(data) => {
                let data_len = data.len();
                tag.required_space() + data_len.required_space() + data_len
            }
            WireType::FixedI32(_) => tag.required_space() + 4,
        }
    }

    pub fn as_view(&self) -> WireTypeView {
        match self {
            WireType::VarInt(data, _) => WireTypeView::VarInt(data),
            WireType::FixedI64(data) => WireTypeView::FixedI64(data),
            WireType::SGroup => WireTypeView::SGroup,
            WireType::EGroup => WireTypeView::EGroup,
            WireType::LengthEncoded(data) => WireTypeView::LengthEncoded(data),
            WireType::FixedI32(data) => WireTypeView::FixedI32(data),
        }
    }
}
