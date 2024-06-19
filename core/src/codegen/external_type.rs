#[derive(Clone)]
pub struct ExternalType {
    pub proto_path: String,
    pub rust_path: String,
    pub rust_type: Option<String>,
}

impl ExternalType {
    fn raw(proto_path: impl Into<String>, rust_path: impl Into<String>) -> Self {
        Self {
            proto_path: proto_path.into(),
            rust_path: rust_path.into(),
            rust_type: None,
        }
    }

    pub fn new(proto_rust_package_name: &str, proto_version: &str, proto_type: &str) -> Self {
        let proto = format!(".{proto_rust_package_name}.{proto_version}.{proto_type}");
        let rust = format!("{proto_rust_package_name}::proto::{proto_rust_package_name}_{proto_version}::{proto_type}");
        Self::raw(proto, rust)
    }

    pub fn rust_type(self, rust_type: impl Into<String>) -> Self {
        Self {
            rust_type: Some(rust_type.into()),
            ..self
        }
    }
}

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
