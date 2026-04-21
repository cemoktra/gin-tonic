#![doc = r"THIS FILE HAS BEEN GENERATED"]
#[allow(unused_imports)]
use ::gin_tonic::{Enumeration, Message, OneOf};
#[derive(Clone, Debug, Message)]
pub struct Echo {
    #[gin(id = 1u32)]
    pub echo: String,
    #[gin(id = 2u32)]
    pub ip: std::net::Ipv4Addr,
}
#[derive(Clone, Debug, Message)]
pub struct Error {
    #[gin(id = 1u32, scalar = "uint32")]
    pub code: u32,
}
