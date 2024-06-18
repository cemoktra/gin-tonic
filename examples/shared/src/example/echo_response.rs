#![doc = r"THIS FILE HAS BEEN GENERATED"]
#[allow(unused_imports)]
use ::gin_tonic_core::{Enumeration, Message, OneOf};
#[derive(Clone, Debug, Message)]
pub struct Echo {
    #[gin(tag = 1u32)]
    pub echo: String,
    #[gin(tag = 2u32)]
    pub ip: std::net::Ipv4Addr,
}
#[derive(Clone, Debug, Message)]
pub struct Error {
    #[gin(tag = 1u32)]
    pub code: u32,
}
