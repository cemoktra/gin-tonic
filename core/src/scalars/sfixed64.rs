use crate::{Scalar, scalars::SFixed64, wire_types::WIRE_TYPE_I64};

impl Scalar<SFixed64> for i64 {
    const WIRE_TYPE: u8 = WIRE_TYPE_I64;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_sfixed64(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_sfixed64()
    }
}

#[cfg(test)]
mod test {
    use crate::scalars::SFixed64;

    #[test]
    fn encode_decode() {
        let test_cases = [
            (2i64.pow(0), 8, b"\x01\x00\x00\x00\x00\x00\x00\x00"),
            (2i64.pow(7), 8, b"\x80\x00\x00\x00\x00\x00\x00\x00"),
            (2i64.pow(15), 8, b"\x00\x80\x00\x00\x00\x00\x00\x00"),
            (2i64.pow(31), 8, b"\x00\x00\x00\x80\x00\x00\x00\x00"),
            (2i64.pow(47), 8, b"\x00\x00\x00\x00\x00\x80\x00\x00"),
            (-2i64.pow(0), 8, b"\xff\xff\xff\xff\xff\xff\xff\xff"),
            (-2i64.pow(7), 8, b"\x80\xff\xff\xff\xff\xff\xff\xff"),
            (-2i64.pow(15), 8, b"\x00\x80\xff\xff\xff\xff\xff\xff"),
            (-2i64.pow(31), 8, b"\x00\x00\x00\x80\xff\xff\xff\xff"),
            (-2i64.pow(47), 8, b"\x00\x00\x00\x00\x00\x80\xff\xff"),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, SFixed64>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
