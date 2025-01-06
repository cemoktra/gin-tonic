pub(crate) mod case;
pub(crate) mod ctx;
pub(crate) mod enums;
pub mod external_type;
pub(crate) mod messages;
pub(crate) mod module;
pub(crate) mod one_of;
#[cfg(feature = "tonic")]
pub(crate) mod service;
#[cfg(test)]
mod test;
pub(crate) mod utils;

use std::path::PathBuf;

use external_type::ExternalType;
use protox::file::{ChainFileResolver, GoogleFileResolver, IncludeFileResolver};

pub use ctx::Generator;

/// [CompileConfig] transforming `*.proto` files into Rust code
pub struct CompileConfig {
    ctx: Generator,

    includes: Vec<PathBuf>,

    proto_files: Vec<PathBuf>,
}

impl CompileConfig {
    pub fn new() -> Self {
        Self {
            ctx: Generator::new(),

            includes: vec![],
            proto_files: vec![],
        }
    }

    pub fn with_filter<F>(filter: F) -> Self
    where
        F: for<'a> Fn(&'a str) -> bool + 'static,
    {
        Self {
            ctx: Generator::with_filter(filter),

            includes: vec![],
            proto_files: vec![],
        }
    }

    /// If the pattern starts with a dot, then the pattern is a prefix match
    /// pattern = "." - Matches everything
    /// pattern = ".package.v1.MyRequest" - Matches all "MyRequest" types
    ///
    /// If it doesn't, then it's treated as a suffix match.
    ///
    /// pattern = ".Error" - All "Error" messages will match.
    pub fn add_attribute(
        mut self,
        attribute: impl Into<String>,
        pattern: impl Into<String>,
    ) -> Self {
        self.ctx.add_attribute(attribute, pattern);
        self
    }

    /// Batch-adds an attribute to all patterns
    pub fn add_attributes(
        mut self,
        attribute: impl Into<String>,
        patterns: &[impl ToString],
    ) -> Self {
        self.ctx.add_attributes(attribute, patterns);
        self
    }

    /// import an external type
    pub fn import<I: IntoIterator<Item = ExternalType>>(&mut self, paths: I) {
        self.ctx.import(paths);
    }

    /// add external types for well known types
    pub fn with_well_known_types(mut self) -> Self {
        tracing::debug!("with well known types");
        self.ctx.with_well_known_types(true);
        self
    }

    /// do not add external types for well known types
    pub fn without_well_known_types(mut self) -> Self {
        tracing::debug!("without well known types");
        self.ctx.with_well_known_types(false);
        self
    }
    /// Skip tonic service generation.
    #[cfg(feature = "tonic")]
    pub fn skip_services(mut self) -> Self {
        self.ctx.skip_services();
        self
    }

    pub fn include(mut self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        tracing::debug!("adding include '{}'", path.display());
        self.includes.push(path);
        self
    }

    pub fn includes(mut self, paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        let paths = paths.into_iter().map(Into::into);
        self.includes.extend(paths);
        self
    }

    /// add `*.proto` file
    pub fn add_proto_file(mut self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        tracing::debug!("adding proto file '{}'", path.display());
        self.proto_files.push(path);
        self
    }

    /// add `*.proto` files
    pub fn add_proto_files(mut self, paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        for path in paths.into_iter() {
            self = self.add_proto_file(path)
        }
        self
    }

    pub fn compile(self) -> Result<(), CompilerError> {
        let out_dir = std::env::var("OUT_DIR").inspect_err(|err| {
            tracing::error!("failed to read OUT_DIR environment variable: {err}")
        })?;
        let out_dir = PathBuf::from(out_dir);
        self.compile_into(out_dir)
    }

    #[cfg(feature = "internals")]
    pub fn into_parts(self) -> Result<(Generator, protox::Compiler), CompilerError> {
        self.into_parts_impl()
    }

    fn into_parts_impl(self) -> Result<(Generator, protox::Compiler), CompilerError> {
        let Self {
            ctx,
            includes: mut include_dirs,
            proto_files,
        } = self;

        for proto_file in &proto_files {
            println!("cargo:rerun-if-changed={}", proto_file.display());

            let parent = proto_file
                .parent()
                .expect("Unable to locate parent. [The proto file should reside in a directory]")
                .to_owned();

            include_dirs.push(parent);
        }

        match std::env::current_dir() {
            Ok(directory) => {
                for manifest in utils::manifests(&directory) {
                    if let Some(parent) = manifest.parent() {
                        include_dirs.push(parent.to_owned());
                    }
                }
            }
            Err(err) => {
                tracing::warn!("Failed to retrieve current directory: {err}");
            }
        }

        let resolver = Resolver::new(include_dirs);

        let mut compiler = protox::Compiler::with_file_resolver(resolver);

        compiler
            .include_source_info(true)
            .include_imports(true)
            .open_files(proto_files)
            .inspect_err(|err| tracing::error!("compiler failed open files: {err}"))?;

        Ok((ctx, compiler))
    }

    /// Generates all the files into the target directory.
    pub fn compile_into(self, target: impl Into<PathBuf>) -> Result<(), CompilerError> {
        let target = target.into();
        tracing::debug!("compiling to target '{}'", target.display());

        std::fs::create_dir_all(&target)
            .inspect_err(|err| tracing::error!("failed to create target directory: {err}"))?;

        let (ctx, compiler) = self.into_parts_impl()?;

        ctx.generate(&compiler.descriptor_pool(), &target)?;

        Ok(())
    }
}

impl Default for CompileConfig {
    fn default() -> Self {
        CompileConfig::new()
    }
}

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
    fn resolve_path(&self, path: &std::path::Path) -> Option<String> {
        self.0.resolve_path(path)
    }

    fn open_file(&self, name: &str) -> Result<protox::file::File, protox::Error> {
        let file = self.0.open_file(name)?;
        if let Some(path) = file.path() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
        Ok(file)
    }
}
