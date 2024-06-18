pub(crate) mod case;
pub(crate) mod enumeration;
pub mod external_type;
pub(crate) mod messages;
pub(crate) mod module;
pub(crate) mod one_of;
pub(crate) mod service;
#[cfg(test)]
mod test;
pub(crate) mod utils;

use std::path::{Path, PathBuf};

use crate::codegen::case::{convert, Case};
use crate::codegen::module::Module;
use external_type::ExternalType;
use protox::file::{ChainFileResolver, GoogleFileResolver, IncludeFileResolver};
use protox::prost_reflect::DescriptorPool;

/// [Compiler] transforming `*.proto` files into Rust code
#[derive(Default)]
pub struct Compiler<E>
where
    E: CompilerEnvironment,
{
    environment: E,
    external_types: Vec<ExternalType>,
    proto_files: Vec<PathBuf>,
    extra_includes: Vec<PathBuf>,
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

impl<E> Compiler<E>
where
    E: CompilerEnvironment,
{
    /// create a new compiler
    pub fn new(environment: E) -> Self {
        Self {
            environment,
            external_types: vec![],
            proto_files: vec![],
            well_known_types: false,
            extra_includes: vec![],
        }
    }

    /// import an external type
    pub fn include<I: IntoIterator<Item = PathBuf>>(mut self, includes: I) -> Self {
        self.extra_includes.extend(includes);
        self
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
        let mut include_dirs = self.environment.includes();
        include_dirs.extend(self.extra_includes);

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
            .unwrap_or_else(|| PathBuf::from(self.environment.out_dir().to_owned()));
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

        Ok(())
    }
}

pub trait CompilerEnvironment {
    fn out_dir(&self) -> &Path;
    /// variables to include directories
    fn includes(&self) -> Vec<PathBuf>;
}

/// environment variables used by the [Compiler]
pub struct BuildEnvironment {
    out_dir: PathBuf,
    manifest_dir: PathBuf,
    workspace_dir: Option<PathBuf>,
}

impl BuildEnvironment {
    /// init environment variables
    pub fn new() -> Result<Self, CompilerError> {
        Ok(Self {
            out_dir: std::env::var("OUT_DIR")?.into(),
            manifest_dir: std::env::var("CARGO_MANIFEST_DIR")?.into(),
            workspace_dir: match std::env::var("CARGO_WORKSPACE_DIR") {
                Ok(workspace_dir) => Some(workspace_dir.into()),
                Err(_) => None,
            },
        })
    }
}

impl CompilerEnvironment for BuildEnvironment {
    fn out_dir(&self) -> &Path {
        self.out_dir.as_path()
    }

    fn includes(&self) -> Vec<PathBuf> {
        let mut include_dirs = Vec::new();
        include_dirs.push(self.manifest_dir.clone());
        if let Some(workspace_dir) = self.workspace_dir.as_ref() {
            include_dirs.push(workspace_dir.clone());
        }

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

pub struct Context {
    pub external_types: Vec<ExternalType>,
    pub path: PathBuf,
}

impl Context {
    pub fn new(path: &Path, external_types: &[ExternalType], well_known_types: bool) -> Self {
        let mut external_types = external_types.to_vec();
        if well_known_types {
            external_types.extend(external_type::well_known_types());
        }

        Self {
            external_types,
            path: path.to_owned(),
        }
    }

    pub fn resolve_ident(&self, identifier: &str) -> Option<String> {
        let identifier = format!(".{}", identifier);

        if let Some(path) = self
            .external_types
            .iter()
            .find(|item| item.proto_path == identifier)
        {
            let rust_type = path.rust_type.as_deref().unwrap_or(&*path.rust_path);
            return Some(rust_type.to_string());
        }

        for (index, _) in identifier.rmatch_indices('.') {
            let (package, type_name) = identifier.split_at(index);
            let type_name = &type_name[1..];

            let path = self
                .external_types
                .iter()
                .find(|item| item.proto_path == package);

            let Some(path) = path else {
                continue;
            };

            let mut segments = type_name.split('.');
            let ident_type = segments
                .next_back()
                .map(|item| convert(item, Case::Pascal).to_string());

            let type_path = path.rust_type.as_deref().unwrap_or(&*path.rust_path);

            let segments = type_path
                .split("::")
                .chain(segments)
                .enumerate()
                .map(|(idx, segment)| {
                    if idx == 0 && segment == "crate" {
                        // If the first segment of the path is 'crate', then do not escape
                        // it into a raw identifier, since it's being used as the keyword.
                        segment.to_owned()
                    } else {
                        convert(segment, Case::Snake).to_string()
                    }
                })
                .chain(ident_type.into_iter())
                .collect::<Vec<_>>();

            return Some(segments.join("::"));
        }

        None
    }
}

pub(crate) fn generate(
    pool: &DescriptorPool,
    external_types: &[ExternalType],
    well_known_types: bool,
    out_dir: &Path,
) -> Result<(), CompilerError> {
    let ctx = Context::new(out_dir, external_types, well_known_types);
    let mut root = Module::new("<root>");

    for ty in pool.all_enums() {
        // only top level types here
        if ty.parent_message().is_some() {
            continue;
        }

        let module_path = String::from(ty.package_name());
        enumeration::generate(&ctx, &mut root, &module_path, ty);
    }

    for ty in pool.all_messages() {
        // only top level types here
        if ty.parent_message().is_some() {
            continue;
        }

        let module_path = String::from(ty.package_name());
        messages::generate(&ctx, &mut root, &module_path, ty);
    }

    for svc in pool.services() {
        let module_path = String::from(svc.package_name());
        service::generate(&mut root, &module_path, svc);
    }

    let path = ctx.path;

    for module in root.children {
        module.write(&path)?;
    }

    Ok(())
}
