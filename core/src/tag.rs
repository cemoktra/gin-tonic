/// a protobuf tag is a u32 the combines the field number and a wire type
pub trait Tag {
    fn from_parts(field_number: u32, wire_type: u8) -> Self;

    fn wire_type(&self) -> u8;
    fn field_number(&self) -> u32;
}

impl Tag for u32 {
    fn from_parts(field_number: u32, wire_type: u8) -> Self {
        field_number << 3 | wire_type as u32
    }

    fn wire_type(&self) -> u8 {
        (self & 0b111) as u8
    }

    fn field_number(&self) -> u32 {
        self >> 3
    }
}
