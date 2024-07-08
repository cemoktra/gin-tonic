#[cfg(any(feature = "uuid_bytes", feature = "uuid_string"))]
use gin_tonic::Message;

/// this would normally be generated
#[cfg(any(feature = "uuid_bytes", feature = "uuid_string"))]
#[derive(Clone, Debug, Message)]
pub struct GinTonic {
    #[gin(tag = 1u32)]
    pub uuid: uuid::Uuid,
    #[gin(tag = 2u32)]
    pub ip: std::net::Ipv4Addr,
}

fn main() {
    divan::main();
}

#[divan::bench]
#[cfg(any(feature = "uuid_bytes", feature = "uuid_string"))]
fn ser(bencher: divan::Bencher) {
    use gin_tonic::gin_tonic_core::Message;

    let data = GinTonic {
        uuid: uuid::Uuid::new_v4(),
        ip: std::net::Ipv4Addr::LOCALHOST,
    };

    let size = data.size_hint();
    let mut buffer = Vec::with_capacity(size);
    let buffer_ref = &mut buffer;

    bencher.bench_local(move || data.clone().serialize(buffer_ref));
}

#[divan::bench]
#[cfg(any(feature = "uuid_bytes", feature = "uuid_string"))]
fn de(bencher: divan::Bencher) {
    use gin_tonic::gin_tonic_core::Message;

    let data = GinTonic {
        uuid: uuid::Uuid::new_v4(),
        ip: std::net::Ipv4Addr::LOCALHOST,
    };

    let size = data.size_hint();
    let mut buffer = Vec::with_capacity(size);
    let buffer_ref = &mut buffer;
    data.clone().serialize(buffer_ref).expect("benchmark works");

    bencher.bench_local(move || {
        GinTonic::deserialize(&buffer).expect("benchmark works");
    });
}
