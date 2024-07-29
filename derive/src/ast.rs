use darling::ast::Data;
use darling::{FromDeriveInput, FromField, FromMeta, FromVariant};
use syn::{Ident, LitInt, Type};

pub(crate) type MessageDeriveData = Data<OneOfVariant, MessageField>;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(gin), supports(struct_named, enum_tuple))]
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

    pub(crate) tag: LitInt,
    pub(crate) cardinality: Option<Cardinality>,
    pub(crate) kind: Option<Kind>,
    pub(crate) proto: Option<Primitive>,
    pub(crate) proto_key: Option<Primitive>,
    pub(crate) proto_value: Option<Primitive>,
}

#[derive(Clone, Copy, Debug, FromMeta)]
pub(crate) enum Primitive {
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
}

#[derive(Clone, Copy, Default, Debug, FromMeta)]
pub(crate) enum Cardinality {
    #[default]
    Required,
    Optional,
    Repeated,
}

#[derive(Clone, Copy, Default, Debug, FromMeta)]
pub(crate) enum Kind {
    #[default]
    Primitive,
    Message,
    OneOf,
    Map,
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
    pub(crate) tag: LitInt,
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
    pub(crate) tag: LitInt,
}
