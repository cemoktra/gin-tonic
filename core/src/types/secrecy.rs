use secrecy::ExposeSecret;

use crate::WIRE_TYPE_LENGTH_ENCODED;

impl crate::Scalar<crate::scalars::ProtoString> for secrecy::SecretBox<str> {
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_str(self.expose_secret());
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::ProtoError>
    where
        Self: Sized,
    {
        Ok(decoder.decode_string()?.into())
    }
}

impl crate::Scalar<crate::scalars::Bytes> for secrecy::SecretBox<[u8]> {
    const WIRE_TYPE: u8 = WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_bytes(self.expose_secret());
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::ProtoError>
    where
        Self: Sized,
    {
        Ok(decoder.decode_bytes()?.into())
    }
}

#[cfg(test)]
mod test {
    use secrecy::{ExposeSecret, SecretBox};

    use crate::Scalar;

    #[test]
    fn encode_decode_secret_string() {
        let test_cases = [
            (
                SecretBox::<str>::from("testing"),
                8,
                b"\x07\x74\x65\x73\x74\x69\x6e\x67\x00\x00\x00\x00\x00",
            ),
            (
                SecretBox::<str>::from("Hello World!"),
                13,
                b"\x0c\x48\x65\x6c\x6c\x6f\x20\x57\x6f\x72\x6c\x64\x21",
            ),
            (
                SecretBox::<str>::from("protobuf"),
                9,
                b"\x08\x70\x72\x6f\x74\x6f\x62\x75\x66\x00\x00\x00\x00",
            ),
            (
                SecretBox::<str>::from("gin-tonic"),
                10,
                b"\x09\x67\x69\x6e\x2d\x74\x6f\x6e\x69\x63\x00\x00\x00",
            ),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            use crate::{decoder::Decoder, encoder::Encoder};

            let size_hint = value.size_hint();
            assert_eq!(size_hint, expected_size);

            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(buffer.as_mut_slice());
            value.encode(&mut encoder);
            assert_eq!(size_hint, buffer.len());
            assert_eq!(
                &expected_bytes[..size_hint],
                &buffer[..size_hint],
                "{value:?} expected to encode as {} but did encode to {}",
                hex::encode(&expected_bytes[..size_hint]),
                hex::encode(&buffer[..size_hint])
            );

            let mut decoder = Decoder::new(&buffer);
            let deserialized = SecretBox::<str>::decode(&mut decoder).unwrap();
            assert_eq!(value.expose_secret(), deserialized.expose_secret())
        }
    }

    #[test]
    fn encode_decode_secret_box_u8() {
        let test_cases = [
            (
                SecretBox::<[u8]>::from(vec![
                    72u8, 101u8, 108u8, 108u8, 111u8, 32u8, 87u8, 111u8, 114u8, 108u8, 100u8, 33u8,
                ]),
                13,
                b"\x0c\x48\x65\x6c\x6c\x6f\x20\x57\x6f\x72\x6c\x64\x21",
            ),
            (
                SecretBox::<[u8]>::from(vec![
                    112u8, 114u8, 111u8, 116u8, 111u8, 98u8, 117u8, 102u8,
                ]),
                9,
                b"\x08\x70\x72\x6f\x74\x6f\x62\x75\x66\x00\x00\x00\x00",
            ),
            (
                SecretBox::<[u8]>::from(vec![
                    103u8, 105u8, 110u8, 45u8, 116u8, 111u8, 110u8, 105u8, 99u8,
                ]),
                10,
                b"\x09\x67\x69\x6e\x2d\x74\x6f\x6e\x69\x63\x00\x00\x00",
            ),
        ];

        for (value, expected_size, expected_bytes) in test_cases {
            use crate::{decoder::Decoder, encoder::Encoder};

            let size_hint = value.size_hint();
            assert_eq!(size_hint, expected_size);

            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(buffer.as_mut_slice());
            value.encode(&mut encoder);
            assert_eq!(size_hint, buffer.len());
            assert_eq!(
                &expected_bytes[..size_hint],
                &buffer[..size_hint],
                "{value:?} expected to encode as {} but did encode to {}",
                hex::encode(&expected_bytes[..size_hint]),
                hex::encode(&buffer[..size_hint])
            );

            let mut decoder = Decoder::new(&buffer);
            let deserialized = SecretBox::<[u8]>::decode(&mut decoder).unwrap();
            assert_eq!(value.expose_secret(), deserialized.expose_secret())
        }
    }
}
