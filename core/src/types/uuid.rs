#[cfg(feature = "uuid_bytes")]
impl crate::Scalar<crate::scalars::Bytes> for uuid::Uuid {
    const WIRE_TYPE: u8 = crate::WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        encoder.encode_bytes(self.as_bytes());
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        let bytes = decoder.decode_bytes()?;
        Self::from_slice(&bytes)
            .map_err(|err| crate::error::ProtoError::Custom(err.to_string()))
    }
}

#[cfg(feature = "uuid_string")]
impl crate::Scalar<crate::scalars::ProtoString> for uuid::Uuid {
    const WIRE_TYPE: u8 = crate::WIRE_TYPE_LENGTH_ENCODED;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        let uuid_string = self.as_simple().to_string();
        <String as crate::Scalar<crate::scalars::ProtoString>>::encode(&uuid_string, encoder);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::error::ProtoError>
    where
        Self: Sized,
    {
        Ok(
            <String as crate::Scalar<crate::scalars::ProtoString>>::decode(decoder)?
                .parse()
                .map_err(|err: uuid::Error| crate::error::ProtoError::Custom(err.to_string()))?,
        )
    }
}

#[cfg(all(test, feature = "uuid_bytes"))]
mod test {
    #[test]
    fn test() {
        use crate::{Scalar, decoder::Decoder, encoder::Encoder};

        let input = uuid::Uuid::from_bytes([
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ]);

        let size_hint = input.size_hint();
        assert_eq!(size_hint, 17);

        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        <uuid::Uuid as Scalar<crate::scalars::Bytes>>::encode(&input, &mut encoder);

        let actual_size = buffer.len();
        assert!(actual_size > 0);
        assert_eq!(actual_size, size_hint);
        assert_eq!(
            &buffer,
            &[
                16, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
                0xdd, 0xee, 0xff,
            ]
        );

        let mut decoder = Decoder::new(&buffer);
        let output = uuid::Uuid::decode(&mut decoder).unwrap();

        assert_eq!(input, output)
    }
}

#[cfg(all(test, feature = "uuid_string"))]
mod test {
    #[test]
    fn test() {
        use crate::{Scalar, decoder::Decoder, encoder::Encoder};

        let input = uuid::Uuid::new_v4();

        let size_hint = input.size_hint();
        let mut buffer = vec![0u8; size_hint];
        let mut encoder = Encoder::new(&mut buffer);
        <uuid::Uuid as Scalar<crate::scalars::ProtoString>>::encode(&input, &mut encoder);

        let actual_size = buffer.len();
        assert!(actual_size > 0);
        assert_eq!(actual_size, size_hint);

        let mut decoder = Decoder::new(&buffer);
        let output = uuid::Uuid::decode(&mut decoder).unwrap();

        assert_eq!(input, output)
    }
}
