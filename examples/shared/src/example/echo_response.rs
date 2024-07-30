#![doc = r"THIS FILE HAS BEEN GENERATED"]
#[allow(unused_imports)]
use ::gin_tonic::{Enumeration, Message, OneOf};
#[derive(Clone, Debug, Message)]
pub struct Echo {
    #[gin(tag = 1u32, proto = "string")]
    pub echo: String,
    #[gin(tag = 2u32)]
    pub ip: std::net::Ipv4Addr,
}
#[derive(Clone, Debug, Message)]
pub struct Error {
    #[gin(tag = 1u32, proto = "uint32")]
    pub code: u32,
}
