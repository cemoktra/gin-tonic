use crate::Encode;

pub struct Encoder<'buf> {
    buffer: &'buf mut [u8],
    position: usize,
}

impl<'buf> Encoder<'buf> {
    pub fn new(buffer: &'buf mut [u8]) -> Self {
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
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        &mut self.buffer[self.position..]
    }
}

impl<'buf> Encode for Encoder<'buf> {
    fn encode_sint32(&mut self, n: i32) {
        let (data, size) = varint_simd::encode_zigzag(n);
        self.buffer_mut()[0..size as usize].clone_from_slice(&data[0..size as usize]);
        self.advance(size as usize);
    }

    fn encode_sint64(&mut self, n: i64) {
        let (data, size) = varint_simd::encode_zigzag(n);
        self.buffer_mut()[0..size as usize].clone_from_slice(&data[0..size as usize]);
        self.advance(size as usize);
    }

    fn encode_uint32(&mut self, n: u32) {
        let (data, size) = varint_simd::encode(n);
        self.buffer_mut()[0..size as usize].clone_from_slice(&data[0..size as usize]);
        self.advance(size as usize);
    }

    fn encode_uint64(&mut self, n: u64) {
        let (data, size) = varint_simd::encode(n);
        self.buffer_mut()[0..size as usize].clone_from_slice(&data[0..size as usize]);
        self.advance(size as usize);
    }

    fn encode_sfixed32(&mut self, n: i32) {
        self.buffer_mut()[0..std::mem::size_of::<i32>()].clone_from_slice(&n.to_le_bytes());
        self.advance(std::mem::size_of::<i32>());
    }

    fn encode_sfixed64(&mut self, n: i64) {
        self.buffer_mut()[0..std::mem::size_of::<i64>()].clone_from_slice(&n.to_le_bytes());
        self.advance(std::mem::size_of::<i64>());
    }

    fn encode_fixed32(&mut self, n: u32) {
        self.buffer_mut()[0..std::mem::size_of::<u32>()].clone_from_slice(&n.to_le_bytes());
        self.advance(std::mem::size_of::<u32>());
    }

    fn encode_fixed64(&mut self, n: u64) {
        self.buffer_mut()[0..std::mem::size_of::<u64>()].clone_from_slice(&n.to_le_bytes());
        self.advance(std::mem::size_of::<u64>());
    }

    fn encode_float(&mut self, n: f32) {
        self.buffer_mut()[0..std::mem::size_of::<f32>()].clone_from_slice(&n.to_le_bytes());
        self.advance(std::mem::size_of::<f32>());
    }

    fn encode_double(&mut self, n: f64) {
        self.buffer_mut()[0..std::mem::size_of::<f64>()].clone_from_slice(&n.to_le_bytes());
        self.advance(std::mem::size_of::<f64>());
    }

    fn encode_bytes(&mut self, b: &[u8]) {
        let len = b.len();
        #[allow(clippy::cast_possible_truncation)]
        self.encode_int32(len as i32);
        self.buffer_mut()[0..len].clone_from_slice(b);
        self.advance(len);
    }
}

#[derive(Debug, Default)]
pub(crate) struct SizeHint {
    size: usize,
}

impl SizeHint {
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }
}

impl Encode for SizeHint {
    #[inline]
    fn encode_sint32(&mut self, n: i32) {
        let (_, size) = varint_simd::encode_zigzag(n);
        self.size += size as usize;
    }

    #[inline]
    fn encode_sint64(&mut self, n: i64) {
        let (_, size) = varint_simd::encode_zigzag(n);
        self.size += size as usize;
    }

    #[inline]
    fn encode_uint32(&mut self, n: u32) {
        let (_, size) = varint_simd::encode(n);
        self.size += size as usize;
    }

    #[inline]
    fn encode_uint64(&mut self, n: u64) {
        let (_, size) = varint_simd::encode(n);
        self.size += size as usize;
    }

    #[inline]
    fn encode_sfixed32(&mut self, _n: i32) {
        self.size += std::mem::size_of::<i32>();
    }

    #[inline]
    fn encode_sfixed64(&mut self, _n: i64) {
        self.size += std::mem::size_of::<i64>();
    }

    #[inline]
    fn encode_fixed32(&mut self, _n: u32) {
        self.size += std::mem::size_of::<u32>();
    }

    #[inline]
    fn encode_fixed64(&mut self, _n: u64) {
        self.size += std::mem::size_of::<u64>();
    }

    #[inline]
    fn encode_float(&mut self, _n: f32) {
        self.size += std::mem::size_of::<f32>();
    }

    #[inline]
    fn encode_double(&mut self, _n: f64) {
        self.size += std::mem::size_of::<f64>();
    }

    #[inline]
    fn encode_bytes(&mut self, b: &[u8]) {
        #[allow(clippy::cast_possible_truncation)]
        self.encode_int32(b.len() as i32);
        self.size += b.len();
    }
}
