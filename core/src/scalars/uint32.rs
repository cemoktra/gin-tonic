use crate::{Scalar, scalars::UInt32, wire_types::WIRE_TYPE_VARINT};

impl Scalar<UInt32> for u32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_uint32(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_uint32()
    }
}

#[cfg(test)]
mod test {
    use crate::scalars::UInt32;

    #[test]
    fn encode_decode() {
        let test_cases = [
            (0, 1, b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            (1, 1, b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            (127, 1, b"\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            (128, 2, b"\x80\x01\x00\x00\x00\x00\x00\x00\x00\x00"),
            (16_383, 2, b"\xff\x7f\x00\x00\x00\x00\x00\x00\x00\x00"),
            (16_384, 3, b"\x80\x80\x01\x00\x00\x00\x00\x00\x00\x00"),
            (2_097_151, 3, b"\xff\xff\x7f\x00\x00\x00\x00\x00\x00\x00"),
            (2_097_152, 4, b"\x80\x80\x80\x01\x00\x00\x00\x00\x00\x00"),
            (268_435_455, 4, b"\xff\xff\xff\x7f\x00\x00\x00\x00\x00\x00"),
            (268_435_456, 5, b"\x80\x80\x80\x80\x01\x00\x00\x00\x00\x00"),
            (u32::MAX, 5, b"\xff\xff\xff\xff\x0f\x00\x00\x00\x00\x00"),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, UInt32>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
