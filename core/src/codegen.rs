pub(crate) mod case;
pub mod external_type;
pub(crate) mod messages;
pub(crate) mod module;

use std::path::PathBuf;

use external_type::ExternalType;
use messages::generate;
use protox::file::{ChainFileResolver, GoogleFileResolver, IncludeFileResolver};

/// [Compiler] transforming `*.proto` files into Rust code
#[derive(Default)]
pub struct Compiler {
    external_types: Vec<ExternalType>,
    proto_files: Vec<PathBuf>,
    well_known_types: bool,
    // TODO:
    // type_attributes: Vec<(String, String)>,
}

/// Compiler errors
#[derive(thiserror::Error, Debug)]
pub enum CompilerError {
    #[error(transparent)]
    Env(#[from] std::env::VarError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Protox(#[from] protox::Error),
    #[error(transparent)]
    Syn(#[from] syn::Error),
}

impl Compiler {
    /// create a new compiler
    pub fn new() -> Self {
        Self::default()
    }

    /// import an external type
    pub fn import<I: IntoIterator<Item = ExternalType>>(mut self, paths: I) -> Self {
        self.external_types.extend(paths);
        self
    }

    /// add `*.proto` files
    pub fn add_proto_files(mut self, paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        let paths = paths.into_iter().map(Into::into);
        self.proto_files.extend(paths);
        self
    }

    pub fn with_well_known_types(mut self) -> Self {
        self.well_known_types = true;
        self
    }

    /// start compilation
    pub fn compile(self, target: Option<impl Into<PathBuf>>) -> Result<(), CompilerError> {
        let env = CompilerEnvironment::new()?;
        let mut include_dirs = env.includes();

        for proto_file in &self.proto_files {
            println!("cargo:rerun-if-changed={}", proto_file.display());
            include_dirs.push(
                proto_file
                    .parent()
                    .expect(
                        "Unable to locate parent. [The proto file should reside in a directory]",
                    )
                    .to_owned(),
            );
        }

        let out = target
            .map(Into::into)
            .unwrap_or_else(|| PathBuf::from(env.out_dir.clone()));
        std::fs::create_dir_all(&out)?;

        let resolver = Resolver::new(include_dirs);
        let mut compiler = protox::Compiler::with_file_resolver(resolver);
        let compiler = compiler
            .include_source_info(true)
            .include_imports(true)
            .open_files(self.proto_files)?;

        generate(
            &compiler.descriptor_pool(),
            &self.external_types,
            self.well_known_types,
            &out,
        )?;

        todo!()
    }
}

/// environment variables used by the [Compiler]
struct CompilerEnvironment {
    out_dir: PathBuf,
    manifest_dir: PathBuf,
    workspace_dir: PathBuf,
}

impl CompilerEnvironment {
    /// init environment variables
    fn new() -> Result<Self, CompilerError> {
        Ok(Self {
            out_dir: std::env::var("OUT_DIR")?.into(),
            manifest_dir: std::env::var("CARGO_MANIFEST_DIR")?.into(),
            workspace_dir: std::env::var("CARGO_WORKSPACE_DIR")?.into(),
        })
    }

    /// variables to include directories
    fn includes(&self) -> Vec<PathBuf> {
        let mut include_dirs = Vec::new();
        include_dirs.push(self.manifest_dir.clone());
        include_dirs.push(self.workspace_dir.clone());
        include_dirs
    }
}

/// protox [protox::file::Resolver]
struct Resolver(ChainFileResolver);

impl Resolver {
    pub fn new(include_directories: impl IntoIterator<Item = PathBuf>) -> Self {
        let mut resolver = ChainFileResolver::new();
        for include in include_directories.into_iter() {
            resolver.add(IncludeFileResolver::new(include));
        }
        resolver.add(GoogleFileResolver::new());
        Self(resolver)
    }
}

impl protox::file::FileResolver for Resolver {
    fn open_file(&self, name: &str) -> Result<protox::file::File, protox::Error> {
        let file = self.0.open_file(name)?;
        if let Some(path) = file.path() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
        Ok(file)
    }

    fn resolve_path(&self, path: &std::path::Path) -> Option<String> {
        self.0.resolve_path(path)
    }
}
