use super::IntoWire;
use crate::VarInt;

/// calculate the size_hint of a nested message
pub fn size_hint<T>(tag: u32, message: &T) -> usize
where
    T: IntoWire,
{
    let size = message.size_hint(tag);
    tag.required_space() as usize + size.required_space() as usize + size
}
