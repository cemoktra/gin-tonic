//! this crate adds derive macros for implementing [PbType](https://docs.rs/gin-tonic-core/latest/gin_tonic_core/types/trait.PbType.html) and
//! [https://docs.rs/gin-tonic-core/latest/gin_tonic_core/types/trait.PbOneOf.html]
pub(crate) mod ast;
pub(crate) mod codegen;

#[proc_macro_derive(Message, attributes(gin))]
/// a macro to derive protobof messages
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
/// a macro to derive enumerations
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
/// a macro to derive a message as oneof if it only contains a oneof
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
