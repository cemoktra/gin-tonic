use darling::ast::Data;
use darling::{FromDeriveInput, FromField, FromMeta, FromVariant};
use syn::{Ident, LitInt, Type};

pub(crate) type MessageDeriveData = Data<(), MessageField>;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(gin), supports(struct_named))]
pub(crate) struct MessageInput {
    pub(crate) ident: Ident,
    pub(crate) data: MessageDeriveData,
}

#[derive(Clone, Debug, FromField)]
#[darling(attributes(gin))]
pub(crate) struct MessageField {
    pub(crate) ident: Option<Ident>,
    pub(crate) ty: Type,

    pub(crate) tag: LitInt,
    #[darling(default)]
    pub(crate) cardinality: Cardinality,
    #[darling(default)]
    pub(crate) kind: Kind,
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
}

#[derive(Clone, Debug, FromVariant)]
#[darling(attributes(gin))]
pub(crate) struct EnumVariant {
    pub(crate) ident: Ident,
    pub(crate) tag: LitInt,
}
