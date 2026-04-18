use crate::{Scalar, scalars::Float, wire_types::WIRE_TYPE_I32};

impl Scalar<Float> for f32 {
    const WIRE_TYPE: u8 = WIRE_TYPE_I32;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_float(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_float()
    }
}

#[cfg(test)]
mod test {
    use core::f32;

    use crate::scalars::Float;

    #[test]
    fn encode_decode() {
        let test_cases = [
            (f32::MIN, 4, b"\xff\xff\x7f\xff"),
            (f32::MAX, 4, b"\xff\xff\x7f\x7f"),
            (f32::EPSILON, 4, b"\x00\x00\x00\x34"),
            (f32::consts::PI, 4, b"\xdb\x0f\x49\x40"),
            (f32::consts::E, 4, b"\x54\xf8\x2d\x40"),
            (f32::consts::TAU, 4, b"\xdb\x0f\xc9\x40"),
            (f32::consts::LOG2_E, 4, b"\x3b\xaa\xb8\x3f"),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, Float>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
