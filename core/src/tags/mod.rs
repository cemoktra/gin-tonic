//! handling of protobuf tags

pub mod reader;

use crate::WireTypeView;
use integer_encoding::VarInt;

/// representation of a deserialized [Tag]
#[derive(Clone, Debug)]
pub struct Tag<'a> {
    field_number: u32,
    wire_type: WireTypeView<'a>,
}

impl<'a> Tag<'a> {
    /// split the [Tag] into field number and [WireTypeView]
    pub fn into_parts(self) -> (u32, WireTypeView<'a>) {
        (self.field_number, self.wire_type)
    }

    /// access the field number
    pub fn field_number(&self) -> u32 {
        self.field_number
    }

    /// access the [WireTypeView]
    pub fn wire_type(&self) -> &WireTypeView {
        &self.wire_type
    }

    /// deserialize a tag
    pub fn deserialize(buf: &'a [u8]) -> Option<(Self, usize)> {
        let (tag, tag_read) = u32::decode_var(buf)?;

        let field_number = tag >> 3;
        let wire_type = tag & 0b111;

        let (wire_type, read) = match wire_type {
            0 => {
                let (_data, read) = u64::decode_var(&buf[tag_read..])?;
                (WireTypeView::VarInt(&buf[tag_read..tag_read + read]), read)
            }
            1 => (WireTypeView::FixedI64(&buf[tag_read..9]), 8),
            2 => {
                let (len, read) = u32::decode_var(&buf[tag_read..])?;
                let len = len as usize;
                let offset = tag_read + read;

                (
                    WireTypeView::LengthEncoded(&buf[offset..offset + len]),
                    read + len,
                )
            }
            3 => (WireTypeView::SGroup, 0),
            4 => (WireTypeView::EGroup, 0),
            5 => (WireTypeView::FixedI32(&buf[tag_read..5]), 4),
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
