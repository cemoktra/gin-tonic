use std::{array::TryFromSliceError, string::FromUtf8Error};

use varint_simd::VarIntDecodeError;

#[derive(Debug, thiserror::Error)]
pub enum ProtoError {
    #[error(transparent)]
    VarInt(#[from] VarIntDecodeError),
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
    #[error(transparent)]
    ArrayFromSlice(#[from] TryFromSliceError),

    #[error("Field number {0} is missing")]
    MissingField(u32),
}
