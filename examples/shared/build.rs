use gin_tonic_core::codegen::{BuildEnvironment, Compiler};
use std::path::PathBuf;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Compiler::new(BuildEnvironment::new()?)
        .include([PathBuf::from("../..")])
        .with_well_known_types()
        .add_proto_files(["proto/example.proto"])
        .compile(Some("./src"))?;

    Ok(())
}
