use crate::protobuf::wire::{WireType, WireTypeView};
use crate::protobuf::{Error, FromWire, IntoWire};
use integer_encoding::VarInt;

impl FromWire for u32 {
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::VarInt(data) => {
                let (value, _) = u32::decode_var(data).ok_or(Error::InvalidVarInt)?;
                Ok(value)
            }
            WireTypeView::FixedI32(data) => {
                let array: [u8; 4] = data.try_into().expect("I32 is always 4 bytes");
                Ok(u32::from_be_bytes(array))
            }
            _ => Err(Error::UnexpectedWireType),
        }
    }
}

impl IntoWire for u32 {
    fn into_wire(self) -> WireType {
        let mut data = [0u8; 10];
        let size = self.encode_var(&mut data);
        WireType::VarInt(data, size)
    }

    fn size_hint(&self) -> usize {
        self.required_space()
    }
}
