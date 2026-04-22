#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag(u32);

impl Tag {
    #[inline]
    pub fn into_parts(self) -> (u32, u8) {
        (self.0 >> 3, (self.0 & 0b111) as u8)
    }

    #[inline]
    pub fn from_parts(field_number: u32, wire_type: u8) -> Self {
        Self(field_number << 3 | wire_type as u32)
    }

    #[inline]
    pub fn wire_type(&self) -> u8 {
        (self.0 & 0b111) as u8
    }

    #[inline]
    pub fn field_number(&self) -> u32 {
        self.0 >> 3
    }
}

impl From<u32> for Tag {
    #[inline]
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Tag> for u32 {
    #[inline]
    fn from(value: Tag) -> Self {
        value.0
    }
}
