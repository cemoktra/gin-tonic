use gin_tonic::Compiler;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    Compiler::new()
        .with_well_known_types()
        .add_proto_file("proto/example.proto")
        .compile_into("./src")?;

    Ok(())
}
