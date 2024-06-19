use bytes::{Buf, BufMut};
use std::marker::PhantomData;
use tonic::codec::{DecodeBuf, EncodeBuf};

use crate::protobuf::Message;

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
    T: Message + Send + 'static + std::fmt::Debug,
    U: Message + Send + 'static + std::fmt::Debug,
{
    type Encode = T;
    type Decode = U;
    type Encoder = GinEncoder<T>;
    type Decoder = GinDecoder<U>;

    fn encoder(&mut self) -> Self::Encoder {
        GinEncoder {
            _pd: PhantomData::default(),
        }
    }

    fn decoder(&mut self) -> Self::Decoder {
        GinDecoder {
            _pd: PhantomData::default(),
        }
    }
}

impl<T: Message + std::fmt::Debug> tonic::codec::Encoder for GinEncoder<T> {
    type Item = T;
    type Error = tonic::Status;

    fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        // TODO: may rewrite Message trait to use bytes::BufMut
        println!("{item:?}");
        item.serialize(&mut dst.writer())?;
        println!("{dst:?}");
        Ok(())
    }
}

impl<U: Message + std::fmt::Debug> tonic::codec::Decoder for GinDecoder<U> {
    type Item = U;
    type Error = tonic::Status;

    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        // TODO: may rewrite Message trait to use bytes::Mut
        println!("{src:?}");
        let x = Self::Item::deserialize(src.chunk())?;
        println!("{x:?}");
        Ok(Some(Self::Item::deserialize(src.chunk())?))
    }
}
