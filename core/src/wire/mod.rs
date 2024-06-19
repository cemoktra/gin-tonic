pub mod map;
pub mod nested;
mod scalars;
mod std_wire_impl;
pub mod wire_type;

/// error enumeration for problems occuring when converting a [WireTypeView] into an actual type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("unexpected wire type")]
    UnexpectedWireType,
    #[error("invalid var int")]
    InvalidVarInt,
    #[error("field with number {0} is missing")]
    MissingField(u32),
    #[error(transparent)]
    Conversion(Box<dyn std::error::Error>),
    #[error("enum variant {0} is not known")]
    UnknownEnumVariant(u32),
    #[error("invalid one of")]
    InvalidOneOf,
}

/// convert a [WireTypeView] into an actual Rust type
pub trait FromWire {
    fn from_wire(wire: wire_type::WireTypeView) -> Result<Self, Error>
    where
        Self: Sized;
}

/// convert a Rust type into a [WireType]
pub trait IntoWire {
    fn into_wire(self) -> wire_type::WireType;
    fn size_hint(&self, tag: u32) -> usize;
}

#[cfg(test)]
mod test;
