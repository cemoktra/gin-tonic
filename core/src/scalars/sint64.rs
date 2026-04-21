use crate::{Scalar, scalars::SInt64, wire_types::WIRE_TYPE_VARINT};

impl Scalar<SInt64> for i64 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_sint64(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_sint64()
    }
}

#[cfg(test)]
mod test {
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
            (
                17_179_869_183,
                5,
                b"\xfe\xff\xff\xff\x7f\x00\x00\x00\x00\x00",
            ),
            (
                17_179_869_184,
                6,
                b"\x80\x80\x80\x80\x80\x01\x00\x00\x00\x00",
            ),
            (
                2_199_023_255_551,
                6,
                b"\xfe\xff\xff\xff\xff\x7f\x00\x00\x00\x00",
            ),
            (
                2_199_023_255_552,
                7,
                b"\x80\x80\x80\x80\x80\x80\x01\x00\x00\x00",
            ),
            (
                281_474_976_710_655,
                7,
                b"\xfe\xff\xff\xff\xff\xff\x7f\x00\x00\x00",
            ),
            (
                281_474_976_710_656,
                8,
                b"\x80\x80\x80\x80\x80\x80\x80\x01\x00\x00",
            ),
            (
                36_028_797_018_963_967,
                8,
                b"\xfe\xff\xff\xff\xff\xff\xff\x7f\x00\x00",
            ),
            (
                36_028_797_018_963_968,
                9,
                b"\x80\x80\x80\x80\x80\x80\x80\x80\x01\x00",
            ),
            (
                4_611_686_018_427_387_903,
                9,
                b"\xfe\xff\xff\xff\xff\xff\xff\xff\x7f\x00",
            ),
            (
                4_611_686_018_427_387_904,
                10,
                b"\x80\x80\x80\x80\x80\x80\x80\x80\x80\x01",
            ),
            (i64::MAX, 10, b"\xfe\xff\xff\xff\xff\xff\xff\xff\xff\x01"),
            (-1, 1, b"\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            (-64, 1, b"\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
            (-65, 2, b"\x81\x01\x00\x00\x00\x00\x00\x00\x00\x00"),
            (-8_192, 2, b"\xff\x7f\x00\x00\x00\x00\x00\x00\x00\x00"),
            (-8_193, 3, b"\x81\x80\x01\x00\x00\x00\x00\x00\x00\x00"),
            (-1_048_576, 3, b"\xff\xff\x7f\x00\x00\x00\x00\x00\x00\x00"),
            (-1_048_577, 4, b"\x81\x80\x80\x01\x00\x00\x00\x00\x00\x00"),
            (-134_217_728, 4, b"\xff\xff\xff\x7f\x00\x00\x00\x00\x00\x00"),
            (-134_217_729, 5, b"\x81\x80\x80\x80\x01\x00\x00\x00\x00\x00"),
            (
                -17_179_869_184,
                5,
                b"\xff\xff\xff\xff\x7f\x00\x00\x00\x00\x00",
            ),
            (
                -17_179_869_185,
                6,
                b"\x81\x80\x80\x80\x80\x01\x00\x00\x00\x00",
            ),
            (
                -2_199_023_255_552,
                6,
                b"\xff\xff\xff\xff\xff\x7f\x00\x00\x00\x00",
            ),
            (
                -2_199_023_255_553,
                7,
                b"\x81\x80\x80\x80\x80\x80\x01\x00\x00\x00",
            ),
            (
                -281_474_976_710_656,
                7,
                b"\xff\xff\xff\xff\xff\xff\x7f\x00\x00\x00",
            ),
            (
                -281_474_976_710_657,
                8,
                b"\x81\x80\x80\x80\x80\x80\x80\x01\x00\x00",
            ),
            (
                -36_028_797_018_963_968,
                8,
                b"\xff\xff\xff\xff\xff\xff\xff\x7f\x00\x00",
            ),
            (
                -36_028_797_018_963_969,
                9,
                b"\x81\x80\x80\x80\x80\x80\x80\x80\x01\x00",
            ),
            (
                -4_611_686_018_427_387_904,
                9,
                b"\xff\xff\xff\xff\xff\xff\xff\xff\x7f\x00",
            ),
            (
                -4_611_686_018_427_387_905,
                10,
                b"\x81\x80\x80\x80\x80\x80\x80\x80\x80\x01",
            ),
            (i64::MIN, 10, b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01"),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, super::SInt64>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
