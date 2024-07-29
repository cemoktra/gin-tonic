use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::{Ident, LitInt, Type};

use crate::ast::{MessageField, Primitive};

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
        #root::gin_tonic_core::encode_nested!(#tag, &self.#field_ident, encoder, Encode::encode_nested);
    });

    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = None;
    });

    decode_impl.extend(quote_spanned! { span=>
        #tag => {
            #root::gin_tonic_core::decode_nested!(#field_ident, wire_type, decoder, Decode::decode_nested);
        },
    });

    decode_set.extend(quote_spanned! { span=>
        #field_ident: #field_ident.ok_or(#root::DecodeError::MissingField(#tag))?,
    });
}

pub fn optional(
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
        if let Some(value) = self.#field_ident {
            #root::gin_tonic_core::encode_nested!(#tag, &value, encoder, Encode::encode_nested);
        }
    });

    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = None;
    });

    decode_impl.extend(quote_spanned! { span=>
        #tag => {
            #root::gin_tonic_core::decode_nested!(#field_ident, wire_type, decoder, Decode::decode_nested)
        },
    });

    decode_set.extend(quote_spanned! { span=>
        #field_ident,
    });
}

pub fn repeated(
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
        if let Some(value) = self.#field_ident {
            #root::gin_tonic_core::encode_vector_unpacked!(#tag, #ty, &self.#field_ident, encoder, Encode::encode_nested);
        }
    });

    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = vec![];
    });

    decode_impl.extend(quote_spanned! { span=>
        #tag => {
            #root::gin_tonic_core::decode_vector!(#ty, &mut #field_ident, wire_type, decoder, Decode::decode_nested)
        },
    });

    decode_set.extend(quote_spanned! { span=>
        #field_ident: #field_ident.ok_or(#root::DecodeError::MissingField(#tag))?,
    });
}
