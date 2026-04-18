use crate::{
    Decode, PackableMarker, Packed, Scalar, Tag, decoder::Decoder,
    wire_types::WIRE_TYPE_LENGTH_ENCODED,
};

impl<RustType, ProtobufType> Packed<ProtobufType> for Vec<RustType>
where
    RustType: Scalar<ProtobufType> + PackableMarker<ProtobufType>,
{
    type Rust = RustType;

    fn encode(&self, field_number: u32, encoder: &mut impl crate::Encode) {
        if self.is_empty() {
            return;
        }

        // packed fields are always length encoded
        encoder.encode_tag(Tag::from_parts(field_number, WIRE_TYPE_LENGTH_ENCODED));

        let mut size = 0;
        for i in self.iter() {
            size += Scalar::<ProtobufType>::size_hint(i);
        }

        encoder.encode_uint64(size as u64);
        for i in self.iter() {
            Scalar::<ProtobufType>::encode(i, encoder);
        }
    }

    fn decode<'buf>(
        field_number: u32,
        raw_message: &'buf crate::RawMessageView<'buf>,
    ) -> Result<Vec<Self::Rust>, crate::error::ProtoError>
    where
        Self: Sized,
    {
        let mut vec = vec![];
        let mut read = 0;

        for buffer in raw_message.tag_data(Tag::from_parts(field_number, WIRE_TYPE_LENGTH_ENCODED))
        {
            let mut decoder = Decoder::new(buffer);
            let size = decoder.decode_uint64()?;

            while read < size {
                let rust_value = Scalar::<ProtobufType>::decode(&mut decoder)?;
                read += Scalar::<ProtobufType>::size_hint(&rust_value) as u64;
                vec.push(rust_value);
            }
        }

        Ok(vec)
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use crate::{PackableMarker, Packed, RawMessageView, Scalar, encoder::Encoder, scalars::*};

    #[test]
    fn packed() {
        fn inner<RustType, ProtobufType>(data: &Vec<RustType>, expected_bytes: &'static [u8])
        where
            RustType: Scalar<ProtobufType> + PackableMarker<ProtobufType> + PartialEq + Debug,
        {
            let size_hint = Packed::<ProtobufType>::size_hint(data, 1);

            let mut buffer = vec![0u8; size_hint];
            let mut encoder = Encoder::new(&mut buffer);
            Packed::<ProtobufType>::encode(data, 1, &mut encoder);

            assert_eq!(size_hint, buffer.len());
            assert_eq!(&expected_bytes[..size_hint], &buffer[..size_hint]);

            let generic_message = RawMessageView::try_from(&buffer[..size_hint]).unwrap();
            let deserialized =
                <Vec<RustType> as Packed<ProtobufType>>::decode(1, &generic_message).unwrap();

            assert_eq!(data, &deserialized)
        }

        inner::<i32, Int32>(
            &vec![1, 2, -3],
            b"\x0a\x0c\x01\x02\xfd\xff\xff\xff\xff\xff\xff\xff\xff\x01",
        );
        inner::<i64, Int64>(
            &vec![1, 2, -3],
            b"\x0a\x0c\x01\x02\xfd\xff\xff\xff\xff\xff\xff\xff\xff\x01",
        );

        inner::<u32, UInt32>(&vec![1, 2, 3], b"\x0a\x03\x01\x02\x03");
        inner::<u64, UInt64>(&vec![1, 2, 3], b"\x0a\x03\x01\x02\x03");

        inner::<i32, SInt32>(&vec![1, 2, -3], b"\x0a\x03\x02\x04\x05");
        inner::<i64, SInt64>(&vec![1, 2, -3], b"\x0a\x03\x02\x04\x05");

        inner::<i32, SFixed32>(
            &vec![1, 2, -3],
            b"\x0a\x0c\x01\x00\x00\x00\x02\x00\x00\x00\xfd\xff\xff\xff",
        );
        inner::<i64, SFixed64 >(
            &vec![1, 2, -3],
            b"\x0a\x18\x01\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\xfd\xff\xff\xff\xff\xff\xff\xff",
        );

        inner::<u32, Fixed32>(
            &vec![1, 2, 3],
            b"\x0a\x0c\x01\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00",
        );
        inner::<u64,Fixed64, >(
            &vec![1, 2, 3],
            b"\x0a\x18\x01\x00\x00\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x00\x03\x00\x00\x00\x00\x00\x00\x00",
        );

        inner::<f32, Float>(
            &vec![1.0, 2.0, -3.0],
            b"\x0a\x0c\x00\x00\x80\x3f\x00\x00\x00\x40\x00\x00\x40\xc0",
        );
        inner::<f64, Double>(
            &vec![1.0, 2.0, -3.0],
            b"\x0a\x18\x00\x00\x00\x00\x00\x00\xf0\x3f\x00\x00\x00\x00\x00\x00\x00\x40\x00\x00\x00\x00\x00\x00\x08\xc0",
        );

        inner::<bool, Bool>(&vec![true, false], b"\x0a\x02\x01\x00");
    }
}
