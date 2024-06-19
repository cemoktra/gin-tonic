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
