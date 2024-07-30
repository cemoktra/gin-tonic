use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::{Ident, LitInt, Type};

pub fn required(
    root: &proc_macro2::TokenStream,
    tag: &LitInt,
    field_ident: &Ident,
    ty: &Type,
    span: Span,
    encode_impl: &mut TokenStream,
    decode_init: &mut TokenStream,
    decode_impl: &mut TokenStream,
    decode_set: &mut TokenStream,
) {
    encode_impl.extend(quote_spanned! { span=>
        self.#field_ident.encode(encoder);
    });

    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = None;
    });

    decode_impl.extend(quote_spanned! { span=>
        n if #ty::matches(n) => #field_ident = Some(#ty::decode(n, wire_type, decoder)?),
    });

    decode_set.extend(quote_spanned! { span=>
        #field_ident: #field_ident.ok_or(#root::DecodeError::MissingField(#tag))?,
    });
}

pub fn optional(
    field_ident: &Ident,
    ty: &Type,
    span: Span,
    encode_impl: &mut TokenStream,
    decode_init: &mut TokenStream,
    decode_impl: &mut TokenStream,
    decode_set: &mut TokenStream,
) {
    let inner_ty = match ty {
        Type::Path(path) => {
            let Some(segment) = path.path.segments.first() else {
                panic!("optional must be Option<T>");
            };
            match &segment.arguments {
                syn::PathArguments::AngleBracketed(arguments) => {
                    let Some(argument) = arguments.args.first() else {
                        panic!("optional must be Option<T>");
                    };

                    match argument {
                        syn::GenericArgument::Type(ty) => ty,
                        _ => panic!("optional must be Option<T>"),
                    }
                }
                _ => panic!("optional must be Option<T>"),
            }
        }
        _ => panic!("optional must be Option<T>"),
    };

    encode_impl.extend(quote_spanned! { span=>
        if let Some(value) = &self.#field_ident {
            value.encode(encoder);
        }
    });

    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = None;
    });

    decode_impl.extend(quote_spanned! { span=>
        n if #inner_ty::matches(n) => #field_ident = Some(#inner_ty::decode(n, wire_type, decoder)?),
    });

    decode_set.extend(quote_spanned! { span=>
        #field_ident,
    });
}
