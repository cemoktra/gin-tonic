use std::{array::TryFromSliceError, str::Utf8Error};

use varint_simd::VarIntDecodeError;

#[derive(Debug, thiserror::Error)]
pub enum ProtoError {
    #[error(transparent)]
    VarInt(#[from] VarIntDecodeError),
    #[error(transparent)]
    Utf8(#[from] Utf8Error),
    #[error(transparent)]
    ArrayFromSlice(#[from] TryFromSliceError),
    #[error("Field number {0} is missing")]
    MissingField(u32),
    #[error("No OneOf variants found, expected one of: {0:?}")]
    MissingOneOf(&'static [u32]),
    #[error("{0} is not a known enum variant")]
    UnknownEnumVariant(u32),
    #[error("{0}")]
    Custom(String),
}
