use crate::{Scalar, scalars::Bool, wire_types::WIRE_TYPE_VARINT};

impl Scalar<Bool> for bool {
    const WIRE_TYPE: u8 = WIRE_TYPE_VARINT;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_bool(*self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_bool()
    }
}

#[cfg(test)]
mod test {
    use crate::scalars::Bool;

    #[test]
    fn encode_decode() {
        let test_cases = [(false, 1, b"\x00"), (true, 1, b"\x01")];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, Bool>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
