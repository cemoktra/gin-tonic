use crate::{Scalar, scalars::Fixed32, wire_types::WIRE_TYPE_I32};

impl Scalar<Fixed32> for u32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_I32;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_fixed32(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_fixed32()
    }
}

#[cfg(test)]
mod test {
    use crate::scalars::Fixed32;

    #[test]
    fn encode_decode() {
        let test_cases = [
            (2u32.pow(0), 4, b"\x01\x00\x00\x00"),
            (2u32.pow(7), 4, b"\x80\x00\x00\x00"),
            (2u32.pow(15), 4, b"\x00\x80\x00\x00"),
            (2u32.pow(31), 4, b"\x00\x00\x00\x80"),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, Fixed32>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
