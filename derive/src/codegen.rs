use crate::ast::{Cardinality, Kind, MessageDeriveData, MessageField, OneOfVariant, Primitive};
use darling::ast::Fields;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

mod map;
mod messages;
mod primitives;

pub(crate) fn expand_message(
    root: &proc_macro2::TokenStream,
    input: crate::ast::MessageInput,
) -> TokenStream {
    let ty = input.ident;

    match input.data {
        MessageDeriveData::Enum(variants) => panic!("TODO: unwrapped oneof"), //expand_unwrapped_oneof(root, ty, variants),
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
        let field_size_ident = format_ident!("{}_size", field_ident);

        match field.cardinality.unwrap_or_default() {
            Cardinality::Required => match field.kind.unwrap_or_default() {
                Kind::Primitive => {
                    primitives::required(
                        &root,
                        &tag,
                        &field_ident,
                        field.proto,
                        &ty,
                        span.clone(),
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::Message => {
                    messages::required(
                        &root,
                        &tag,
                        &field_ident,
                        &ty,
                        span.clone(),
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::OneOf => {
                    serialize_impl.extend(quote_spanned! { span=>
                        //#root::gin_tonic_core::Message::serialize(self.#field_ident, writer);
                        todo!()
                    });

                    deserialize_init.extend(quote_spanned! { span=>
                        let mut #field_ident = None;
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        todo!()
                        // tag if <#ty as #root::gin_tonic_core::OneOf>::matches_tag(tag) => {
                        //     #field_ident = Some(#root::gin_tonic_core::OneOf::deserialize_wire(tag, wire_type)?);
                        // }
                    });

                    deserialize_set.extend(quote_spanned! { span=>
                        #field_ident: #field_ident.ok_or(#root::DecodeError::MissingField(#tag))?,
                    });
                }
                Kind::Map => {
                    map::required(
                        &root,
                        &tag,
                        &field_ident,
                        field.proto_key,
                        field.proto_value,
                        &ty,
                        span.clone(),
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
                        &root,
                        &tag,
                        &field_ident,
                        field.proto,
                        &ty,
                        span.clone(),
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::Message => {
                    messages::optional(
                        &root,
                        &tag,
                        &field_ident,
                        &ty,
                        span.clone(),
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::OneOf => {
                    serialize_impl.extend(quote_spanned! { span=>
                        todo!()
                        // if let Some(value) = self.#field_ident {
                        //     #root::gin_tonic_core::Message::serialize(value, writer);
                        // }
                    });

                    deserialize_init.extend(quote_spanned! { span=>
                        let mut #field_ident = None;
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        todo!()
                        // tag if #ty::matches_tag(tag) => {
                        //     #field_ident = Some(#root::gin_tonic_core::OneOf::deserialize_wire(tag, wire_type)?);
                        // }
                    });

                    deserialize_set.extend(quote_spanned! { span=>
                        #field_ident,
                    });
                }
                Kind::Map => {
                    map::optional(
                        &root,
                        &tag,
                        &field_ident,
                        field.proto_key,
                        field.proto_value,
                        &ty,
                        span.clone(),
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
                        &root,
                        &tag,
                        &field_ident,
                        field.proto,
                        &ty,
                        span.clone(),
                        &mut serialize_impl,
                        &mut deserialize_init,
                        &mut deserialize_impl,
                        &mut deserialize_set,
                    );
                }
                Kind::Message => {
                    messages::repeated(
                        &root,
                        &tag,
                        &field_ident,
                        span.clone(),
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

                #serialize_impl
            }

            fn decode(decoder: &mut impl #root::Decode) -> Result<Self, #root::DecodeError>
            where
                Self: Sized
            {
                use #root::{Decode, DecodeError, Tag};
                use #root::gin_tonic_core::{WIRE_TYPE_LENGTH_ENCODED, WIRE_TYPE_VARINT};

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

// fn expand_unwrapped_oneof(
//     root: &proc_macro2::TokenStream,
//     ty: Ident,
//     variants: Vec<OneOfVariant>,
// ) -> TokenStream {
//     let span = ty.span();

//     let mut serialize_impl = TokenStream::new();
//     let mut deserialize_impl = TokenStream::new();
//     let mut size_hint_impl = TokenStream::new();
//     let mut tags = TokenStream::new();

//     for variant in variants.into_iter() {
//         let var_ident = variant.ident;
//         let span = var_ident.span();
//         let tag = variant.tag;

//         tags.extend(quote_spanned! {span=>
//             #tag,
//         });

//         serialize_impl.extend(quote_spanned! {span=>
//             #ty::#var_ident(v) => {
//                 let wire_type = v.into_wire();
//                 wire_type.serialize(#tag, writer);
//             }
//         });

//         deserialize_impl.extend(quote_spanned! { span=>
//             if tag == #tag {
//                 let value = FromWire::from_wire(wire_type)?;
//                 return Ok(#ty::#var_ident(value));
//             }
//         });

//         size_hint_impl.extend(quote_spanned! {span=>
//             #ty::#var_ident(v) => {
//                 IntoWire::size_hint(v, #tag)
//             },
//         });
//     }

//     quote_spanned! {span=>
//         #[automatically_derived]
//         #[allow(unused_imports)]
//         impl #root::gin_tonic_core::Message for #ty {
//             fn serialize(self, writer: &mut impl #root::gin_tonic_core::bytes::BufMut) {
//                 use #root::IntoWire;

//                 match self {
//                     #serialize_impl
//                 }
//             }

//             fn size_hint(&self) -> usize {
//                 use #root::IntoWire;
//                 use #root::export::VarInt;

//                 match self {
//                     #size_hint_impl
//                 }
//             }

//             fn deserialize_tags<'a>(tags: impl Iterator<Item = #root::gin_tonic_core::Tag<'a>>) -> Result<Self, #root::Error> {
//                 let mut slf = None;

//                 for tag in tags {
//                     let (field_number, wire_type) = tag.into_parts();
//                     slf = Some(<Self as #root::gin_tonic_core::OneOf>::deserialize_wire(field_number, wire_type)?);
//                 }

//                 slf.ok_or(#root::Error::InvalidOneOf)
//             }
//         }

//         #[automatically_derived]
//         #[allow(unused_imports)]
//         impl #root::gin_tonic_core::OneOf for #ty {
//             fn matches_tag(tag: u32) -> bool {
//                 [#tags].contains(&tag)
//             }

//             fn deserialize_wire(tag: u32, wire_type: #root::gin_tonic_core::WireTypeView) -> Result<Self, #root::Error> {
//                 use #root::FromWire;

//                 #deserialize_impl

//                 Err(#root::Error::InvalidOneOf)
//             }
//         }
//     }
// }

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
                encoder.encode_uint64(#tag)
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

        // #[automatically_derived]
        // #[allow(unused_imports)]
        // impl #root::IntoWire for #ty {
        //     fn into_wire(self) -> #root::gin_tonic_core::WireType {
        //         match self {
        //             #into_impl
        //         }
        //     }

        //     fn size_hint(&self, tag: u32) -> usize {
        //         use #root::export::VarInt;

        //         tag.required_space() as usize
        //             + match self {
        //                 #size_hint_impl
        //             }
        //     }
        // }

        // #[automatically_derived]
        // #[allow(unused_imports)]
        // impl #root::FromWire for #ty {
        //     fn from_wire(wire: #root::gin_tonic_core::WireTypeView) -> Result<Self, #root::Error>
        //     where
        //         Self: Sized,
        //     {
        //         match u32::from_wire(wire)? {
        //             #from_impluse #root::Tag;
        //             n => Err(#root::Error::UnknownEnumVariant(n)),
        //         }
        //     }
        // }
    }
}

// pub(crate) fn one_of_enumeration(
//     root: &proc_macro2::TokenStream,
//     input: crate::ast::OneOfInput,
// ) -> TokenStream {
//     let ty = input.ident;

//     let variants = input
//         .data
//         .take_enum()
//         .expect("OneOF derive only works on newtype enums");

//     expand_unwrapped_oneof(root, ty, variants)
// }
