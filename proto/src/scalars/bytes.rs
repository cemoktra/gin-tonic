use crate::{Scalar, wire_types::WIRE_TYPE_LENGTH_ENCODED};

impl Scalar<super::Bytes> for Vec<u8> {
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_bytes(self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        Ok(decoder.decode_bytes()?.to_vec())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn encode_decode() {
        let test_cases = [
            (
                vec![
                    72u8, 101u8, 108u8, 108u8, 111u8, 32u8, 87u8, 111u8, 114u8, 108u8, 100u8, 33u8,
                ],
                13,
                b"\x0c\x48\x65\x6c\x6c\x6f\x20\x57\x6f\x72\x6c\x64\x21",
            ),
            (
                vec![112u8, 114u8, 111u8, 116u8, 111u8, 98u8, 117u8, 102u8],
                9,
                b"\x08\x70\x72\x6f\x74\x6f\x62\x75\x66\x00\x00\x00\x00",
            ),
            (
                vec![103u8, 105u8, 110u8, 45u8, 116u8, 111u8, 110u8, 105u8, 99u8],
                10,
                b"\x09\x67\x69\x6e\x2d\x74\x6f\x6e\x69\x63\x00\x00\x00",
            ),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, crate::scalars::Bytes>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
