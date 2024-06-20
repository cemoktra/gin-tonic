use gin_tonic::CompileConfig;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    CompileConfig::new()
        .with_well_known_types()
        .add_proto_file("proto/example.proto")
        .compile_into("./src")?;

    Ok(())
}
