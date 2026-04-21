use crate::{Scalar, scalars::SFixed32, wire_types::WIRE_TYPE_I32};

impl Scalar<SFixed32> for i32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_I32;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_sfixed32(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_sfixed32()
    }
}

#[cfg(test)]
mod test {
    use crate::scalars::SFixed32;

    #[test]
    fn encode_decode() {
        let test_cases = [
            (2i32.pow(0), 4, b"\x01\x00\x00\x00"),
            (2i32.pow(7), 4, b"\x80\x00\x00\x00"),
            (2i32.pow(15), 4, b"\x00\x80\x00\x00"),
            (-2i32.pow(0), 4, b"\xff\xff\xff\xff"),
            (-2i32.pow(7), 4, b"\x80\xff\xff\xff"),
            (-2i32.pow(15), 4, b"\x00\x80\xff\xff"),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, SFixed32>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
