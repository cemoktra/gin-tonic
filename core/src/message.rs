use crate::{
    decoder::{Decode, DecodeError},
    encoder::Encode,
};

pub trait EncodeMessage {
    fn size_hint(&self) -> usize;

    fn encode(&self, encoder: &mut impl Encode);
}

pub trait DecodeMessage {
    fn decode(decoder: &mut impl Decode) -> Result<Self, DecodeError>
    where
        Self: Sized;
}
