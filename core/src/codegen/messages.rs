use std::path::{Path, PathBuf};

use protox::prost_reflect::{DescriptorPool, EnumDescriptor, MessageDescriptor};

use crate::codegen::module::Module;

use super::{
    case::{convert, Case},
    external_type::{well_known_types, ExternalType},
    CompilerError,
};

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
        generate_enum(&ctx, &mut root, &module_path, ty);
    }

    for ty in pool.all_messages() {
        // only top level types here
        if ty.parent_message().is_some() {
            continue;
        }

        let module_path = String::from(ty.package_name());

        todo!()
    }

    todo!()
}

fn create_child<'a>(parent: &'a mut Module, module_path: &str) -> &'a mut Module {
    let module_path = module_path
        .split('.')
        .map(|segment| convert(segment, Case::Snake));
    let module = parent.create_child_from_path(module_path);
    if module.is_empty() {
        let prelude = quote::quote! {
            #[allow(unused_imports)]
            use ::domain_type::DomainType;
            #[allow(unused_imports)]
            use ::strum::{EnumString, Display};
        };
        module.extend(prelude);
    }
    module
}

fn generate_enum(ctx: &Context, parent: &mut Module, module_path: &str, ty: EnumDescriptor) {
    let qualified_name = ty.full_name();

    // external types are not generated
    if ctx.resolve_ident(qualified_name).is_some() {
        return;
    }

    let module = create_child(parent, module_path);

    let ty_name = convert(ty.name(), Case::Pascal);
    let name = quote::format_ident!("{}", ty_name);

    let mut body = quote::quote!();

    for value in ty.values() {
        let tag = value.number();
        let (_package, value) = value
            .full_name()
            .rsplit_once('.')
            .expect("Enum values should be in a package");
        if value.ends_with("UNSPECIFIED") {
            // TODO: add support for UNSPECIFIED variant
            continue;
        }
        let value = convert(value, Case::Pascal);
        let value = value
            .strip_prefix(ty_name.as_ref())
            .unwrap_or(value.as_ref());

        let value_name = quote::format_ident!("{}", value);

        body.extend(quote::quote! {
            #[gin(tag = #tag)]
            #value_name,
        });
    }

    let item: syn::ItemEnum = syn::parse_quote! {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, gin_tonic_core::Enumeration)]
        pub enum #name {
            #body
        }
    };

    module.extend(quote::quote! {
        #item
    });
}

fn generate_message(ctx: &Context, parent: &mut Module, module_path: &str, ty: MessageDescriptor) {
    if ty.is_map_entry() {
        return;
    }

    let qualified_name = ty.full_name();

    // external types are not generated
    if ctx.resolve_ident(qualified_name).is_some() {
        return;
    }

    // TODO: detect unwrappable oneofs

    let module = create_child(parent, module_path);

    let ty_name = convert(ty.name(), Case::Pascal);
    let name = quote::format_ident!("{}", ty_name);

    for field in ty.fields() {}

    todo!()
}

pub struct Context {
    pub external_types: Vec<ExternalType>,
    pub path: PathBuf,
}

impl Context {
    pub fn new(path: &Path, external_types: &[ExternalType], well_known_types: bool) -> Self {
        let mut external_types = external_types.to_vec();
        if well_known_types {
            external_types.extend(crate::codegen::external_type::well_known_types());
        }

        Self {
            external_types,
            path: path.to_owned(),
        }
    }

    pub fn resolve_ident(&self, identifier: &str) -> Option<String> {
        if let Some(path) = self
            .external_types
            .iter()
            .find(|item| &item.proto_path[1..] == identifier)
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
