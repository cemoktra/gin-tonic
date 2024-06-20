#[test]
fn test_compiler() {
    crate::codegen::CompileConfig::new()
        .add_proto_file("test.proto")
        .with_well_known_types()
        // .include(&[".."])
        .compile_into("./test_codegen")
        .unwrap();
}
