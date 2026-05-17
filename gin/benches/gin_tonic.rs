#[cfg(not(feature = "uuid"))]
fn main() {}

#[cfg(feature = "uuid")]
use criterion::criterion_main;

#[cfg(feature = "uuid")]
criterion_main!(crate::gin_bench::benches);

#[cfg(feature = "uuid")]
pub(crate) mod gin_bench {
    use criterion::{Criterion, black_box, criterion_group};
    use gin_tonic::{Message, Scalar, decoder::Decoder, encoder::Encoder, fxhash::FxHashMap};

    criterion_group!(benches, encode, decode);

    /// this would normally be generated
    #[derive(Clone, Debug, Message)]
    pub struct GinTonic {
        #[gin(id = 1u32, scalar = "bytes")]
        pub uuid: uuid::Uuid,
        #[gin(id = 2u32, scalar = "uint32")]
        pub ip: Vec<std::net::Ipv4Addr>,
        #[gin(id = 3u32, scalar = "string")]
        pub text: String,
        #[gin(id = 4u32)]
        pub nested: GinTonicNested,
    }

    #[derive(Clone, Debug, Message)]
    pub struct GinTonicNested {
        #[gin(id = 1u32, value_scalar = "uint64")]
        pub counts: FxHashMap<String, u64>,
    }

    fn encode(c: &mut Criterion) {
        use gin_tonic::gin_tonic_core::Message;

        let mut counts = FxHashMap::with_capacity_and_hasher(8, Default::default());
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
        let mut buffer = black_box(vec![0u8; size]);

        c.bench_function("gin_encode", |b| {
            b.iter(|| {
                data.encode_message(&mut Encoder::new(&mut buffer));
            })
        });
    }

    fn decode(c: &mut Criterion) {
        let mut counts = FxHashMap::with_capacity_and_hasher(8, Default::default());
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

        let size = data.message_size_hint();
        let mut buffer = black_box(vec![0u8; size]);
        data.encode_message(&mut Encoder::new(&mut buffer));

        c.bench_function("gin_decode", |b| {
            b.iter(|| {
                GinTonic::decode_message(&mut Decoder::new(&buffer)).expect("benchmark works")
            });
        });
    }
}
