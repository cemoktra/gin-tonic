#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
use crate::Scalar;

mod bool;
mod bytes;
mod f32;
mod f64;
mod fixed32;
mod fixed64;
mod int32;
mod int64;
mod sfixed32;
mod sfixed64;
mod sint32;
mod sint64;
mod string;
mod uint32;
mod uint64;

pub struct Int32;

pub struct Int64;

pub struct UInt32;

pub struct UInt64;

pub struct SInt32;

pub struct SInt64;

pub struct Fixed32;

pub struct Fixed64;

pub struct SFixed32;

pub struct SFixed64;

pub struct Float;

pub struct Double;

pub struct Bool;

pub struct ProtoString;

pub struct Bytes;

#[cfg(test)]
fn test_scalar_encode_decode<Rust, Proto>(value: Rust, expected_size: usize, expected_bytes: &[u8])
where
    Rust: Scalar<Proto> + PartialEq + Debug,
{
    use crate::{decoder::Decoder, encoder::Encoder};

    let size_hint = value.size_hint();
    assert_eq!(size_hint, expected_size);

    let mut buffer = vec![0u8; size_hint];
    let mut encoder = Encoder::new(buffer.as_mut_slice());
    value.encode(&mut encoder);
    assert_eq!(size_hint, buffer.len());
    assert_eq!(
        &expected_bytes[..size_hint],
        &buffer[..size_hint],
        "{value:?} expected to encode as {} but did encode to {}",
        hex::encode(&expected_bytes[..size_hint]),
        hex::encode(&buffer[..size_hint])
    );

    let mut decoder = Decoder::new(&buffer);
    let deserialized = Rust::decode(&mut decoder).unwrap();
    assert_eq!(value, deserialized)
}
