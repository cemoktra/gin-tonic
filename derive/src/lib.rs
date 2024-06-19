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

    codegen::expand_message(input).into()
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

    codegen::expand_enumeration(input).into()
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

    codegen::one_of_enumeration(input).into()
}
