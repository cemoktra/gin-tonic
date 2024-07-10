//! [FromWire] and [IntoWire] for [uuid::Uuid]

#[cfg(all(feature = "uuid_string", feature = "uuid_bytes"))]
compile_error!("cannot use 'uuid_string' and 'uuid_bytes' at the same time");

#[cfg(feature = "uuid_string")]
mod uuid_string {
    use crate::{Error, WireType, WireTypeView};
    use uuid::Uuid;

    impl crate::IntoWire for Uuid {
        fn into_wire(self) -> WireType {
            let simple = self.as_simple().to_string();
            simple.into_wire()
        }

        fn size_hint(&self, tag: u32) -> usize {
            let simple = self.as_simple().to_string();
            simple.size_hint(tag)
        }
    }

    impl crate::FromWire for Uuid {
        fn from_wire(wire: WireTypeView) -> Result<Self, Error>
        where
            Self: Sized,
        {
            let simple = String::from_wire(wire)?;
            simple
                .parse()
                .map_err(|err| Error::Conversion(Box::new(err)))
        }
    }

    #[cfg(test)]
    mod test {
        use crate::{FromWire, IntoWire};
        use uuid::Uuid;

        #[test]
        fn wire_conversion() {
            let new_uuid = Uuid::new_v4();
            let wire = new_uuid.into_wire();
            assert_eq!(34, wire.size_hint(1));
            assert_eq!(new_uuid.size_hint(1), wire.size_hint(1));
            let wire_uuid = Uuid::from_wire(wire.as_view()).unwrap();
            assert_eq!(new_uuid, wire_uuid);
        }
    }
}

#[cfg(feature = "uuid_bytes")]
mod uuid_bytes {
    use crate::{Error, VarInt, WireType, WireTypeView};
    use uuid::Uuid;

    impl crate::IntoWire for Uuid {
        fn into_wire(self) -> WireType {
            let mut buffer = [0u8; 20];
            let (high, low) = self.as_u64_pair();

            let high_len = high.encode_var(&mut buffer[0..]);
            let low_len = low.encode_var(&mut buffer[high_len..]);

            WireType::LengthEncoded(bytes::Bytes::copy_from_slice(
                &buffer[0..high_len + low_len],
            ))
        }

        fn size_hint(&self, tag: u32) -> usize {
            let (high, low) = self.as_u64_pair();
            let data_len = high.required_space() + low.required_space();
            tag.required_space() + data_len.required_space() + data_len
        }
    }

    impl crate::FromWire for Uuid {
        fn from_wire(wire: WireTypeView) -> Result<Self, Error>
        where
            Self: Sized,
        {
            match wire {
                WireTypeView::LengthEncoded(data) => {
                    let (high, read) = u64::decode_var(data).ok_or(Error::InvalidVarInt)?;
                    let (low, _) = u64::decode_var(&data[read..]).ok_or(Error::InvalidVarInt)?;
                    Ok(Uuid::from_u64_pair(high, low))
                }
                _ => Err(Error::UnexpectedWireType),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use crate::{FromWire, IntoWire};
        use uuid::Uuid;

        #[test]
        fn wire_conversion() {
            let new_uuid = Uuid::new_v4();
            let wire = new_uuid.into_wire();
            assert!(wire.size_hint(1) <= 22);
            assert_eq!(new_uuid.size_hint(1), wire.size_hint(1));
            let wire_uuid = Uuid::from_wire(wire.as_view()).unwrap();
            assert_eq!(new_uuid, wire_uuid);
        }
    }
}
