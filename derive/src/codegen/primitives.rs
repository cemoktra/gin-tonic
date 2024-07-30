use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::{Ident, LitInt, Type};

use crate::{ast::Primitive, codegen::utils::inner_type};

pub(super) fn primitive_types(
    root: &proc_macro2::TokenStream,
    span: Span,
    ty: &Type,
    protobuf_type: Option<Primitive>,
    repeated: bool,
    optional: bool,
) -> (TokenStream, TokenStream, TokenStream, bool) {
    match protobuf_type {
        Some(Primitive::Float) => {
            let pb_type = quote_spanned! { span=>f32 };
            let encode_fn = quote_spanned! { span=>Encode::encode_float };
            let decode_fn = quote_spanned! { span=>Decode::decode_float };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::Double) => {
            let pb_type = quote_spanned! { span=>f64 };
            let encode_fn = quote_spanned! { span=>Encode::encode_double };
            let decode_fn = quote_spanned! { span=>Decode::decode_double };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::Int32) => {
            let pb_type = quote_spanned! { span=>#root::types::Int32 };
            let encode_fn = quote_spanned! { span=>Encode::encode_int32 };
            let decode_fn = quote_spanned! { span=>Decode::decode_int32 };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::Int64) => {
            let pb_type = quote_spanned! { span=>#root::types::Int64 };
            let encode_fn = quote_spanned! { span=>Encode::encode_int64 };
            let decode_fn = quote_spanned! { span=>Decode::decode_int64 };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::Uint32) => {
            let pb_type = quote_spanned! { span=>#root::types::UInt32 };
            let encode_fn = quote_spanned! { span=>Encode::encode_uint32 };
            let decode_fn = quote_spanned! { span=>Decode::decode_uint32 };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::Uint64) => {
            let pb_type = quote_spanned! { span=>#root::types::UInt64 };
            let encode_fn = quote_spanned! { span=>Encode::encode_uint64 };
            let decode_fn = quote_spanned! { span=>Decode::decode_uint64 };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::Sint32) => {
            let pb_type = quote_spanned! { span=>#root::types::SInt32 };
            let encode_fn = quote_spanned! { span=>Encode::encode_sint32 };
            let decode_fn = quote_spanned! { span=>Decode::decode_sint32 };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::Sint64) => {
            let pb_type = quote_spanned! { span=>#root::types::SInt64 };
            let encode_fn = quote_spanned! { span=>Encode::encode_sint64 };
            let decode_fn = quote_spanned! { span=>Decode::decode_sint64 };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::Fixed32) => {
            let pb_type = quote_spanned! { span=>#root::types::Fixed32 };
            let encode_fn = quote_spanned! { span=>Encode::encode_fixed32 };
            let decode_fn = quote_spanned! { span=>Decode::decode_fixed32 };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::Fixed64) => {
            let pb_type = quote_spanned! { span=>#root::types::Fixed64 };
            let encode_fn = quote_spanned! { span=>Encode::encode_fixed64 };
            let decode_fn = quote_spanned! { span=>Decode::decode_fixed64 };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::Sfixed32) => {
            let pb_type = quote_spanned! { span=>#root::types::SFixed32 };
            let encode_fn = quote_spanned! { span=>Encode::encode_sfixed32 };
            let decode_fn = quote_spanned! { span=>Decode::decode_sfixed32 };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::Sfixed64) => {
            let pb_type = quote_spanned! { span=>#root::types::SFixed64 };
            let encode_fn = quote_spanned! { span=>Encode::encode_sfixed64 };
            let decode_fn = quote_spanned! { span=>Decode::decode_sfixed64 };
            (pb_type, encode_fn, decode_fn, false)
        }
        Some(Primitive::String) => {
            let pb_type = quote_spanned! { span=>String };
            let encode_fn = quote_spanned! { span=>Encode::encode_str };
            let decode_fn = quote_spanned! { span=>Decode::decode_string };
            (pb_type, encode_fn, decode_fn, true)
        }
        Some(Primitive::Bool) => {
            let pb_type = quote_spanned! { span=>bool };
            let encode_fn = quote_spanned! { span=>Encode::encode_bool };
            let decode_fn = quote_spanned! { span=>Decode::decode_bool };
            (pb_type, encode_fn, decode_fn, false)
        }
        None => {
            let pb_type = if optional || repeated {
                let inner_ty = inner_type(ty);
                quote_spanned! { span=>#inner_ty }
            } else {
                quote_spanned! { span=>#ty }
            };

            let encode_fn = quote_spanned! { span=>Encode::encode_type };
            let decode_fn = quote_spanned! { span=>Decode::decode_type };
            (pb_type, encode_fn, decode_fn, true)
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn required(
    root: &proc_macro2::TokenStream,
    tag: &LitInt,
    field_ident: &Ident,
    protobuf_type: Option<Primitive>,
    ty: &Type,
    span: Span,
    encode_impl: &mut TokenStream,
    decode_init: &mut TokenStream,
    decode_impl: &mut TokenStream,
    decode_set: &mut TokenStream,
) {
    let (pb_type, encode_fn, decode_fn, as_ref) =
        primitive_types(root, span, ty, protobuf_type, false, false);

    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = None;
    });
    decode_set.extend(quote_spanned! { span=>
        #field_ident: #field_ident.ok_or(#root::DecodeError::MissingField(#tag))?,
    });

    if as_ref {
        encode_impl.extend(quote_spanned! { span=>
            #root::gin_tonic_core::encode_field!(#tag, #pb_type, &self.#field_ident, encoder, #encode_fn);
        });
    } else {
        encode_impl.extend(quote_spanned! { span=>
            #root::gin_tonic_core::encode_field!(#tag, #pb_type, self.#field_ident, encoder, #encode_fn);
        });
    }

    decode_impl.extend(quote_spanned! { span=>
        #tag => {
            #root::gin_tonic_core::decode_field!(#pb_type, #field_ident, wire_type, decoder, #decode_fn);
        },
    });
}

#[allow(clippy::too_many_arguments)]
pub fn optional(
    root: &proc_macro2::TokenStream,
    tag: &LitInt,
    field_ident: &Ident,
    protobuf_type: Option<Primitive>,
    ty: &Type,
    span: Span,
    encode_impl: &mut TokenStream,
    decode_init: &mut TokenStream,
    decode_impl: &mut TokenStream,
    decode_set: &mut TokenStream,
) {
    let (pb_type, encode_fn, decode_fn, as_ref) =
        primitive_types(root, span, ty, protobuf_type, false, true);

    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = None;
    });
    decode_set.extend(quote_spanned! { span=>
        #field_ident,
    });

    if as_ref {
        encode_impl.extend(quote_spanned! { span=>
            if let Some(value) = &self.#field_ident {
                #root::gin_tonic_core::encode_field!(#tag, #pb_type, value, encoder, #encode_fn);
            }
        });
    } else {
        encode_impl.extend(quote_spanned! { span=>
            if let Some(value) = self.#field_ident {
                #root::gin_tonic_core::encode_field!(#tag, #pb_type, value, encoder, #encode_fn);
            }
        });
    }

    decode_impl.extend(quote_spanned! { span=>
        #tag => {
            #root::gin_tonic_core::decode_field!(#pb_type, #field_ident, wire_type, decoder, #decode_fn)
        },
    });
}

#[allow(clippy::too_many_arguments)]
pub fn repeated(
    root: &proc_macro2::TokenStream,
    tag: &LitInt,
    field_ident: &Ident,
    protobuf_type: Option<Primitive>,
    ty: &Type,
    span: Span,
    encode_impl: &mut TokenStream,
    decode_init: &mut TokenStream,
    decode_impl: &mut TokenStream,
    decode_set: &mut TokenStream,
) {
    let (pb_type, encode_fn, decode_fn, unpacked) =
        primitive_types(root, span, ty, protobuf_type, true, false);

    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = vec![];
    });
    decode_set.extend(quote_spanned! { span=>
        #field_ident,
    });

    if unpacked {
        encode_impl.extend(quote_spanned! { span=>
            #root::gin_tonic_core::encode_vector_unpacked!(#tag, #pb_type, &self.#field_ident, encoder, #encode_fn);
        });
    } else {
        encode_impl.extend(quote_spanned! { span=>
            #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, #encode_fn);
        });
    }

    decode_impl.extend(quote_spanned! { span=>
        #tag => {
            #root::gin_tonic_core::decode_vector!(#pb_type, &mut #field_ident, wire_type, decoder, #decode_fn)
        }
    });
}
