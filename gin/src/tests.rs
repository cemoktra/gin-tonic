use gin_tonic_core::{
    export::VarInt,
    protobuf::{Error, FromWire, IntoWire, Message, WireType, WireTypeView},
};
use std::collections::HashMap;
use std::ops::Deref;

// in order to implFromWire/IntoWire for foreign types
#[derive(Debug)]
struct IpWrapper(std::net::Ipv4Addr);

impl From<std::net::Ipv4Addr> for IpWrapper {
    fn from(value: std::net::Ipv4Addr) -> Self {
        Self(value)
    }
}

impl From<IpWrapper> for std::net::Ipv4Addr {
    fn from(value: IpWrapper) -> Self {
        value.0
    }
}

impl Deref for IpWrapper {
    type Target = std::net::Ipv4Addr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromWire for IpWrapper {
    fn from_wire(wire: WireTypeView) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let ip = match wire {
            WireTypeView::VarInt(data) => {
                let (ip, _) = u32::decode_var(data).ok_or(Error::InvalidVarInt)?;
                ip
            }
            WireTypeView::FixedI32(data) => {
                let array: [u8; 4] = data.try_into().expect("I32 is always 4 bytes");
                u32::from_be_bytes(array)
            }
            _ => return Err(Error::UnexpectedWireType),
        };

        Ok(std::net::Ipv4Addr::from(ip).into())
    }
}

impl IntoWire for IpWrapper {
    fn into_wire(self) -> WireType {
        let mut data = [0u8; 10];
        let ip: u32 = self.0.into();
        let size = ip.encode_var(&mut data);
        WireType::VarInt(data, size)
    }

    fn size_hint(&self, tag: u32) -> usize {
        let ip: u32 = self.0.into();
        ip.required_space() + tag.required_space()
    }
}

#[derive(Debug, gin_tonic_core::Message)]
struct Test {
    #[gin(tag = 1)]
    ip: IpWrapper,
    #[gin(tag = 2, cardinality = "optional")]
    port: Option<u32>,
    #[gin(tag = 3, cardinality = "repeated")]
    protocols: Vec<String>,
    #[gin(tag = 4, kind = "message")]
    nested: Nested,
    //#[gin(tag = 5)]
    // logging: Logging,
    // // 6 + 7
    //#[gin(tag = 0)]
    // oneof: Oneof,
    //#[gin(tag = 8)]
    // map: HashMap<u32, String>,
}

#[derive(Debug, gin_tonic_core::Message)]
struct Nested {
    #[gin(tag = 1)]
    number: i32,
}

// #[derive(Clone, Copy, Debug, Eq, PartialEq)]
// enum Logging {
//     Human = 1,
//     Json = 2,
// }
//
// #[derive(Clone, Debug, Eq, PartialEq)]
// enum Oneof {
//     Num(i32),
//     Str(String),
// }

#[test]
fn pb_serde() {
    let test = Test {
        ip: std::net::Ipv4Addr::LOCALHOST.into(),
        port: None,
        protocols: vec![],
        nested: Nested { number: -1 },
        // logging: Logging::Human,
        // oneof: Oneof::Num(123),
        // map: HashMap::new(),
    };

    let size_hint = Message::size_hint(&test);
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();
    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let test = Test::deserialize(&buffer).unwrap();

    assert_eq!(
        Into::<std::net::Ipv4Addr>::into(test.ip),
        std::net::Ipv4Addr::LOCALHOST
    );
    assert!(test.port.is_none());
    assert!(test.protocols.is_empty());
    assert_eq!(test.nested.number, -1);
    // assert_eq!(test.logging, Logging::Human);
    // match test.oneof {
    //     Oneof::Num(n) => assert_eq!(n, 123),
    //     _ => panic!("wrong oneof"),
    // }
    // assert!(test.map.is_empty());

    // first round with optional field set to Some
    let mut map = HashMap::new();
    map.insert(10, String::from("ten"));
    map.insert(20, String::from("twenty"));
    let test = Test {
        ip: std::net::Ipv4Addr::LOCALHOST.into(),
        port: Some(8080),
        protocols: vec![String::from("tcp"), String::from("udp")],
        nested: Nested { number: -1 },
        // logging: Logging::Json,
        // oneof: Oneof::Str(String::from("hello")),
        // map,
    };
    let size_hint = Message::size_hint(&test);
    let mut buffer = Vec::<u8>::with_capacity(size_hint);

    let actual_size = test.serialize(&mut buffer).unwrap();

    assert!(actual_size > 0);
    assert_eq!(actual_size, size_hint);

    let test = Test::deserialize(&buffer).unwrap();

    assert_eq!(
        Into::<std::net::Ipv4Addr>::into(test.ip),
        std::net::Ipv4Addr::LOCALHOST
    );
    assert_eq!(test.port, Some(8080));
    assert_eq!(test.protocols.len(), 2);
    assert_eq!(test.nested.number, -1);
    // assert_eq!(test.logging, Logging::Json);
    // match test.oneof {
    //     Oneof::Str(s) => assert_eq!(s, "hello"),
    //     _ => panic!("wrong oneof"),
    // }
    // assert_eq!(test.map.len(), 2);
}
