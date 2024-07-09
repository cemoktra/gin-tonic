use std::{collections::HashMap, path::PathBuf};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prost::Message;

criterion_main!(benches);

/// this would normally be generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Prost {
    #[prost(bytes = "vec", tag = "1")]
    pub uuid: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2", repeated)]
    pub ip: Vec<u32>,
    #[prost(string, tag = "3")]
    pub text: String,
    #[prost(message, tag = "4")]
    pub nested: Option<ProstNested>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProstNested {
    #[prost(map = "string, uint64", tag = "1")]
    pub counts: HashMap<String, u64>,
}

#[derive(Clone)]
pub struct Rust {
    pub uuid: uuid::Uuid,
    pub ip: Vec<std::net::Ipv4Addr>,
    pub text: String,
    pub nested: RustNested,
}

#[derive(Clone)]
pub struct RustNested {
    pub counts: HashMap<PathBuf, u64>,
}

impl From<Prost> for Rust {
    fn from(value: Prost) -> Self {
        let uuid_array: [u8; 16] = value.uuid.try_into().expect("benchmark works");
        Self {
            uuid: uuid::Uuid::from_bytes(uuid_array),
            ip: value.ip.into_iter().map(Into::into).collect(),
            text: value.text,
            nested: value.nested.unwrap().into(),
        }
    }
}

impl From<Rust> for Prost {
    fn from(value: Rust) -> Self {
        Self {
            uuid: value.uuid.as_bytes().to_vec(),
            ip: value.ip.into_iter().map(Into::into).collect(),
            text: value.text,
            nested: Some(value.nested.into()),
        }
    }
}

impl From<ProstNested> for RustNested {
    fn from(value: ProstNested) -> Self {
        Self {
            counts: value
                .counts
                .into_iter()
                .map(|(k, v)| (k.into(), v))
                .collect(),
        }
    }
}

impl From<RustNested> for ProstNested {
    fn from(value: RustNested) -> Self {
        Self {
            counts: value
                .counts
                .into_iter()
                .map(|(k, v)| (k.display().to_string(), v))
                .collect(),
        }
    }
}

criterion_group!(benches, de, ser);

fn ser(c: &mut Criterion) {
    let mut counts = HashMap::new();
    counts.insert("a".into(), 1);
    counts.insert("b".into(), 2);
    counts.insert("c".into(), 3);
    counts.insert("d".into(), 4);
    counts.insert("e".into(), 5);

    let data = black_box(Rust {
        uuid: uuid::Uuid::new_v4(),
        ip: vec![
            std::net::Ipv4Addr::LOCALHOST,
            std::net::Ipv4Addr::BROADCAST,
            std::net::Ipv4Addr::LOCALHOST,
            std::net::Ipv4Addr::BROADCAST,
            std::net::Ipv4Addr::LOCALHOST,
            std::net::Ipv4Addr::BROADCAST,
            std::net::Ipv4Addr::LOCALHOST,
            std::net::Ipv4Addr::BROADCAST,
            std::net::Ipv4Addr::LOCALHOST,
            std::net::Ipv4Addr::BROADCAST,
        ],
        text: "Benchmarking some longer text for fucks sake".into(),
        nested: RustNested { counts },
    });

    let prost_data: Prost = data.clone().into();

    let size = prost_data.encoded_len();
    let mut buffer = black_box(Vec::with_capacity(size));
    let buffer_ref = black_box(&mut buffer);

    c.bench_function("prost_ser", |b| {
        let data = data.clone();
        b.iter(|| {
            let prost_data: Prost = data.clone().into();
            prost_data.encode(buffer_ref)
        })
    });
}

fn de(c: &mut Criterion) {
    let mut counts = HashMap::new();
    counts.insert("a".into(), 1);
    counts.insert("b".into(), 2);
    counts.insert("c".into(), 3);
    counts.insert("d".into(), 4);
    counts.insert("e".into(), 5);

    let data = black_box(Rust {
        uuid: uuid::Uuid::new_v4(),
        ip: vec![
            std::net::Ipv4Addr::LOCALHOST,
            std::net::Ipv4Addr::BROADCAST,
            std::net::Ipv4Addr::LOCALHOST,
            std::net::Ipv4Addr::BROADCAST,
            std::net::Ipv4Addr::LOCALHOST,
            std::net::Ipv4Addr::BROADCAST,
            std::net::Ipv4Addr::LOCALHOST,
            std::net::Ipv4Addr::BROADCAST,
            std::net::Ipv4Addr::LOCALHOST,
            std::net::Ipv4Addr::BROADCAST,
        ],
        text: "Benchmarking some longer text for fucks sake".into(),
        nested: RustNested { counts },
    });

    let prost_data: Prost = data.clone().into();

    let size = prost_data.encoded_len();

    let mut buffer = black_box(bytes::BytesMut::with_capacity(size));
    prost_data.encode(&mut buffer).expect("benchmark works");
    let buffer = black_box(buffer.freeze());

    c.bench_function("prost_de", |b| {
        b.iter(|| {
            let prost_data = Prost::decode(&*buffer).expect("benchmark works");
            let _data: Rust = prost_data.into();
        });
    });
}
