// use darling::ast::Data;
// use darling::{FromDeriveInput, FromField};
// use syn::{Ident, Type};
//
// pub(crate) type DeriveData = Data<(), Field>;
//
// #[derive(Debug, FromDeriveInput)]
// #[darling(attributes(gin), supports(struct_named))]
// pub(crate) struct Input {
//     pub ident: Ident,
//     pub data: DeriveData,
// }
//
// #[derive(Clone, Debug, FromField)]
// #[darling(attributes(gin))]
// pub(crate) struct Field {
//     pub ident: Option<Ident>,
//     pub ty: Type,
// }
