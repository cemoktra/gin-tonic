use crate::{Scalar, scalars::Double, wire_types::WIRE_TYPE_I64};

impl Scalar<Double> for f64 {
    const WIRE_TYPE: u8 = WIRE_TYPE_I64;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_double(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_double()
    }
}

#[cfg(test)]
mod test {
    use core::f64;

    use crate::scalars::Double;

    #[test]
    fn encode_decode() {
        let test_cases = [
            (f64::MIN, 8, b"\xff\xff\xff\xff\xff\xff\xef\xff"),
            (f64::MAX, 8, b"\xff\xff\xff\xff\xff\xff\xef\x7f"),
            (f64::EPSILON, 8, b"\x00\x00\x00\x00\x00\x00\xb0\x3c"),
            (f64::consts::PI, 8, b"\x18\x2d\x44\x54\xfb\x21\x09\x40"),
            (f64::consts::E, 8, b"\x69\x57\x14\x8b\x0a\xbf\x05\x40"),
            (f64::consts::TAU, 8, b"\x18\x2d\x44\x54\xfb\x21\x19\x40"),
            (f64::consts::LOG2_E, 8, b"\xfe\x82\x2b\x65\x47\x15\xf7\x3f"),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, Double>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
