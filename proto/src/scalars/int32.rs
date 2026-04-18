use crate::{Scalar, scalars::Int32, wire_types::WIRE_TYPE_VARINT};

impl Scalar<Int32> for i32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_int32(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_int32()
    }
}

#[cfg(test)]
mod test {
    use crate::scalars::Int32;

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
            (i32::MAX, 5, b"\xff\xff\xff\xff\x07\x00\x00\x00\x00\x00"),
            (-1, 10, b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01"),
            (-128, 10, b"\x80\xff\xff\xff\xff\xff\xff\xff\xff\x01"),
            (-129, 10, b"\xff\xfe\xff\xff\xff\xff\xff\xff\xff\x01"),
            (-16_384, 10, b"\x80\x80\xff\xff\xff\xff\xff\xff\xff\x01"),
            (-16_385, 10, b"\xff\xff\xfe\xff\xff\xff\xff\xff\xff\x01"),
            (-2_097_152, 10, b"\x80\x80\x80\xff\xff\xff\xff\xff\xff\x01"),
            (-2_097_153, 10, b"\xff\xff\xff\xfe\xff\xff\xff\xff\xff\x01"),
            (
                -268_435_456,
                10,
                b"\x80\x80\x80\x80\xff\xff\xff\xff\xff\x01",
            ),
            (
                -268_435_457,
                10,
                b"\xff\xff\xff\xff\xfe\xff\xff\xff\xff\x01",
            ),
            (i32::MIN, 10, b"\x80\x80\x80\x80\xf8\xff\xff\xff\xff\x01"),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, Int32>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
