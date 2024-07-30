use std::borrow::Cow;

use proc_macro2::{Delimiter, TokenStream};
use protox::prost_reflect::{
    Cardinality, DescriptorPool, DynamicMessage, FieldDescriptor, Kind, Value,
};
use quote::quote;

use crate::codegen::{case, Context};

const RUST_TYPE: &str = ".gin_tonic.v1.rust_type";

pub fn ext_ref<'a>(
    pool: &DescriptorPool,
    ext: &str,
    options: &'a DynamicMessage,
) -> Option<&'a Value> {
    let ext = pool.get_extension_by_name(ext);

    if let Some(Cow::Borrowed(value)) = ext.map(|ext| options.get_extension(&ext)) {
        Some(value)
    } else {
        None
    }
}

pub fn resolve_message(ctx: &Context, origin_type: &str, qualified_name: &str) -> TokenStream {
    if let Some(external) = ctx.resolve_ident(qualified_name) {
        let ty = syn::parse_str::<syn::Type>(&external).expect("Invalid path");
        quote::quote!(#ty)
    } else {
        let path = relative_path(origin_type, qualified_name);
        let path = syn::parse2::<syn::Path>(path).expect("Invalid path");
        quote::quote!(#path)
    }
}

pub fn common_prefix<'a, 'b>(
    left_raw: &'a str,
    right_raw: &'b str,
) -> Option<(&'a str, &'a str, &'b str)> {
    let mut index = 0;

    loop {
        let (Some(left), Some(right)) = (left_raw[index..].find('.'), right_raw[index..].find('.'))
        else {
            // One has more '.' than the other.
            break;
        };

        // Offset the indices, as we only inspect the slice after the previous index.
        let left = index + left;
        let right = index + right;

        // Remember where we are.
        let mark = left;

        // Slice the strings
        let left = &left_raw[..left];
        let right = &right_raw[..right];

        // Check if they're equal
        if left == right {
            index = mark + 1;
        } else {
            break;
        }
    }

    // No matches, so no common prefix.
    if index == 0 {
        return None;
    }

    let (common_left, left) = left_raw.split_at(index);
    let (common_right, right) = right_raw.split_at(index);

    assert_eq!(common_left, common_right);

    Some((common_left, left, right))
}

/// Given an enclosing type, determine the relative path to another.
/// This function creates a rust module path to another type, given the "origin".
/// Enclosing is perhaps a misnomer, as it could also be a child type.
///
/// Another way to phrase this function is given 'X', what is the path to 'Y'
///
/// Given 'a.b.B', what is the path to 'a.A':
/// relative_path("a.b.B", "a.A") => "super::A"
///
/// Given 'a.A', what is the path to 'a.b.B':
/// relative_path("a.A", "a.b.B") => "b::B"
///
/// relative_path("a.c.", "a.b.B") => "super::b::B"
///
/// Implementation details:
/// We only use the origin type to determine the package.
/// We can't use the type's package (the useful helper function), as it doesn't include nested types.
///
fn relative_path(origin_type: &str, qualified_name: &str) -> TokenStream {
    let (origin_package, _origin_ty) = origin_type.rsplit_once('.').unwrap_or(("", origin_type));
    let (package, ty) = qualified_name
        .rsplit_once('.')
        .unwrap_or(("", qualified_name));

    let ty = case::convert(ty, case::Case::Pascal);
    let ty = quote::format_ident!("{}", ty);

    // if package("a.A") == package("a.B")
    if origin_package == package {
        return quote::quote!(#ty);
    }

    let parent_handler: fn(&str) -> proc_macro2::Ident = |_segment| quote::format_ident!("super");
    let child_handler: fn(&str) -> proc_macro2::Ident = |segment| {
        let segment = case::convert(segment, case::Case::Snake);
        quote::format_ident!("{}", segment)
    };

    let (diff, handler) = if let Some(diff) = origin_package.strip_prefix(package) {
        // Accessing a type from a parent scope.
        (diff, parent_handler)
    } else if let Some(diff) = package.strip_prefix(origin_package) {
        // Accessing a child message.
        (diff, child_handler)
    } else {
        let Some(prefix) = common_prefix(origin_package, package) else {
            // We already checked for an exact package match, so there's no other possibility.
            // The "reference" implementation (the prost one) generates all non-external types it comes across.
            // In our case, we don't want to generate those, so we panic.
            panic!("Unknown external type: \"{}\" [Did you forget to add the external type to the imports?]", qualified_name);
        };
        let (common_prefix, origin_diff, target_diff) = prefix;

        // We expect the package + version, so there should always be two '.'
        let same_package = common_prefix.bytes().filter(|c| *c == b'.').count() >= 2;
        if !same_package {
            panic!(
                "Two types come from different packages: {} and {}",
                origin_type, qualified_name
            );
        }

        let segments = origin_diff
            .split('.')
            .map(parent_handler)
            .chain(target_diff.split('.').map(child_handler))
            .collect::<Vec<_>>();

        return quote::quote! {
            #(#segments::)*#ty
        };
    };

    let segments = diff.split('.').skip(1).map(handler).collect::<Vec<_>>();

    quote::quote! {
        #(#segments::)*#ty
    }
}

pub fn proto_attribute(field: &FieldDescriptor) -> TokenStream {
    fn resolve(field: &FieldDescriptor) -> Option<TokenStream> {
        match field.kind() {
            Kind::Double => Some(quote! { "double" }),
            Kind::Float => Some(quote! { "float" }),
            Kind::Int32 => Some(quote! { "int32" }),
            Kind::Int64 => Some(quote! { "int64" }),
            Kind::Uint32 => Some(quote! { "uint32" }),
            Kind::Uint64 => Some(quote! { "uint64" }),
            Kind::Sint32 => Some(quote! { "sint32" }),
            Kind::Sint64 => Some(quote! { "sint64" }),
            Kind::Fixed32 => Some(quote! { "fixed32" }),
            Kind::Fixed64 => Some(quote! { "fixed64" }),
            Kind::Sfixed32 => Some(quote! { "sfixed32" }),
            Kind::Sfixed64 => Some(quote! { "sfixed64" }),
            Kind::Bool => Some(quote! { "bool" }),
            Kind::String => Some(quote! { "string" }),
            Kind::Bytes => None,
            Kind::Message(_) => None,
            Kind::Enum(_) => None,
        }
    }

    let options = field.options();
    if let Some(Value::String(_)) = ext_ref(field.parent_pool(), RUST_TYPE, &options) {
        return quote::quote!();
    }

    if let Kind::Message(ty) = field.kind() {
        let cardinality = field.cardinality();
        if cardinality == Cardinality::Repeated && ty.is_map_entry() {
            let key_resolved = resolve(&ty.map_entry_key_field());
            let value_resolved = resolve(&ty.map_entry_value_field());

            match (key_resolved, value_resolved) {
                (Some(key), Some(value)) => quote! {
                    , proto_key = #key, proto_value = #value
                },
                (Some(key), None) => quote! {
                    , proto_key = #key
                },
                (None, Some(value)) => quote! {
                    , proto_value = #value
                },
                (None, None) => quote! {},
            }
        } else {
            quote! {}
        }
    } else {
        let resolved = resolve(field);

        if let Some(resolved) = resolved {
            quote! { ,proto = #resolved }
        } else {
            quote! {}
        }
    }
}

pub fn field_type(ctx: &Context, enclosed_type: &str, field: &FieldDescriptor) -> TokenStream {
    let options = field.options();

    if let Some(Value::String(rust_type)) = ext_ref(field.parent_pool(), RUST_TYPE, &options) {
        // TODO Better error message.
        let path = syn::parse_str::<syn::Type>(rust_type).expect("Invalid path");
        return quote::quote!(#path);
    }

    let cardinality = field.cardinality();

    let field_type = match field.kind() {
        Kind::Double => quote::quote!(f64),
        Kind::Float => quote::quote!(f32),
        Kind::Int32 | Kind::Sint32 | Kind::Sfixed32 => quote::quote!(i32),
        Kind::Int64 | Kind::Sint64 | Kind::Sfixed64 => quote::quote!(i64),
        Kind::Uint32 | Kind::Fixed32 => quote::quote!(u32),
        Kind::Uint64 | Kind::Fixed64 => quote::quote!(u64),
        Kind::Bool => quote::quote!(bool),
        Kind::String => quote::quote!(String),
        Kind::Bytes => quote::quote!(Vec<u8>),
        Kind::Message(ty) => {
            if cardinality == Cardinality::Repeated && ty.is_map_entry() {
                let key_ty = field_type(ctx, enclosed_type, &ty.map_entry_key_field());
                let value_ty = field_type(ctx, enclosed_type, &ty.map_entry_value_field());
                return quote::quote! {
                    std::collections::HashMap<#key_ty, #value_ty>
                };
            } else {
                resolve_message(ctx, enclosed_type, ty.full_name())
            }
        }
        Kind::Enum(ty) => resolve_message(ctx, enclosed_type, ty.full_name()),
    };

    let optional = field.field_descriptor_proto().proto3_optional();
    if optional {
        quote::quote! {
            Option<#field_type>
        }
    } else if field.is_list() {
        quote::quote! {
            Vec<#field_type>
        }
    } else {
        field_type
    }
}

// Checks whether a path pattern matches a given path.
pub(crate) fn match_name(pattern: &str, path: &str) -> bool {
    // @HACK jeremy.barrow - 19 Jan 2024: Just a stupid hack for now.
    let path = format!(".{}", path);
    if pattern == "." || pattern == path {
        true
    } else {
        let pattern_segments = pattern.split('.').collect::<Vec<_>>();
        let path_segments = path.split('.').collect::<Vec<_>>();

        if pattern_segments.len() > path_segments.len() {
            false
        } else if &pattern[..1] == "." {
            // prefix match
            pattern_segments[..] == path_segments[..pattern_segments.len()]
        } else {
            // suffix match
            pattern_segments[..] == path_segments[path_segments.len() - pattern_segments.len()..]
        }
    }
}

/// Checks `ts == ()`
pub(crate) fn is_unit_type(ts: &TokenStream) -> bool {
    let mut iter = ts.clone().into_iter();
    let Some(proc_macro2::TokenTree::Group(grp)) = iter.next() else {
        return false;
    };
    grp.delimiter() == Delimiter::Parenthesis && grp.stream().is_empty() && iter.next().is_none()
}

const CARGO_TOML: &str = "Cargo.toml";

pub fn manifests(path: &std::path::Path) -> impl Iterator<Item = std::path::PathBuf> + '_ {
    path.ancestors().filter_map(|path| {
        let manifest = path.join(CARGO_TOML);
        manifest.exists().then_some(manifest)
    })
}
