use std::net::Ipv4Addr;

use crate::{Scalar, scalars::UInt32};

impl Scalar<UInt32> for Ipv4Addr {
    const WIRE_TYPE: u8 = <u32 as Scalar<UInt32>>::WIRE_TYPE;

    fn encode(&self, encoder: &mut impl crate::Encode) {
        <u32 as Scalar<UInt32>>::encode(&self.to_bits(), encoder);
    }

    fn decode(decoder: &mut impl crate::Decode) -> Result<Self, crate::ProtoError>
    where
        Self: Sized,
    {
        <u32 as Scalar<UInt32>>::decode(decoder).map(std::net::Ipv4Addr::from_bits)
    }
}
