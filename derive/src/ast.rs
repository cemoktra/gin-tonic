use darling::ast::Data;
use darling::{FromDeriveInput, FromField, FromMeta};
use syn::{Ident, LitInt, Type};

pub(crate) type DeriveData = Data<(), MessageField>;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(gin), supports(struct_named))]
pub(crate) struct MessageInput {
    pub(crate) ident: Ident,
    pub(crate) data: DeriveData,
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
