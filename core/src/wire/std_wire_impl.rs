//! [FromWire] and [IntoWire] for Rust std types

use crate::{Error, FromWire, IntoWire, WireType, WireTypeView};

impl FromWire for std::net::Ipv4Addr {
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let n = u32::from_wire(wire)?;
        Ok(n.into())
    }
}

impl IntoWire for std::net::Ipv4Addr {
    fn into_wire(self) -> WireType {
        let n: u32 = self.into();
        n.into_wire()
    }

    fn size_hint(&self, tag: u32) -> usize {
        let n: u32 = (*self).into();
        n.size_hint(tag)
    }
}

impl FromWire for std::path::PathBuf {
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let path = String::from_wire(wire)?;
        Ok(path.into())
    }
}

impl IntoWire for std::path::PathBuf {
    fn into_wire(self) -> WireType {
        let path: String = self.display().to_string();
        path.into_wire()
    }

    fn size_hint(&self, tag: u32) -> usize {
        let path: String = self.display().to_string();
        path.size_hint(tag)
    }
}
