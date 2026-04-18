use crate::{Scalar, scalars::Int64, wire_types::WIRE_TYPE_VARINT};

impl Scalar<Int64> for i64 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_int64(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_int64()
    }
}

#[cfg(test)]
mod test {
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
            (
                34_359_738_367,
                5,
                b"\xff\xff\xff\xff\x7f\x00\x00\x00\x00\x00",
            ),
            (
                34_359_738_368,
                6,
                b"\x80\x80\x80\x80\x80\x01\x00\x00\x00\x00",
            ),
            (
                4_398_046_511_103,
                6,
                b"\xff\xff\xff\xff\xff\x7f\x00\x00\x00\x00",
            ),
            (
                4_398_046_511_104,
                7,
                b"\x80\x80\x80\x80\x80\x80\x01\x00\x00\x00",
            ),
            (
                562_949_953_421_311,
                7,
                b"\xff\xff\xff\xff\xff\xff\x7f\x00\x00\x00",
            ),
            (
                562_949_953_421_312,
                8,
                b"\x80\x80\x80\x80\x80\x80\x80\x01\x00\x00",
            ),
            (
                72_057_594_037_927_935,
                8,
                b"\xff\xff\xff\xff\xff\xff\xff\x7f\x00\x00",
            ),
            (
                72_057_594_037_927_936,
                9,
                b"\x80\x80\x80\x80\x80\x80\x80\x80\x01\x00",
            ),
            (i64::MAX, 9, b"\xff\xff\xff\xff\xff\xff\xff\xff\x7f\x00"),
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
            (
                -34_359_738_368,
                10,
                b"\x80\x80\x80\x80\x80\xff\xff\xff\xff\x01",
            ),
            (
                -34_359_738_369,
                10,
                b"\xff\xff\xff\xff\xff\xfe\xff\xff\xff\x01",
            ),
            (
                -4_398_046_511_104,
                10,
                b"\x80\x80\x80\x80\x80\x80\xff\xff\xff\x01",
            ),
            (
                -4_398_046_511_105,
                10,
                b"\xff\xff\xff\xff\xff\xff\xfe\xff\xff\x01",
            ),
            (
                -562_949_953_421_312,
                10,
                b"\x80\x80\x80\x80\x80\x80\x80\xff\xff\x01",
            ),
            (
                -562_949_953_421_313,
                10,
                b"\xff\xff\xff\xff\xff\xff\xff\xfe\xff\x01",
            ),
            (
                -72057594037927935,
                10,
                b"\x81\x80\x80\x80\x80\x80\x80\x80\xff\x01",
            ),
            (
                -72057594037927936,
                10,
                b"\x80\x80\x80\x80\x80\x80\x80\x80\xff\x01",
            ),
            (i64::MIN, 10, b"\x80\x80\x80\x80\x80\x80\x80\x80\x80\x01"),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, super::Int64>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
