pub(crate) mod case;
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

use proc_macro2::TokenStream;
use std::path::PathBuf;

use crate::codegen::case::{convert, Case};
use crate::codegen::module::Module;
use external_type::ExternalType;
use protox::file::{ChainFileResolver, GoogleFileResolver, IncludeFileResolver};
use protox::prost_reflect::DescriptorPool;

/// [CompileConfig] transforming `*.proto` files into Rust code
pub struct CompileConfig {
    type_filter: Box<dyn for<'a> Fn(&'a str) -> bool>,
    type_attributes: Vec<(String, String)>,

    includes: Vec<PathBuf>,

    external_types: Vec<ExternalType>,
    proto_files: Vec<PathBuf>,
    well_known_types: bool,

    #[cfg(feature = "tonic")]
    generate_services: bool,
}

impl CompileConfig {
    pub fn new() -> Self {
        Self::with_filter(|_| true)
    }

    pub fn with_filter<F>(filter: F) -> Self
    where
        F: for<'a> Fn(&'a str) -> bool + 'static,
    {
        tracing::info!("adding filter function");
        Self {
            type_filter: Box::new(filter),
            type_attributes: vec![],
            includes: vec![],
            external_types: vec![],
            proto_files: vec![],
            well_known_types: true,

            #[cfg(feature = "tonic")]
            generate_services: true,
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
        let pattern = pattern.into();
        if !pattern.is_empty() {
            let attribute = attribute.into();
            tracing::info!("adding attribute '{attribute}' with pattern '{pattern}'",);
            self.type_attributes.push((pattern, attribute));
        }
        self
    }

    /// Batch-adds an attribute to all patterns
    pub fn add_attributes(
        mut self,
        attribute: impl Into<String> + Clone,
        patterns: &[impl std::fmt::Display],
    ) -> Self {
        for pattern in patterns {
            self = self.add_attribute(attribute.clone(), format!("{}", pattern));
        }
        self
    }

    /// import an external type
    pub fn import<I: IntoIterator<Item = ExternalType>>(mut self, paths: I) -> Self {
        for path in paths.into_iter() {
            tracing::info!(
                "importing external type: {} => {}",
                path.proto_path,
                path.rust_path
            );
            self.external_types.push(path);
        }

        self
    }

    pub fn include(mut self, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        tracing::info!("adding include '{}'", path.display());
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
        tracing::info!("adding proto file '{}'", path.display());
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

    /// add external types for well known types
    pub fn with_well_known_types(mut self) -> Self {
        tracing::info!("with well known types");
        self.well_known_types = true;
        self
    }

    /// do not add external types for well known types
    pub fn without_well_known_types(mut self) -> Self {
        tracing::info!("without well known types");
        self.well_known_types = false;
        self
    }

    /// Skip tonic service generation.
    #[cfg(feature = "tonic")]
    pub fn skip_services(mut self) -> Self {
        self.generate_services = false;
        self
    }

    pub fn compile(self) -> Result<(), CompilerError> {
        let out_dir = std::env::var("OUT_DIR").inspect_err(|err| {
            tracing::error!("failed to read OUT_DIR environment variable: {err}")
        })?;
        let out_dir = PathBuf::from(out_dir);
        self.compile_into(out_dir)
    }

    /// Generates all the files into the target directory.
    pub fn compile_into(mut self, target: impl Into<PathBuf>) -> Result<(), CompilerError> {
        let target = target.into();
        tracing::info!("compiling to target '{}'", target.display());

        let proto_files = std::mem::take(&mut self.proto_files);
        let mut include_dirs = std::mem::take(&mut self.includes);

        for proto_file in &proto_files {
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

        std::fs::create_dir_all(&target)
            .inspect_err(|err| tracing::error!("failed to create target directory: {err}"))?;

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
        let compiler = compiler
            .include_source_info(true)
            .include_imports(true)
            .open_files(proto_files)
            .inspect_err(|err| tracing::error!("compiler failed open files: {err}"))?;

        #[cfg(feature = "tonic")]
        let generate_services = self.generate_services;

        let ctx = Context::from_config(self);

        generate(
            &compiler.descriptor_pool(),
            ctx,
            &target,
            #[cfg(feature = "tonic")]
            generate_services,
        )?;

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

pub struct Context {
    external_types: Vec<ExternalType>,
    type_filter: Box<dyn for<'a> Fn(&'a str) -> bool>,
    type_attributes: Vec<(String, String)>,
}

impl Context {
    fn from_config(config: CompileConfig) -> Self {
        let CompileConfig {
            type_filter,
            type_attributes,
            includes: _,
            external_types,
            proto_files: _,
            well_known_types,
            #[cfg(feature = "tonic")]
                generate_services: _,
        } = config;

        let mut external_types = external_types;
        if well_known_types {
            external_types.extend(external_type::well_known_types());
        }

        Self {
            external_types,
            type_filter,
            type_attributes,
        }
    }

    pub fn filter(&self, name: &str) -> bool {
        (self.type_filter)(name)
    }

    pub fn attributes(&self, name: &str) -> TokenStream {
        for (pattern, attributes) in self.type_attributes.iter() {
            if utils::match_name(pattern, name) {
                let attrs =
                    syn::parse_str::<syn::DeriveInput>(&format!("{}\nstruct fake;", attributes))
                        .expect("Hardcoded struct should always be correct.")
                        .attrs;

                return quote::quote!(
                    #(#attrs)*
                );
            }
        }

        quote::quote!()
    }

    pub fn resolve_ident(&self, identifier: &str) -> Option<String> {
        let identifier = format!(".{}", identifier);

        if let Some(path) = self
            .external_types
            .iter()
            .find(|item| item.proto_path == identifier)
        {
            return Some(path.rust_path.clone());
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

            let type_path = path.rust_path.clone();

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
    ctx: Context,
    out: &std::path::Path,
    #[cfg(feature = "tonic")] generate_services: bool,
) -> Result<(), CompilerError> {
    let mut root = Module::new("<root>");

    for ty in pool.all_enums() {
        // only top level types here
        if ty.parent_message().is_some() {
            continue;
        }

        let module_path = String::from(ty.package_name());
        enums::generate(&ctx, &mut root, &module_path, ty);
    }

    for ty in pool.all_messages() {
        // only top level types here
        if ty.parent_message().is_some() {
            continue;
        }

        let module_path = String::from(ty.package_name());
        messages::generate(&ctx, &mut root, &module_path, ty);
    }

    #[cfg(feature = "tonic")]
    if generate_services {
        for svc in pool.services() {
            let module_path = String::from(svc.package_name());
            service::generate(&mut root, &module_path, svc);
        }
    }

    for module in root.children {
        module.write(out)?;
    }

    Ok(())
}
