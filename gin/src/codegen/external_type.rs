/// types that are not generated
#[derive(Clone, Debug)]
pub struct ExternalType {
    pub proto_path: String,
    pub rust_path: String,
}

impl ExternalType {
    fn raw(proto_path: impl Into<String>, rust_path: impl Into<String>) -> Self {
        Self {
            proto_path: proto_path.into(),
            rust_path: rust_path.into(),
        }
    }
}

// some of the well known google protobuf types
pub fn well_known_types() -> Vec<ExternalType> {
    vec![
        ExternalType::raw(".google.protobuf", "::prost_types"),
        ExternalType::raw(".google.protobuf.BoolValue", "bool"),
        ExternalType::raw(
            ".google.protobuf.BytesValue",
            "::prost::alloc::vec::Vec<u8>",
        ),
        ExternalType::raw(".google.protobuf.DoubleValue", "f64"),
        ExternalType::raw(".google.protobuf.Empty", "()"),
        ExternalType::raw(".google.protobuf.FloatValue", "f32"),
        ExternalType::raw(".google.protobuf.Int32Value", "i32"),
        ExternalType::raw(".google.protobuf.Int64Value", "i64"),
        ExternalType::raw(
            ".google.protobuf.StringValue",
            "::prost::alloc::string::String",
        ),
        ExternalType::raw(".google.protobuf.UInt32Value", "u32"),
        ExternalType::raw(".google.protobuf.UInt64Value", "u64"),
    ]
}

pub fn gin_types() -> Vec<ExternalType> {
    vec![
        ExternalType::raw(".gin_tonic.v1.IpV4", "std::net::Ipv4Addr"),
        #[cfg(any(feature = "uuid_bytes", feature = "uuid_string"))]
        ExternalType::raw(".gin_tonic.v1.Uuid", "uuid::Uuid"),
    ]
}
