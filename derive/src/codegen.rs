use darling::ast::Fields;
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Ident;

use crate::ast::{
    IsMap, IsOption, IsPackable, IsRepeated, MessageDeriveData, MessageField, ScalarToken,
};

pub(crate) fn expand_message(
    root: &proc_macro2::TokenStream,
    input: crate::ast::MessageInput,
) -> TokenStream {
    let ty = input.ident;

    match input.data {
        MessageDeriveData::Enum(_) => quote! { compile_error!("enum_tuple not supported") },
        MessageDeriveData::Struct(fields) => expand_struct_message(root, ty, fields),
    }
}
fn expand_struct_message(
    root: &proc_macro2::TokenStream,
    ty: Ident,
    fields: Fields<MessageField>,
) -> TokenStream {
    let span = ty.span();

    let mut encode_impl = TokenStream::new();
    let mut decode_impl = TokenStream::new();

    for field in fields {
        let id = field.id;
        let ty = field.ty;
        let field_ident = field
            .ident
            .clone()
            .expect("named struct fields have idents");
        let span = field_ident.span();

        if field.oneof.is_present() {
            encode_impl.extend(quote_spanned! { span=>
                self.#field_ident.encode_message(encoder);
            });
            decode_impl.extend(quote_spanned! { span=>
                #field_ident: #ty::decode_raw_message(raw_message)?,
            });
        } else if let Some(inner) = ty.is_option() {
            let scalar_ty = match field.scalar {
                Some(scalar) => scalar.scalar_token(root),
                None => inner.scalar_token(root),
            };

            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = &self.#field_ident {
                    <#inner as Scalar::<#scalar_ty>>::encode_field(value, #id, encoder);
                }
            });
            decode_impl.extend(quote_spanned! { span=>
                #field_ident: match <#inner as Scalar::<#scalar_ty>>::decode_field(#id, &raw_message) {
                    Ok(#field_ident) => Some(#field_ident),
                    Err(#root::ProtoError::MissingField(_)) => None,
                    Err(err) => return Err(err),
                },
            });
        } else if let Some(inner) = ty.is_repeated() {
            let (scalar_ty, packed) = match field.scalar {
                Some(scalar) => {
                    let is_packable = scalar.is_packable();
                    let packed = field
                        .packed
                        .map(|lit_bool| lit_bool.value)
                        .unwrap_or(is_packable);
                    let scalar_ty = scalar.scalar_token(root);
                    (scalar_ty, packed)
                }
                None => {
                    let is_packable = inner.is_packable();
                    let packed = field
                        .packed
                        .map(|lit_bool| lit_bool.value)
                        .unwrap_or(is_packable);
                    let scalar_ty = inner.scalar_token(root);
                    (scalar_ty, packed)
                }
            };

            if packed {
                encode_impl.extend(quote_spanned! { span=>
                    <Vec<#inner> as #root::Packed::<#scalar_ty>>::encode(&self.#field_ident, #id, encoder);
                });
                decode_impl.extend(quote_spanned! { span=>
                    #field_ident: <Vec<#inner> as #root::Packed<#scalar_ty>>::decode(#id, &raw_message)?,
                });
            } else {
                encode_impl.extend(quote_spanned! { span=>
                    <Vec<#inner> as #root::Unpacked::<#scalar_ty>>::encode(
                        &self.#field_ident,
                        #root::Tag::from_parts(#id, #root::WIRE_TYPE_LENGTH_ENCODED),
                        encoder,
                    );
                });
                decode_impl.extend(quote_spanned! { span=>
                    #field_ident: <Vec<#inner> as #root::Unpacked<#scalar_ty>>::decode(
                        #root::Tag::from_parts(#id, #root::WIRE_TYPE_LENGTH_ENCODED),
                        &raw_message,
                    )?,
                });
            }
        } else if let Some((key_ty, value_ty)) = ty.is_map() {
            let key_scalar_ty = match field.key_scalar {
                Some(scalar) => scalar.scalar_token(root),
                None => key_ty.scalar_token(root),
            };
            let value_scalar_ty = match field.value_scalar {
                Some(scalar) => scalar.scalar_token(root),
                None => value_ty.scalar_token(root),
            };

            encode_impl.extend(quote_spanned! { span=>
                #root::Map::<#key_scalar_ty, #value_scalar_ty>::encode(&self.#field_ident, #id, encoder);
            });
            decode_impl.extend(quote_spanned! { span=>
                #field_ident: #root::Map::<#key_scalar_ty, #value_scalar_ty>::decode(#id, &raw_message)?,
            });
        } else {
            let scalar_ty = match field.scalar {
                Some(scalar) => scalar.scalar_token(root),
                None => ty.scalar_token(root),
            };

            encode_impl.extend(quote_spanned! { span=>
                <#ty as Scalar::<#scalar_ty>>::encode_field(&self.#field_ident, #id, encoder);
            });
            decode_impl.extend(quote_spanned! { span=>
                #field_ident: <#ty as Scalar::<#scalar_ty>>::decode_field(#id, &raw_message)?,
            });
        }
    }

    quote_spanned! {span=>
        #[automatically_derived]
        #[allow(unused_imports)]
        impl #root::Message for #ty {
            fn encode_message(&self, encoder: &mut impl #root::Encode) {
                use #root::Scalar;

                #encode_impl
            }

            fn decode_raw_message<'buf>(
                raw_message: #root::RawMessageView<'buf>,
            ) -> Result<Self, #root::ProtoError>
            where
                Self: Sized
            {
                use #root::Scalar;

                Ok(Self {
                    #decode_impl
                })
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
        let id = match variant.id.base10_parse::<u32>() {
            Ok(id) => id,
            Err(_) => return quote_spanned! {span=> compile_error!("field number (id) is no u32")},
        };

        encode_impl.extend(quote_spanned! {span=>
            Self::#var_ident => #id,
        });

        decode_impl.extend(quote_spanned! {span=>
            #id => Ok(Self::#var_ident),
        });
    }

    quote_spanned! {span=>
        #[automatically_derived]
        #[allow(unused_imports)]
        impl #root::PackableMarker<#root::scalars::UInt32> for #ty {}

        #[automatically_derived]
        #[allow(unused_imports)]
        impl #root::Scalar<#root::scalars::UInt32> for #ty {
            const WIRE_TYPE: u8 = #root::WIRE_TYPE_VARINT;

            fn encode(&self, encoder: &mut impl #root::Encode) {
                let value = match self {
                    #encode_impl
                };

                <u32 as #root::Scalar::<#root::scalars::UInt32>>::encode(&value, encoder)
            }

            fn decode(decoder: &mut impl #root::Decode) -> Result<Self, #root::ProtoError>
            where
                Self: Sized
            {
                let value = <u32 as #root::Scalar<#root::scalars::UInt32>>::decode(decoder)?;

                match value {
                    #decode_impl
                    n => Err(#root::ProtoError::UnknownEnumVariant(n)),
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
        .expect("OneOf derive only works on newtype enums");

    let span = ty.span();

    let mut encode_impl = TokenStream::new();
    let mut decode_impl = TokenStream::new();
    let mut ids = TokenStream::new();

    for variant in variants.into_iter() {
        let Some(field_ty) = variant.fields.fields.first() else {
            panic!("OneOfs must contain fields for now");
        };
        let var_ident = variant.ident;
        let span = var_ident.span();
        let id = variant.id;

        let scalar_ty = match variant.scalar {
            Some(scalar) => scalar.scalar_token(root),
            None => field_ty.scalar_token(root),
        };

        ids.extend(quote_spanned! {span=>
            #id,
        });

        encode_impl.extend(quote_spanned! {span=>
            Self::#var_ident(value) => #root::Scalar::<#scalar_ty>::encode_field(value, #id, encoder),
        });

        decode_impl.extend(quote_spanned! {span=>
            if let Some(bytes) = raw_message
                .tag_data(#root::Tag::from_parts(
                    #id,
                    <#field_ty as #root::Scalar<#scalar_ty>>::WIRE_TYPE,
                ))
                .rev()
                .next()
            {
                slf = Some(Self::#var_ident(<#field_ty as #root::Scalar<#scalar_ty>>::decode(
                    &mut #root::decoder::Decoder::new(bytes),
                )?));
            }
        });
    }

    quote_spanned! {span=>
        #[automatically_derived]
        #[allow(unused_imports)]
        impl #root::Message for #ty {
            fn encode_message(&self, encoder: &mut impl #root::Encode) {
                match self {
                    #encode_impl
                }
            }

            fn decode_raw_message<'buf>(
                raw_message: #root::RawMessageView<'buf>,
            ) -> Result<Self, #root::ProtoError>
            where
                Self: Sized,
            {
                let mut slf = None;

                #decode_impl

                slf.ok_or(#root::ProtoError::MissingOneOf(&[#ids]))
            }
        }
    }
}
