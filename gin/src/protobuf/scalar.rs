use crate::protobuf::wire::{WireType, WireTypeView};
use crate::protobuf::{Error, FromWire, IntoWire, Message};
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

    fn size_hint(&self, tag: u32) -> usize {
        self.required_space() + tag.required_space()
    }
}

impl FromWire for i32 {
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::VarInt(data) => {
                let (value, _) = i32::decode_var(data).ok_or(Error::InvalidVarInt)?;
                Ok(value)
            }
            WireTypeView::FixedI32(data) => {
                let array: [u8; 4] = data.try_into().expect("I32 is always 4 bytes");
                Ok(i32::from_be_bytes(array))
            }
            _ => Err(Error::UnexpectedWireType),
        }
    }
}

impl IntoWire for i32 {
    fn into_wire(self) -> WireType {
        let mut data = [0u8; 10];
        let size = self.encode_var(&mut data);
        WireType::VarInt(data, size)
    }

    fn size_hint(&self, tag: u32) -> usize {
        self.required_space() + tag.required_space()
    }
}

impl FromWire for String {
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::LengthEncoded(data) => Ok(String::from_utf8(data.to_vec())?),
            _ => Err(Error::UnexpectedWireType),
        }
    }
}

impl IntoWire for String {
    fn into_wire(self) -> WireType {
        WireType::LengthEncoded(self.into_bytes())
    }

    fn size_hint(&self, tag: u32) -> usize {
        let len = self.len();
        len.required_space() + tag.required_space() + len
    }
}

impl<T> IntoWire for T
where
    T: Message,
{
    fn into_wire(self) -> WireType {
        let mut buffer = Vec::with_capacity(self.size_hint());
        self.serialize(&mut buffer).expect("must work");
        WireType::LengthEncoded(buffer)
    }

    fn size_hint(&self, tag: u32) -> usize {
        tag.required_space() + self.size_hint().required_space()
    }
}

impl<T> FromWire for T
where
    T: Message,
{
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match wire {
            WireTypeView::LengthEncoded(data) => {
                let (value, _) = T::deserialize(data)?;
                Ok(value)
            }
            _ => Err(Error::UnexpectedWireType),
        }
    }
}
