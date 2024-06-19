mod ast;
mod codegen;

// #[proc_macro_derive(Protobuf, attributes(env))]
// pub fn protobuf_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let input = syn::parse_macro_input!(input as syn::DeriveInput);
//     let _input: ast::Input = match darling::FromDeriveInput::from_derive_input(&input) {
//         Ok(parsed) => parsed,
//         Err(err) => {
//             return err.write_errors().into();
//         }
//     };
//
//     todo!()
// }
