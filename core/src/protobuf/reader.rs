use crate::protobuf::tag::Tag;

/// [TagReader] implements an [Iterator] over [Tag]s and can be used to read all [Tag]s from a slice
/// of bytes
pub struct TagReader<'a> {
    position: usize,
    buffer: &'a [u8],
}

impl<'a> TagReader<'a> {
    /// create a new [TagReader] for a slice
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            position: 0,
            buffer,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

impl<'a> Iterator for TagReader<'a> {
    type Item = Tag<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (tag, read) = Tag::deserialize(&self.buffer[self.position..])?;
        self.position += read;
        Some(tag)
    }
}
