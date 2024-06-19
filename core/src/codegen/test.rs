use crate::codegen::{Compiler, CompilerEnvironment};
use std::path::{Path, PathBuf};

struct TestEnvironment(PathBuf);

impl CompilerEnvironment for TestEnvironment {
    fn out_dir(&self) -> &Path {
        &self.0
    }

    fn includes(&self) -> Vec<PathBuf> {
        vec![self.0.to_owned()]
    }
}

#[test]
fn test_compiler() {
    Compiler::new(TestEnvironment("..".into()))
        .add_proto_files(&["test.proto"])
        .with_well_known_types()
        .compile(Some("./test_codegen"))
        .unwrap();
}
