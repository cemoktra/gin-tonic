fn main() {
    divan::main();
}

#[cfg(feature = "uuid_bytes")]
mod gin_bench {

    use std::{collections::HashMap, path::PathBuf};

    use gin_tonic::Message;

    /// this would normally be generated
    #[derive(Clone, Debug, Message)]
    pub struct GinTonic {
        #[gin(tag = 1u32)]
        pub uuid: uuid::Uuid,
        #[gin(tag = 2u32, cardinality = "repeated")]
        pub ip: Vec<std::net::Ipv4Addr>,
        #[gin(tag = 3u32)]
        pub text: String,
        #[gin(tag = 4u32, kind = "message")]
        pub nested: GinTonicNested,
    }

    #[derive(Clone, Debug, Message)]
    pub struct GinTonicNested {
        #[gin(tag = 1u32, kind = "map")]
        pub counts: HashMap<PathBuf, u64>,
    }

    #[divan::bench(min_time = std::time::Duration::from_secs(1))]
    fn ser(bencher: divan::Bencher) {
        use gin_tonic::gin_tonic_core::Message;

        let mut counts = HashMap::new();
        counts.insert("a".into(), 1);
        counts.insert("b".into(), 2);
        counts.insert("c".into(), 3);
        counts.insert("d".into(), 4);
        counts.insert("e".into(), 5);

        let data = divan::black_box(GinTonic {
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
        let mut buffer = divan::black_box(bytes::BytesMut::with_capacity(size));
        let buffer_ref = divan::black_box(&mut buffer);

        bencher.bench_local(move || data.clone().serialize(buffer_ref));
    }

    #[divan::bench(min_time = std::time::Duration::from_secs(1))]
    fn de(bencher: divan::Bencher) {
        use gin_tonic::gin_tonic_core::Message;

        let mut counts = HashMap::new();
        counts.insert("a".into(), 1);
        counts.insert("b".into(), 2);
        counts.insert("c".into(), 3);
        counts.insert("d".into(), 4);
        counts.insert("e".into(), 5);

        let data = divan::black_box(GinTonic {
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
        let mut buffer = divan::black_box(bytes::BytesMut::with_capacity(size));
        let buffer_ref = divan::black_box(&mut buffer);
        data.clone().serialize(buffer_ref);

        bencher.bench_local(move || {
            GinTonic::deserialize(&buffer).expect("benchmark works");
        });
    }
}
