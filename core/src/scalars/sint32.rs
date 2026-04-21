use crate::{Scalar, scalars::SInt32, wire_types::WIRE_TYPE_VARINT};

impl Scalar<SInt32> for i32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_sint32(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_sint32()
    }
}

#[cfg(test)]
mod test {
    use crate::scalars::SInt32;

    #[test]
    fn encode_decode() {
        let test_cases = [
            (0, 1, b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            (1, 1, b"\x02\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            (63, 1, b"\x7e\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            (64, 2, b"\x80\x01\x00\x00\x00\x00\x00\x00\x00\x00"),
            (8_191, 2, b"\xfe\x7f\x00\x00\x00\x00\x00\x00\x00\x00"),
            (8_192, 3, b"\x80\x80\x01\x00\x00\x00\x00\x00\x00\x00"),
            (1_048_575, 3, b"\xfe\xff\x7f\x00\x00\x00\x00\x00\x00\x00"),
            (1_048_576, 4, b"\x80\x80\x80\x01\x00\x00\x00\x00\x00\x00"),
            (134_217_727, 4, b"\xfe\xff\xff\x7f\x00\x00\x00\x00\x00\x00"),
            (134_217_728, 5, b"\x80\x80\x80\x80\x01\x00\x00\x00\x00\x00"),
            (i32::MAX, 5, b"\xfe\xff\xff\xff\x0f\x00\x00\x00\x00\x00"),
            (-1, 1, b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            (-64, 1, b"\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            (-65, 2, b"\x81\x01\x00\x00\x00\x00\x00\x00\x00\x00"),
            (-8_192, 2, b"\xff\x7f\x00\x00\x00\x00\x00\x00\x00\x00"),
            (-8_193, 3, b"\x81\x80\x01\x00\x00\x00\x00\x00\x00\x00"),
            (-1_048_576, 3, b"\xff\xff\x7f\x00\x00\x00\x00\x00\x00\x00"),
            (-1_048_577, 4, b"\x81\x80\x80\x01\x00\x00\x00\x00\x00\x00"),
            (-134_217_728, 4, b"\xff\xff\xff\x7f\x00\x00\x00\x00\x00\x00"),
            (-134_217_729, 5, b"\x81\x80\x80\x80\x01\x00\x00\x00\x00\x00"),
            (i32::MIN, 5, b"\xff\xff\xff\xff\x0f\x00\x00\x00\x00\x00"),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, SInt32>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
