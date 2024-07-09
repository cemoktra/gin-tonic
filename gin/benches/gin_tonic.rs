fn main() {
    divan::main();
}

#[cfg(feature = "uuid_bytes")]
mod gin_bench {

    use gin_tonic::Message;

    /// this would normally be generated
    #[derive(Clone, Debug, Message)]
    pub struct GinTonic {
        #[gin(tag = 1u32)]
        pub uuid: uuid::Uuid,
        #[gin(tag = 2u32)]
        pub ip: std::net::Ipv4Addr,
    }

    #[divan::bench]
    fn ser(bencher: divan::Bencher) {
        use gin_tonic::gin_tonic_core::Message;

        let data = GinTonic {
            uuid: uuid::Uuid::new_v4(),
            ip: std::net::Ipv4Addr::LOCALHOST,
        };

        let size = data.size_hint();
        let mut buffer = bytes::BytesMut::with_capacity(size);
        let buffer_ref = &mut buffer;

        bencher.bench_local(move || data.clone().serialize(buffer_ref));
    }

    #[divan::bench]
    fn de(bencher: divan::Bencher) {
        use gin_tonic::gin_tonic_core::Message;

        let data = GinTonic {
            uuid: uuid::Uuid::new_v4(),
            ip: std::net::Ipv4Addr::LOCALHOST,
        };

        let size = data.size_hint();
        let mut buffer = bytes::BytesMut::with_capacity(size);
        let buffer_ref = &mut buffer;
        data.clone().serialize(buffer_ref);

        bencher.bench_local(move || {
            GinTonic::deserialize(&buffer).expect("benchmark works");
        });
    }
}
