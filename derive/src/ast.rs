use darling::ast::Data;
use darling::util::Flag;
use darling::{FromDeriveInput, FromField, FromMeta, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Ident, LitBool, LitInt, PathArguments, PathSegment, Type};

pub(crate) type MessageDeriveData = Data<(), MessageField>;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(gin), supports(struct_named))]
pub(crate) struct MessageInput {
    pub(crate) ident: Ident,
    pub(crate) data: MessageDeriveData,
    pub(crate) root: Option<syn::Path>,
}

#[derive(Clone, Debug, FromField)]
#[darling(attributes(gin))]
#[allow(clippy::manual_unwrap_or_default)]
pub(crate) struct MessageField {
    pub(crate) ident: Option<Ident>,
    pub(crate) ty: Type,

    pub(crate) id: LitInt,
    pub(crate) oneof: Flag,
    pub(crate) packed: Option<LitBool>,
    pub(crate) scalar: Option<Scalar>,
    pub(crate) key_scalar: Option<Scalar>,
    pub(crate) value_scalar: Option<Scalar>,
}

#[derive(Clone, Copy, Debug, FromMeta)]
pub(crate) enum Scalar {
    Float,
    Double,
    Int32,
    Int64,
    Uint32,
    Uint64,
    Sint32,
    Sint64,
    Fixed32,
    Fixed64,
    Sfixed32,
    Sfixed64,
    String,
    Bool,
    Bytes,
}

pub(crate) trait ScalarToken {
    fn scalar_token(&self, root: &proc_macro2::TokenStream) -> TokenStream;
}

impl ScalarToken for syn::Type {
    fn scalar_token(&self, root: &proc_macro2::TokenStream) -> TokenStream {
        match self {
            Type::Path(type_path) => {
                let ident = type_path.path.get_ident();

                if let Some(segment) = type_path.path.segments.last()
                    && segment.ident == "Ipv4Addr"
                {
                    return quote! { #root::scalars::UInt32 };
                }

                match ident {
                    Some(ident) => {
                        let text = ident.to_string();

                        match text.as_str() {
                            "i32" => quote!(compile_error!("i32 needs scalar type")),
                            "i64" => quote!(compile_error!("i64 needs scalar type")),
                            "u32" => quote!(compile_error!("u32 needs scalar type")),
                            "u64" => quote!(compile_error!("u64 needs scalar type")),
                            "String" => quote! { #root::scalars::ProtoString },
                            "bool" => quote! { #root::scalars::Bool },
                            "f32" => quote! { #root::scalars::Float },
                            "f64" => quote! { #root::scalars::Double },
                            _ => quote! { #root::scalars::Bytes },
                        }
                    }
                    None => {
                        quote! { #root::scalars::Bytes }
                    }
                }
            }
            _ => todo!("impl ScalarToken for syn::Type = {self:?}"),
        }
    }
}

impl ScalarToken for Scalar {
    fn scalar_token(&self, root: &proc_macro2::TokenStream) -> TokenStream {
        match self {
            Scalar::Float => quote! { #root::scalars::Float },
            Scalar::Double => quote! { #root::scalars::Double },
            Scalar::Int32 => quote! { #root::scalars::Int32 },
            Scalar::Int64 => quote! { #root::scalars::Int64 },
            Scalar::Uint32 => quote! { #root::scalars::UInt32 },
            Scalar::Uint64 => quote! { #root::scalars::UInt64 },
            Scalar::Sint32 => quote! { #root::scalars::SInt32 },
            Scalar::Sint64 => quote! { #root::scalars::SInt64 },
            Scalar::Fixed32 => quote! { #root::scalars::Fixed32 },
            Scalar::Fixed64 => quote! { #root::scalars::Fixed64 },
            Scalar::Sfixed32 => quote! { #root::scalars::SFixed32 },
            Scalar::Sfixed64 => quote! { #root::scalars::SFixed64 },
            Scalar::String => quote! { #root::scalars::ProtoString },
            Scalar::Bool => quote! { #root::scalars::Bool },
            Scalar::Bytes => quote! { #root::scalars::Bytes },
        }
    }
}

pub(crate) trait IsOption {
    fn is_option(&self) -> Option<&syn::Type>;
}

impl IsOption for syn::Type {
    fn is_option(&self) -> Option<&syn::Type> {
        match self {
            Type::Path(type_path) => {
                // Get the last segment (e.g., 'Option' in 'std::option::Option')
                let segment: &PathSegment = type_path.path.segments.last()?;

                if segment.ident != "Option" {
                    return None;
                }

                // Look for the angle brackets: <T>
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    // We expect exactly one generic argument: the inner type
                    if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }

                None
            }
            _ => None,
        }
    }
}

pub(crate) trait IsRepeated {
    fn is_repeated(&self) -> Option<&syn::Type>;
}

impl IsRepeated for syn::Type {
    fn is_repeated(&self) -> Option<&syn::Type> {
        match self {
            Type::Path(type_path) => {
                // Get the last segment (e.g., 'Vec' in 'std::option::Option')
                let segment: &PathSegment = type_path.path.segments.last()?;

                if segment.ident != "Vec" {
                    return None;
                }

                // Look for the angle brackets: <T>
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    // We expect exactly one generic argument: the inner type
                    if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
                        return Some(inner_type);
                    }
                }

                None
            }
            _ => None,
        }
    }
}

pub(crate) trait IsMap {
    fn is_map(&self) -> Option<(&syn::Type, &syn::Type)>;
}

impl IsMap for syn::Type {
    fn is_map(&self) -> Option<(&syn::Type, &syn::Type)> {
        match self {
            Type::Path(type_path) => {
                // Get the last segment (e.g., 'IndexMap' in 'std::option::Option')
                let segment: &PathSegment = type_path.path.segments.last()?;

                // TODO: also add HashMap
                if segment.ident != "IndexMap" {
                    return None;
                }

                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    // IndexMap requires two type arguments: IndexMap<K, V>
                    let mut types = args.args.iter().filter_map(|arg| {
                        if let GenericArgument::Type(inner_type) = arg {
                            Some(inner_type)
                        } else {
                            None
                        }
                    });

                    match (types.next(), types.next()) {
                        (Some(k), Some(v)) => return Some((k, v)),
                        _ => return None,
                    }
                }

                None
            }
            _ => None,
        }
    }
}

pub(crate) trait IsPackable {
    fn is_packable(&self) -> bool;
}

impl IsPackable for Scalar {
    fn is_packable(&self) -> bool {
        !matches!(self, Scalar::Bytes | Scalar::String)
    }
}

impl IsPackable for syn::Type {
    fn is_packable(&self) -> bool {
        match self {
            Type::Path(type_path) => {
                if let Some(segment) = type_path.path.segments.last()
                    && segment.ident == "Ipv4Addr"
                {
                    return true;
                }

                let ident = type_path.path.get_ident();
                match ident {
                    Some(ident) => {
                        let text = ident.to_string();

                        matches!(
                            text.as_str(),
                            "i32" | "i64" | "u32" | "u64" | "bool" | "f32" | "f64"
                        )
                    }
                    None => false,
                }
            }
            _ => false,
        }
    }
}

pub(crate) type EnumDeriveData = Data<EnumVariant, ()>;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(gin), supports(enum_unit))]
pub(crate) struct EnumerationInput {
    pub(crate) ident: Ident,
    pub(crate) data: EnumDeriveData,
    pub(crate) root: Option<syn::Path>,
}

#[derive(Clone, Debug, FromVariant)]
#[darling(attributes(gin))]
pub(crate) struct EnumVariant {
    pub(crate) ident: Ident,
    pub(crate) id: LitInt,
}

pub(crate) type OneOfDeriveData = Data<OneOfVariant, ()>;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(gin), supports(enum_tuple))]
pub(crate) struct OneOfInput {
    pub(crate) ident: Ident,

    pub(crate) data: OneOfDeriveData,
    pub(crate) root: Option<syn::Path>,
}

#[derive(Clone, Debug, FromVariant)]
#[darling(attributes(gin))]
pub(crate) struct OneOfVariant {
    pub(crate) ident: Ident,
    pub(crate) fields: darling::ast::Fields<syn::Type>,

    pub(crate) id: LitInt,
    pub(crate) scalar: Option<Scalar>,
}
