use crate::ast::{Cardinality, Kind, MessageDeriveData, MessageField, OneOfVariant};
use darling::ast::Fields;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Ident;

mod map;
mod messages;
mod oneof;
mod primitives;
mod utils;

pub(crate) fn expand_message(
    root: &proc_macro2::TokenStream,
    input: crate::ast::MessageInput,
) -> TokenStream {
    let ty = input.ident;

    match input.data {
        MessageDeriveData::Enum(variants) => expand_unwrapped_oneof(root, ty, variants),
        MessageDeriveData::Struct(fields) => expand_message_message(root, ty, fields),
    }
}

fn expand_message_message(
    root: &proc_macro2::TokenStream,
    ty: Ident,
    fields: Fields<MessageField>,
) -> TokenStream {
    let span = ty.span();

    let mut serialize_impl = TokenStream::new();
    let mut deserialize_init = TokenStream::new();
    let mut deserialize_impl = TokenStream::new();
    let mut deserialize_set = TokenStream::new();

    for field in fields {
        let tag = field.tag;
        let ty = field.ty;
        let field_ident = field
            .ident
            .clone()
            .expect("named struct fields have idents");
        let span = field_ident.span();

        match field.cardinality.unwrap_or_default() {
            Cardinality::Required => match field.kind.unwrap_or_default() {
                Kind::Primitive => {
                    primitives::required(
                        root,
                        &tag,
                        &field_ident,
                        field.proto,
                        &ty,
                        span,
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::Message => {
                    messages::required(
                        root,
                        &tag,
                        &field_ident,
                        span,
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::OneOf => {
                    oneof::required(
                        root,
                        &tag,
                        &field_ident,
                        &ty,
                        span,
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::Map => {
                    map::required(
                        root,
                        &tag,
                        &field_ident,
                        field.proto_key,
                        field.proto_value,
                        &ty,
                        span,
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
            },
            Cardinality::Optional => match field.kind.unwrap_or_default() {
                Kind::Primitive => {
                    primitives::optional(
                        root,
                        &tag,
                        &field_ident,
                        field.proto,
                        &ty,
                        span,
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::Message => {
                    messages::optional(
                        root,
                        &tag,
                        &field_ident,
                        span,
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::OneOf => {
                    oneof::optional(
                        &field_ident,
                        &ty,
                        span,
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::Map => {
                    map::optional(
                        root,
                        &tag,
                        &field_ident,
                        field.proto_key,
                        field.proto_value,
                        &ty,
                        span,
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
            },
            Cardinality::Repeated => match field.kind.unwrap_or_default() {
                Kind::Primitive => {
                    primitives::repeated(
                        root,
                        &tag,
                        &field_ident,
                        field.proto,
                        &ty,
                        span,
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::Message => {
                    messages::repeated(
                        root,
                        &tag,
                        &field_ident,
                        span,
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::OneOf => {
                    return quote! {
                        compile_error!("A repeated OneOf is not a thing")
                    }
                }
                Kind::Map => {
                    return quote! {
                        compile_error!("A repeated map is not a thing")
                    }
                }
            },
        }
    }

    quote_spanned! {span=>
        #[automatically_derived]
        #[allow(unused_imports)]
        impl #root::PbType for #ty {
            const WIRE_TYPE: u8 = #root::gin_tonic_core::WIRE_TYPE_LENGTH_ENCODED;

            fn encode(&self, encoder: &mut impl #root::Encode) {
                use #root::{Encode, Tag};
                use #root::gin_tonic_core::{WIRE_TYPE_LENGTH_ENCODED, WIRE_TYPE_VARINT};
                use #root::gin_tonic_core::types::{PbOneOf, PbType};

                #serialize_impl
            }

            fn decode(decoder: &mut impl #root::Decode) -> Result<Self, #root::DecodeError>
            where
                Self: Sized
            {
                use #root::{Decode, DecodeError, Tag};
                use #root::gin_tonic_core::{WIRE_TYPE_LENGTH_ENCODED, WIRE_TYPE_VARINT};
                use #root::gin_tonic_core::types::{PbOneOf, PbType};

                #deserialize_init

                while !decoder.eof() {
                    let tag = decoder.decode_uint32()?;
                    let field_number = tag.field_number();
                    let wire_type = tag.wire_type();

                    match field_number {
                        #deserialize_impl
                        n => {
                            return Err(#root::DecodeError::UnexpectedFieldNumber(n))
                        }
                    }
                }

                Ok(Self {
                    #deserialize_set
                })
            }
        }
    }
}

fn expand_unwrapped_oneof(
    root: &proc_macro2::TokenStream,
    ty: Ident,
    variants: Vec<OneOfVariant>,
) -> TokenStream {
    let span = ty.span();

    let mut encode_impl = TokenStream::new();
    let mut decode_impl = TokenStream::new();
    let mut tags = TokenStream::new();

    for variant in variants.into_iter() {
        let Some(field_ty) = variant.fields.fields.first() else {
            panic!("OneOfs must contain fields for now");
        };
        let var_ident = variant.ident;
        let span = var_ident.span();
        let tag = variant.tag;
        let protobuf_type = variant.proto;

        let (pb_type, encode_fn, decode_fn, as_ref) =
            primitives::primitive_types(root, span, field_ty, protobuf_type, false, false);

        tags.extend(quote_spanned! {span=>
            #tag,
        });

        if as_ref {
            encode_impl.extend(quote_spanned! {span=>
                #ty::#var_ident(v) => {
                    #root::gin_tonic_core::encode_field!(#tag, #pb_type, v, encoder, #encode_fn);
                },
            });
        } else {
            encode_impl.extend(quote_spanned! {span=>
                #ty::#var_ident(v) => {
                    #root::gin_tonic_core::encode_field!(#tag, #pb_type, *v, encoder, #encode_fn);
                }
            });
        }

        decode_impl.extend(quote_spanned! { span=>
            #tag => {
                let inner;
                #root::gin_tonic_core::decode_field!(#pb_type, inner, wire_type, decoder, #decode_fn);
                Ok(#ty::#var_ident(inner.ok_or(#root::DecodeError::MissingField(#tag))?))
            },
        });
    }

    quote_spanned! {span=>
        #[automatically_derived]
        #[allow(unused_imports)]
        impl #root::PbType for #ty {
            const WIRE_TYPE: u8 = #root::gin_tonic_core::WIRE_TYPE_LENGTH_ENCODED;

            fn encode(&self, encoder: &mut impl #root::Encode) {
                use #root::{Encode, Tag};

                match self {
                    #encode_impl
                }
            }

            #[allow(clippy::needless_late_init)]
            fn decode(decoder: &mut impl #root::Decode) -> Result<Self, #root::DecodeError>
            where
                Self: Sized
            {
                use #root::{Decode, DecodeError, Tag};
                use #root::gin_tonic_core::{WIRE_TYPE_LENGTH_ENCODED, WIRE_TYPE_VARINT};
                use #root::gin_tonic_core::types::{PbOneOf, PbType};

                let tag = decoder.decode_uint32()?;
                let field_number = tag.field_number();
                let wire_type = tag.wire_type();

                match field_number {
                    #decode_impl
                    n => {
                        Err(#root::DecodeError::UnexpectedFieldNumber(n))
                    }
                }
            }
        }
    }
}

pub(crate) fn expand_enumeration(
    root: &proc_macro2::TokenStream,
    input: crate::ast::EnumerationInput,
) -> TokenStream {
    let ty = input.ident;
    let span = ty.span();

    let variants = input
        .data
        .take_enum()
        .expect("Enumeration derive only works on unit enumerations");

    let mut encode_impl = TokenStream::new();
    let mut decode_impl = TokenStream::new();

    for variant in variants {
        let var_ident = variant.ident;
        let span = var_ident.span();
        let tag = variant.tag;

        encode_impl.extend(quote_spanned! {span=>
            #ty::#var_ident => {
                encoder.encode_uint64(#tag);
            },
        });

        decode_impl.extend(quote_spanned! {span=>
            #tag => Ok(Self::#var_ident),
        });
    }

    quote_spanned! {span=>
        #[automatically_derived]
        #[allow(unused_imports)]
        impl #root::PbType for #ty {
            const WIRE_TYPE: u8 = #root::gin_tonic_core::WIRE_TYPE_VARINT;

            fn encode(&self, encoder: &mut impl #root::Encode) {
                use #root::{Encode, Tag};

                match self {
                    #encode_impl
                }
            }

            fn decode(decoder: &mut impl #root::Decode) -> Result<Self, #root::DecodeError>
            where
                Self: Sized
            {
                use #root::{Decode, DecodeError, Tag};

                match decoder.decode_uint64()? {
                    #decode_impl
                    n => Err(#root::DecodeError::UnexpectedEnumVariant(n)),
                }
            }
        }
    }
}

pub(crate) fn one_of_enumeration(
    root: &proc_macro2::TokenStream,
    input: crate::ast::OneOfInput,
) -> TokenStream {
    let ty = input.ident;

    let variants = input
        .data
        .take_enum()
        .expect("OneOF derive only works on newtype enums");

    let span = ty.span();

    let mut serialize_impl = TokenStream::new();
    let mut deserialize_impl = TokenStream::new();
    let mut tags = TokenStream::new();

    for variant in variants.into_iter() {
        let Some(field_ty) = variant.fields.fields.first() else {
            panic!("OneOfs must contain fields for now");
        };
        let var_ident = variant.ident;
        let span = var_ident.span();
        let tag = variant.tag;
        let protobuf_type = variant.proto;

        let (pb_type, encode_fn, decode_fn, as_ref) =
            primitives::primitive_types(root, span, field_ty, protobuf_type, false, false);

        tags.extend(quote_spanned! {span=>
            #tag,
        });

        if as_ref {
            serialize_impl.extend(quote_spanned! {span=>
                #ty::#var_ident(v) => {
                    #root::gin_tonic_core::encode_field!(#tag, #pb_type, v, encoder, #encode_fn);
                },
            });
        } else {
            serialize_impl.extend(quote_spanned! {span=>
                #ty::#var_ident(v) => {
                    #root::gin_tonic_core::encode_field!(#tag, #pb_type, *v, encoder, #encode_fn);
                }
            });
        }

        deserialize_impl.extend(quote_spanned! { span=>
            #tag => {
                let inner;
                #root::gin_tonic_core::decode_field!(#pb_type, inner, wire_type, decoder, #decode_fn);
                Ok(#ty::#var_ident(inner.ok_or(#root::DecodeError::MissingField(#tag))?))
            },
        });
    }

    quote_spanned! {span=>
        #[automatically_derived]
        #[allow(unused_imports)]
        impl #root::PbOneOf for #ty {
            fn matches(field_number: u32) -> bool {
                [#tags].contains(&field_number)
            }

            fn encode(&self, encoder: &mut impl #root::Encode) {
                use #root::{Encode, Tag};
                use #root::gin_tonic_core::{WIRE_TYPE_LENGTH_ENCODED, WIRE_TYPE_VARINT};
                use #root::gin_tonic_core::types::{PbOneOf, PbType};

                match self {
                    #serialize_impl
                };
            }

            #[allow(clippy::needless_late_init)]
            fn decode(
                field_number: u32,
                wire_type: u8,
                decoder: &mut impl #root::Decode,
            ) -> Result<Self, #root::DecodeError>
            where
                Self: Sized,
            {
                use #root::{Decode, DecodeError, Tag};
                use #root::gin_tonic_core::{WIRE_TYPE_LENGTH_ENCODED, WIRE_TYPE_VARINT};
                use #root::gin_tonic_core::types::{PbOneOf, PbType};

                match field_number {
                    #deserialize_impl
                    n => Err(#root::DecodeError::UnexpectedOneOfVariant(n)),
                }
            }
        }
    }
}
