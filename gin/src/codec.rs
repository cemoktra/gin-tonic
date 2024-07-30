//! [tonic::codec::Codec] implementation for gin-tonic

use std::marker::PhantomData;
use tonic::codec::{DecodeBuf, EncodeBuf};

use gin_tonic_core::types::PbType;

#[derive(Debug, Clone)]
pub struct GinCodec<T, U> {
    _pd: PhantomData<(T, U)>,
}

impl<T, U> Default for GinCodec<T, U> {
    fn default() -> Self {
        Self { _pd: PhantomData }
    }
}

#[derive(Debug, Clone)]
pub struct GinEncoder<T> {
    _pd: PhantomData<T>,
}

#[derive(Debug, Clone)]
pub struct GinDecoder<U> {
    _pd: PhantomData<U>,
}

impl<T, U> tonic::codec::Codec for GinCodec<T, U>
where
    T: PbType + Send + 'static + std::fmt::Debug,
    U: PbType + Send + 'static + std::fmt::Debug,
{
    type Encode = T;
    type Decode = U;
    type Encoder = GinEncoder<T>;
    type Decoder = GinDecoder<U>;

    fn encoder(&mut self) -> Self::Encoder {
        GinEncoder { _pd: PhantomData }
    }

    fn decoder(&mut self) -> Self::Decoder {
        GinDecoder { _pd: PhantomData }
    }
}

impl<T: PbType + std::fmt::Debug> tonic::codec::Encoder for GinEncoder<T> {
    type Item = T;
    type Error = tonic::Status;

    fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        item.encode(dst);
        Ok(())
    }
}

impl<U: PbType + std::fmt::Debug> tonic::codec::Decoder for GinDecoder<U> {
    type Item = U;
    type Error = tonic::Status;

    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        let decoded = Self::Item::decode(src).map_err(map_core_err)?;
        Ok(Some(decoded))
    }
}

fn map_core_err(err: gin_tonic_core::DecodeError) -> tonic::Status {
    tonic::Status::internal(err.to_string())
}
