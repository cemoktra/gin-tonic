#[cfg(not(feature = "uuid_bytes"))]
fn main() {}

#[cfg(feature = "uuid_bytes")]
use criterion::criterion_main;

#[cfg(feature = "uuid_bytes")]
criterion_main!(crate::gin_bench::benches);

#[cfg(feature = "uuid_bytes")]
pub(crate) mod gin_bench {
    use std::collections::HashMap;

    use bytes::Buf;
    use criterion::{black_box, criterion_group, Criterion};
    use gin_tonic::Message;

    criterion_group!(benches, de, ser);

    /// this would normally be generated
    #[derive(Clone, Debug, Message)]
    pub struct GinTonic {
        #[gin(tag = 1u32)]
        pub uuid: uuid::Uuid,
        #[gin(tag = 2u32, cardinality = "repeated")]
        pub ip: Vec<std::net::Ipv4Addr>,
        #[gin(tag = 3u32, proto = "string")]
        pub text: String,
        #[gin(tag = 4u32, kind = "message")]
        pub nested: GinTonicNested,
    }

    #[derive(Clone, Debug, Message)]
    pub struct GinTonicNested {
        #[gin(tag = 1u32, kind = "map", proto_key = "string", proto_value = "uint64")]
        pub counts: HashMap<String, u64>,
    }

    fn ser(c: &mut Criterion) {
        use gin_tonic::gin_tonic_core::types::PbType;

        let mut counts = HashMap::new();
        counts.insert("a".into(), 1);
        counts.insert("b".into(), 2);
        counts.insert("c".into(), 3);
        counts.insert("d".into(), 4);
        counts.insert("e".into(), 5);

        let data = black_box(GinTonic {
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
            nested: GinTonicNested { counts },
        });

        let size = data.size_hint();
        let mut buffer = black_box(bytes::BytesMut::with_capacity(size));

        c.bench_function("gin_ser", |b| {
            let data = data.clone();
            buffer.clear();
            b.iter(|| data.clone().encode(&mut buffer))
        });
    }

    fn de(c: &mut Criterion) {
        use gin_tonic::gin_tonic_core::types::PbType;

        let mut counts = HashMap::new();
        counts.insert("a".into(), 1);
        counts.insert("b".into(), 2);
        counts.insert("c".into(), 3);
        counts.insert("d".into(), 4);
        counts.insert("e".into(), 5);

        let data = black_box(GinTonic {
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
            nested: GinTonicNested { counts },
        });

        let size = data.size_hint();
        let mut buffer = black_box(bytes::BytesMut::with_capacity(size));
        data.encode(&mut buffer);

        c.bench_function("gin_de", |b| {
            b.iter(|| {
                let mut buf = buffer.chunk();
                GinTonic::decode(&mut buf).expect("benchmark works")
            });
        });
    }
}
