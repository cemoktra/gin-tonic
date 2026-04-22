use crate::{Scalar, Unpacked};

impl<RustType, ProtobufType> Unpacked<ProtobufType> for Vec<RustType>
where
    RustType: Scalar<ProtobufType>,
{
    type Rust = RustType;

    #[inline]
    fn encode(&self, tag: crate::Tag, encoder: &mut impl crate::Encode) {
        if self.is_empty() {
            return;
        }

        for i in self.iter() {
            encoder.encode_tag(tag);
            <RustType as Scalar<ProtobufType>>::encode(i, encoder);
        }
    }
}

// #[cfg(test)]
// mod test {
//     use std::fmt::Debug;

//     use crate::{RawMessageView, Scalar, Tag, Unpacked, encoder::Encoder, scalars::*};

//     #[test]
//     fn unpacked() {
//         fn inner<RustType, ProtobufType>(data: &Vec<RustType>, expected_bytes: &'static [u8])
//         where
//             RustType: Scalar<ProtobufType> + PartialEq + Debug,
//         {
//             let tag = Tag::from_parts(1, RustType::WIRE_TYPE);
//             let size_hint = Unpacked::<ProtobufType>::size_hint(data, tag);

//             let mut buffer = vec![0u8; size_hint];
//             let mut encoder = Encoder::new(&mut buffer);
//             Unpacked::<ProtobufType>::encode(data, tag, &mut encoder);

//             assert_eq!(size_hint, buffer.len());
//             assert_eq!(&expected_bytes[..size_hint], &buffer[..size_hint]);

//             let generic_message = RawMessageView::try_from(&buffer[..size_hint]).unwrap();
//             let deserialized =
//                 <Vec<RustType> as Unpacked<ProtobufType>>::decode(tag, &generic_message).unwrap();

//             assert_eq!(data, &deserialized)
//         }

//         inner::<i32, Int32>(
//             &vec![1, 2, -3],
//             b"\x08\x01\x08\x02\x08\xfd\xff\xff\xff\xff\xff\xff\xff\xff\x01",
//         );
//         inner::<i64, Int64>(
//             &vec![1, 2, -3],
//             b"\x08\x01\x08\x02\x08\xfd\xff\xff\xff\xff\xff\xff\xff\xff\x01",
//         );

//         inner::<u32, UInt32>(&vec![1, 2, 3], b"\x08\x01\x08\x02\x08\x03");
//         inner::<u64, UInt64>(&vec![1, 2, 3], b"\x08\x01\x08\x02\x08\x03");

//         inner::<i32, SInt32>(&vec![1, 2, -3], b"\x08\x02\x08\x04\x08\x05");
//         inner::<i64, SInt64>(&vec![1, 2, -3], b"\x08\x02\x08\x04\x08\x05");

//         inner::<i32, SFixed32>(
//             &vec![1, 2, -3],
//             b"\x0d\x01\x00\x00\x00\x0d\x02\x00\x00\x00\x0d\xfd\xff\xff\xff",
//         );
//         inner::<i64,SFixed64>(&vec![1, 2, -3], b"\x09\x01\x00\x00\x00\x00\x00\x00\x00\x09\x02\x00\x00\x00\x00\x00\x00\x00\x09\xfd\xff\xff\xff\xff\xff\xff\xff");

//         inner::<u32, Fixed32>(
//             &vec![1, 2, 3],
//             b"\x0d\x01\x00\x00\x00\x0d\x02\x00\x00\x00\x0d\x03\x00\x00\x00",
//         );
//         inner::<u64,Fixed64>(&vec![1, 2, 3],  b"\x09\x01\x00\x00\x00\x00\x00\x00\x00\x09\x02\x00\x00\x00\x00\x00\x00\x00\x09\x03\x00\x00\x00\x00\x00\x00\x00",);

//         inner::<f32, Float>(
//             &vec![1.0, 2.0, -3.0],
//             b"\x0d\x00\x00\x80\x3f\x0d\x00\x00\x00\x40\x0d\x00\x00\x40\xc0",
//         );
//         inner::<f64, Double>(
//             &vec![1.0, 2.0, -3.0],
//             b"\x09\x00\x00\x00\x00\x00\x00\xf0\x3f\x09\x00\x00\x00\x00\x00\x00\x00\x40\x09\x00\x00\x00\x00\x00\x00\x08\xc0",
//         );

//         inner::<bool, Bool>(&vec![true, false], b"\x08\x01\x08\x00");

//         let data = vec![String::from("hello"), String::from("world")];
//         inner::<String, ProtoString>(
//             data.as_ref(),
//             b"\x0a\x05\x68\x65\x6c\x6c\x6f\x0a\x05\x77\x6f\x72\x6c\x64",
//         );
//     }
// }
