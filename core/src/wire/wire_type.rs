use integer_encoding::VarInt;

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

/// [WireType] is used for writing messages
#[derive(Debug, Clone, PartialEq)]
pub enum WireType {
    VarInt([u8; 10], usize),
    FixedI64([u8; 8]),
    SGroup,
    EGroup,
    LengthEncoded(Vec<u8>),
    FixedI32([u8; 4]),
}

impl WireType {
    /// serialize a [WireType] to anything that implements [std::io::Write]
    pub fn serialize(
        &self,
        field_number: u32,
        writer: &mut impl std::io::Write,
    ) -> std::io::Result<usize> {
        let mut tag_varint = [0u8; 10];
        let tag = field_number << 3;
        let mut written = 0;

        match self {
            WireType::VarInt(data, size) => {
                let tag_size = tag.encode_var(&mut tag_varint);
                written += writer.write(&tag_varint[0..tag_size])?;
                written += writer.write(&data[0..*size])?;
            }
            WireType::FixedI64(data) => {
                let tag = tag | 0b1;
                let tag_size = tag.encode_var(&mut tag_varint);
                written += writer.write(&tag_varint[0..tag_size])?;
                written += writer.write(data)?;
            }
            WireType::LengthEncoded(data) => {
                let mut len_varint = [0u8; 10];

                let tag = tag | 0b10;
                let tag_size = tag.encode_var(&mut tag_varint);
                written += writer.write(&tag_varint[0..tag_size])?;

                let len: u32 = data.len().try_into().expect("this is good");
                let len_size = len.encode_var(&mut len_varint);
                written += writer.write(&len_varint[0..len_size])?;
                written += writer.write(data)?;
            }
            WireType::SGroup => {
                let tag = tag | 0b11;
                let tag_size = tag.encode_var(&mut tag_varint);
                written += writer.write(&tag_varint[0..tag_size])?;
            }
            WireType::EGroup => {
                let tag = tag | 0b100;
                let tag_size = tag.encode_var(&mut tag_varint);
                written += writer.write(&tag_varint[0..tag_size])?;
            }

            WireType::FixedI32(data) => {
                let tag = tag | 0b101;
                let tag_size = tag.encode_var(&mut tag_varint);
                written += writer.write(&tag_varint[0..tag_size])?;
                written += writer.write(data)?;
            }
        }

        Ok(written)
    }

    pub fn size_hint(&self, tag: u32) -> usize {
        match self {
            WireType::VarInt(_, size) => tag.required_space() + size,
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
