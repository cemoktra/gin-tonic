#[cfg(all(feature = "uuid-u64-pair", feature = "uuid-string"))]
compile_error!("'uuid-u64-pair' and 'uuid-string' are mutually exclusive");

#[cfg(feature = "uuid-u64-pair")]
mod uuid_u64_pair {
    use crate::{Error, TagReader, WireType, WireTypeView};
    use integer_encoding::VarInt;
    use std::collections::HashMap;
    use uuid::Uuid;

    impl crate::IntoWire for Uuid {
        fn into_wire(self) -> WireType {
            let (high, low) = self.as_u64_pair();

            let mut map_buffer = Vec::with_capacity(high.size_hint(1) + low.size_hint(2));

            let wire_type = high.into_wire();
            wire_type
                .serialize(1, &mut map_buffer)
                .expect("buffer has correct size");

            let wire_type = low.into_wire();
            wire_type
                .serialize(2, &mut map_buffer)
                .expect("buffer has correct size");

            WireType::LengthEncoded(map_buffer)
        }

        fn size_hint(&self, tag: u32) -> usize {
            let (high, low) = self.as_u64_pair();
            let size = high.size_hint(1) + low.size_hint(2);
            size + size.required_space() + tag.required_space()
        }
    }

    impl crate::FromWire for Uuid {
        fn from_wire(wire: WireTypeView) -> Result<Self, Error>
        where
            Self: Sized,
        {
            match wire {
                WireTypeView::LengthEncoded(data) => {
                    let reader = TagReader::new(data);
                    let mut field_map = HashMap::<u32, Vec<WireTypeView>>::new();

                    for tag in reader {
                        let (field_number, wire_type) = tag.into_parts();
                        field_map.entry(field_number).or_default().push(wire_type);
                    }

                    let high = field_map
                        .remove(&1)
                        .ok_or(Error::MissingField(1))?
                        .into_iter()
                        .next()
                        .ok_or(Error::MissingField(1))?;
                    let high = u64::from_wire(high)?;

                    let low = field_map
                        .remove(&2)
                        .ok_or(Error::MissingField(2))?
                        .into_iter()
                        .next()
                        .ok_or(Error::MissingField(2))?;
                    let low = u64::from_wire(low)?;

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
            assert!(wire.size_hint(1) <= 24);
            let wire_uuid = Uuid::from_wire(wire.as_view()).unwrap();
            assert_eq!(new_uuid, wire_uuid);
        }
    }
}

#[cfg(feature = "uuid-string")]
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
            let wire_uuid = Uuid::from_wire(wire.as_view()).unwrap();
            assert_eq!(new_uuid, wire_uuid);
        }
    }
}
