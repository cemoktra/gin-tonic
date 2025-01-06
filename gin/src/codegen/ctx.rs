use crate::codegen::case::{convert, Case};
use crate::codegen::external_type::ExternalType;
use crate::codegen::module::Module;
use crate::codegen::{enums, external_type, messages, service, utils};
use crate::CompilerError;
use proc_macro2::TokenStream;
use protox::prost_reflect::DescriptorPool;

pub struct Generator {
    well_known_types: bool,
    external_types: Vec<ExternalType>,
    type_filter: Box<dyn for<'a> Fn(&'a str) -> bool>,
    type_attributes: Vec<(String, String)>,

    #[cfg(feature = "tonic")]
    generate_services: bool,
}

impl Generator {
    pub fn new() -> Self {
        Self::with_filter(|_| true)
    }

    pub fn with_filter<F>(filter: F) -> Self
    where
        F: for<'a> Fn(&'a str) -> bool + 'static,
    {
        Self {
            well_known_types: true,
            external_types: vec![],
            type_filter: Box::new(filter),
            type_attributes: vec![],

            #[cfg(feature = "tonic")]
            generate_services: true,
        }
    }

    /// Skip tonic service generation.
    #[cfg(feature = "tonic")]
    pub fn skip_services(&mut self) {
        self.generate_services = false;
    }

    /// If the pattern starts with a dot, then the pattern is a prefix match
    /// pattern = "." - Matches everything
    /// pattern = ".package.v1.MyRequest" - Matches all "MyRequest" types
    ///
    /// If it doesn't, then it's treated as a suffix match.
    ///
    /// pattern = ".Error" - All "Error" messages will match.
    pub fn add_attribute(&mut self, attribute: impl Into<String>, pattern: impl Into<String>) {
        let pattern = pattern.into();
        if !pattern.is_empty() {
            let attribute = attribute.into();
            tracing::debug!("adding attribute '{attribute}' with pattern '{pattern}'",);
            self.type_attributes.push((pattern, attribute));
        }
    }

    /// Batch-adds an attribute to all patterns
    pub fn add_attributes(&mut self, attribute: impl Into<String>, patterns: &[impl ToString]) {
        let attribute = attribute.into();
        for pattern in patterns {
            self.add_attribute(attribute.clone(), pattern.to_string());
        }
    }

    /// import an external type
    pub fn import<I: IntoIterator<Item = ExternalType>>(&mut self, paths: I) {
        for path in paths.into_iter() {
            tracing::debug!(
                "importing external type: {} => {}",
                path.proto_path,
                path.rust_path
            );
            self.external_types.push(path);
        }
    }

    /// add external types for well known types
    pub fn with_well_known_types(&mut self, value: bool) {
        tracing::debug!(
            "{} well known types",
            if value { "with" } else { "without" }
        );

        self.well_known_types = value;
    }

    pub fn generate(
        mut self,
        pool: &DescriptorPool,
        out: &std::path::Path,
    ) -> Result<(), CompilerError> {
        if self.well_known_types {
            self.external_types
                .extend(external_type::well_known_types());
        }

        let mut root = Module::new("<root>");

        for ty in pool.all_enums() {
            // only top level types here
            if ty.parent_message().is_some() {
                continue;
            }

            let module_path = String::from(ty.package_name());
            enums::generate(&self, &mut root, &module_path, ty);
        }

        for ty in pool.all_messages() {
            // only top level types here
            if ty.parent_message().is_some() {
                continue;
            }

            let module_path = String::from(ty.package_name());
            messages::generate(&self, &mut root, &module_path, ty);
        }

        #[cfg(feature = "tonic")]
        if self.generate_services {
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

    pub(crate) fn filter(&self, name: &str) -> bool {
        (self.type_filter)(name)
    }

    pub(crate) fn attributes(&self, name: &str) -> TokenStream {
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

    pub(crate) fn resolve_ident(&self, identifier: &str) -> Option<String> {
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
