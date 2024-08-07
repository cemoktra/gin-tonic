use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{Ident, LitInt, Type};

use crate::ast::Primitive;

fn primitive_types(
    root: &proc_macro2::TokenStream,
    ty: &Type,
    protobuf_type: Option<Primitive>,
) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
    match protobuf_type {
        Some(Primitive::Float) => {
            let wire_type = quote! { f32::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_float };
            let decode_fn = quote! { Decode::decode_float };
            let access = quote! { f32::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Double) => {
            let wire_type = quote! { f64::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_double };
            let decode_fn = quote! { Decode::decode_double };
            let access = quote! { f64::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Int32) => {
            let wire_type = quote! { #root::types::Int32::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_int32 };
            let decode_fn = quote! { Decode::decode_int32 };
            let access = quote! { i32::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Int64) => {
            let wire_type = quote! { #root::types::Int64::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_int64 };
            let decode_fn = quote! { Decode::decode_int64 };
            let access = quote! { i64::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Uint32) => {
            let wire_type = quote! { #root::types::UInt32::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_uint32 };
            let decode_fn = quote! { Decode::decode_uint32 };
            let access = quote! { u32::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Uint64) => {
            let wire_type = quote! { #root::types::UInt64::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_uint64 };
            let decode_fn = quote! { Decode::decode_uint64 };
            let access = quote! { u64::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Sint32) => {
            let wire_type = quote! { #root::types::SInt32::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_sint32 };
            let decode_fn = quote! { Decode::decode_sint32 };
            let access = quote! { i32::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Sint64) => {
            let wire_type = quote! { #root::types::SInt64::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_sint64 };
            let decode_fn = quote! { Decode::decode_sint64 };
            let access = quote! { i64::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Fixed32) => {
            let wire_type = quote! { #root::types::Fixed32::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_fixed32 };
            let decode_fn = quote! { Decode::decode_fixed32 };
            let access = quote! { u32::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Fixed64) => {
            let wire_type = quote! { #root::types::Fixed64::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_fixed64 };
            let decode_fn = quote! { Decode::decode_fixed64 };
            let access = quote! { u64::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Sfixed32) => {
            let wire_type = quote! { #root::types::SFixed32::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_sfixed32 };
            let decode_fn = quote! { Decode::decode_sfixed32 };
            let access = quote! { i32::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Sfixed64) => {
            let wire_type = quote! { #root::types::SFixed64::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_sfixed64 };
            let decode_fn = quote! { Decode::decode_sfixed64 };
            let access = quote! { i64::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::String) => {
            let wire_type = quote! { String::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_str };
            let decode_fn = quote! { Decode::decode_string };
            let access = quote! { String::as_str };
            (wire_type, encode_fn, decode_fn, access)
        }
        Some(Primitive::Bool) => {
            let wire_type = quote! { bool::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_bool };
            let decode_fn = quote! { Decode::decode_bool };
            let access = quote! { bool::clone };
            (wire_type, encode_fn, decode_fn, access)
        }
        None => {
            let wire_type = quote! { #ty::WIRE_TYPE };
            let encode_fn = quote! { Encode::encode_type };
            let decode_fn = quote! { Decode::decode_type };
            // TODO: figure this out
            let access = quote! {};
            (wire_type, encode_fn, decode_fn, access)
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn required(
    root: &proc_macro2::TokenStream,
    tag: &LitInt,
    field_ident: &Ident,
    key_protobuf_type: Option<Primitive>,
    value_protobuf_type: Option<Primitive>,
    ty: &Type,
    span: Span,
    encode_impl: &mut TokenStream,
    decode_init: &mut TokenStream,
    decode_impl: &mut TokenStream,
    decode_set: &mut TokenStream,
) {
    let (key_wire_type, key_encode_fn, key_decode_fn, key_access) =
        primitive_types(root, ty, key_protobuf_type);
    let (value_wire_type, value_encode_fn, value_decode_fn, value_access) =
        primitive_types(root, ty, value_protobuf_type);

    encode_impl.extend(quote_spanned! { span=>
        #root::gin_tonic_core::encode_map!(
            #tag,
            &self.#field_ident,
            #key_access,
            #value_access,
            #key_wire_type,
            #value_wire_type,
            encoder,
            #key_encode_fn,
            #value_encode_fn,
        );
    });

    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = std::collections::HashMap::new();
    });

    decode_impl.extend(quote_spanned! { span=>
        #tag => {
            #root::gin_tonic_core::decode_map!(
                &mut #field_ident,
                wire_type,
                decoder,
                #key_decode_fn,
                #value_decode_fn
            );
        },
    });

    decode_set.extend(quote_spanned! { span=>
        #field_ident,
    });
}

#[allow(clippy::too_many_arguments)]
pub fn optional(
    root: &proc_macro2::TokenStream,
    tag: &LitInt,
    field_ident: &Ident,
    key_protobuf_type: Option<Primitive>,
    value_protobuf_type: Option<Primitive>,
    ty: &Type,
    span: Span,
    encode_impl: &mut TokenStream,
    decode_init: &mut TokenStream,
    decode_impl: &mut TokenStream,
    decode_set: &mut TokenStream,
) {
    let (key_wire_type, key_encode_fn, key_decode_fn, key_access) =
        primitive_types(root, ty, key_protobuf_type);
    let (value_wire_type, value_encode_fn, value_decode_fn, value_access) =
        primitive_types(root, ty, value_protobuf_type);

    encode_impl.extend(quote_spanned! { span=>
        if let Some(map) = &self.#field_ident {
            #root::gin_tonic_core::encode_map!(
                #tag,
                map,
                #key_access,
                #value_access,
                #key_wire_type,
                #value_wire_type,
                encoder,
                #key_encode_fn,
                #value_encode_fn,
            );
        }
    });

    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = None;
    });

    decode_impl.extend(quote_spanned! { span=>
        #tag => {
            let map = match #field_ident.as_mut() {
                Some(map) => map,
                None => {
                    #field_ident = Some(std::collections::HashMap::new());
                    #field_ident.as_mut().expect("value has been set to some")
                }
            };
            #root::gin_tonic_core::decode_map!(
                map,
                wire_type,
                decoder,
                #key_decode_fn,
                #value_decode_fn
            );
        },
    });

    decode_set.extend(quote_spanned! { span=>
        #field_ident,
    });
}
