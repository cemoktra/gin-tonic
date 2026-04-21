use crate::Decode;

pub struct Decoder<'buf> {
    buffer: &'buf [u8],
    position: usize,
}

impl<'buf> Decoder<'buf> {
    #[inline]
    pub fn new(buffer: &'buf [u8]) -> Self {
        Self {
            buffer,
            position: 0,
        }
    }

    #[inline]
    pub fn advance(&mut self, offset: usize) {
        self.position += offset
    }

    #[inline]
    pub fn position(&self) -> usize {
        self.position
    }

    #[inline]
    pub fn buffer(&self) -> &'buf [u8] {
        &self.buffer[self.position..]
    }
}

impl<'buf> Decode for Decoder<'buf> {
    fn buffer(&self) -> &[u8] {
        &self.buffer[self.position..]
    }

    fn decode_sint32(&mut self) -> Result<i32, crate::error::ProtoError> {
        let (value, size) = varint_simd::decode_zigzag(self.buffer())?;
        self.advance(size);
        Ok(value)
    }

    fn decode_sint64(&mut self) -> Result<i64, crate::error::ProtoError> {
        let (value, size) = varint_simd::decode_zigzag(self.buffer())?;
        self.advance(size);
        Ok(value)
    }

    #[inline]
    fn decode_uint32(&mut self) -> Result<u32, crate::error::ProtoError> {
        let (value, size) = varint_simd::decode(self.buffer())?;
        self.advance(size);
        Ok(value)
    }

    #[inline]
    fn decode_uint64(&mut self) -> Result<u64, crate::error::ProtoError> {
        let (value, size) = varint_simd::decode(self.buffer())?;
        self.advance(size);
        Ok(value)
    }

    fn decode_sfixed32(&mut self) -> Result<i32, crate::error::ProtoError> {
        let value = i32::from_le_bytes(self.buffer()[0..std::mem::size_of::<i32>()].try_into()?);
        self.advance(std::mem::size_of::<i32>());
        Ok(value)
    }

    fn decode_sfixed64(&mut self) -> Result<i64, crate::error::ProtoError> {
        let value = i64::from_le_bytes(self.buffer()[0..std::mem::size_of::<i64>()].try_into()?);
        self.advance(std::mem::size_of::<i64>());
        Ok(value)
    }

    fn decode_fixed32(&mut self) -> Result<u32, crate::error::ProtoError> {
        let value = u32::from_le_bytes(self.buffer()[0..std::mem::size_of::<u32>()].try_into()?);
        self.advance(std::mem::size_of::<u32>());
        Ok(value)
    }

    fn decode_fixed64(&mut self) -> Result<u64, crate::error::ProtoError> {
        let value = u64::from_le_bytes(self.buffer()[0..std::mem::size_of::<u64>()].try_into()?);
        self.advance(std::mem::size_of::<u64>());
        Ok(value)
    }

    fn decode_float(&mut self) -> Result<f32, crate::error::ProtoError> {
        let value = f32::from_le_bytes(self.buffer()[0..std::mem::size_of::<f32>()].try_into()?);
        self.advance(std::mem::size_of::<f32>());
        Ok(value)
    }

    fn decode_double(&mut self) -> Result<f64, crate::error::ProtoError> {
        let value = f64::from_le_bytes(self.buffer()[0..std::mem::size_of::<f64>()].try_into()?);
        self.advance(std::mem::size_of::<f64>());
        Ok(value)
    }

    fn decode_bytes(&mut self) -> Result<Vec<u8>, crate::error::ProtoError> {
        let len = self.decode_uint32()? as usize;
        let vec = (self.buffer()[0..len]).to_vec();
        self.advance(len);
        Ok(vec)
    }
}
