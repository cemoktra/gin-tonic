use crate::{Scalar, wire_types::WIRE_TYPE_LENGTH_ENCODED};

impl Scalar<super::ProtoString> for String {
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_str(self);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        decoder.decode_string()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn encode_decode() {
        let test_cases = [
            (
                String::from("testing"),
                8,
                b"\x07\x74\x65\x73\x74\x69\x6e\x67\x00\x00\x00\x00\x00",
            ),
            (
                String::from("Hello World!"),
                13,
                b"\x0c\x48\x65\x6c\x6c\x6f\x20\x57\x6f\x72\x6c\x64\x21",
            ),
            (
                String::from("protobuf"),
                9,
                b"\x08\x70\x72\x6f\x74\x6f\x62\x75\x66\x00\x00\x00\x00",
            ),
            (
                String::from("gin-tonic"),
                10,
                b"\x09\x67\x69\x6e\x2d\x74\x6f\x6e\x69\x63\x00\x00\x00",
            ),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            crate::scalars::test_scalar_encode_decode::<_, crate::scalars::ProtoString>(
                value,
                expected_size,
                expected_bytes,
            );
        }
    }
}
