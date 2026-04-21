use crate::{Scalar, scalars::UInt64, wire_types::WIRE_TYPE_VARINT};

impl Scalar<UInt64> for u64 {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_uint64(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_uint64()
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
            (u64::MAX, 10, b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\x01"),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, super::UInt64>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
