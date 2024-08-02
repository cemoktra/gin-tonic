//! this crate adds derive macros for implementing [PbType] and [PbOneOf]
pub(crate) mod ast;
pub(crate) mod codegen;

#[proc_macro_derive(Message, attributes(gin))]
pub fn message_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let input: ast::MessageInput = match darling::FromDeriveInput::from_derive_input(&input) {
        Ok(parsed) => parsed,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let root = input
        .root
        .as_ref()
        .map(|path| quote::quote!(#path))
        .unwrap_or(quote::quote! {
            ::gin_tonic
        });

    codegen::expand_message(&root, input).into()
}

#[proc_macro_derive(Enumeration, attributes(gin))]
pub fn enum_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let input: ast::EnumerationInput = match darling::FromDeriveInput::from_derive_input(&input) {
        Ok(parsed) => parsed,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let root = input
        .root
        .as_ref()
        .map(|path| quote::quote!(#path))
        .unwrap_or(quote::quote! {
            ::gin_tonic
        });

    codegen::expand_enumeration(&root, input).into()
}

#[proc_macro_derive(OneOf, attributes(gin))]
pub fn one_of_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let input: ast::OneOfInput = match darling::FromDeriveInput::from_derive_input(&input) {
        Ok(parsed) => parsed,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let root = input
        .root
        .as_ref()
        .map(|path| quote::quote!(#path))
        .unwrap_or(quote::quote! {
            ::gin_tonic
        });

    codegen::one_of_enumeration(&root, input).into()
}
