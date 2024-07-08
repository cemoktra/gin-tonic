use prost::Message;

/// this would normally be generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Prost {
    #[prost(bytes = "vec", tag = "1")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub ip: u32,
}

#[derive(Clone)]
pub struct Rust {
    pub uuid: uuid::Uuid,
    pub ip: std::net::Ipv4Addr,
}

impl From<Prost> for Rust {
    fn from(value: Prost) -> Self {
        let uuid_array: [u8; 16] = value.uuid.try_into().expect("benchmark works");
        Self {
            uuid: uuid::Uuid::from_bytes(uuid_array),
            ip: value.ip.into(),
        }
    }
}

impl From<Rust> for Prost {
    fn from(value: Rust) -> Self {
        Self {
            uuid: value.uuid.as_bytes().to_vec(),
            ip: value.ip.into(),
        }
    }
}

fn main() {
    divan::main();
}

#[divan::bench]
fn ser(bencher: divan::Bencher) {
    let data = Rust {
        uuid: uuid::Uuid::new_v4(),
        ip: std::net::Ipv4Addr::LOCALHOST,
    };

    let prost_data: Prost = data.clone().into();

    let size = prost_data.encoded_len();
    let mut buffer = Vec::with_capacity(size);
    let buffer_ref = &mut buffer;

    bencher.bench_local(move || {
        let prost_data: Prost = data.clone().into();
        prost_data.encode(buffer_ref)
    });
}

#[divan::bench]
fn de(bencher: divan::Bencher) {
    let data = Rust {
        uuid: uuid::Uuid::new_v4(),
        ip: std::net::Ipv4Addr::LOCALHOST,
    };

    let prost_data: Prost = data.clone().into();

    let size = prost_data.encoded_len();

    let mut buffer = bytes::BytesMut::with_capacity(size);
    prost_data.encode(&mut buffer).expect("benchmark works");
    let buffer = buffer.freeze();

    bencher.bench_local(move || {
        let prost_data = Prost::decode(&*buffer).expect("benchmark works");
        let _data: Rust = prost_data.into();
    });
}
