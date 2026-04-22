use darling::ast::Fields;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
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

    let builder_ident = format_ident!("{ty}Builder");
    let mut builder_fields = TokenStream::new();
    let mut builder_destructuring = TokenStream::new();
    let mut finish_field = TokenStream::new();
    let mut decode_field = TokenStream::new();

    let mut oneof_match = TokenStream::new();

    for field in fields {
        let id = field.id;
        let ty = field.ty;
        let field_ident = field
            .ident
            .clone()
            .expect("named struct fields have idents");
        let span = field_ident.span();

        if field.oneof.is_present() {
            builder_fields.extend(quote_spanned! { span=>
                #field_ident: Option<#ty>,
            });
            builder_destructuring.extend(quote_spanned! { span=>
                #field_ident,
            });
            finish_field.extend(quote_spanned! { span=>
                #field_ident: #field_ident.ok_or(#root::gin_tonic_core::ProtoError::MissingField(#id))?,
            });
            decode_field.extend(quote_spanned! { span=> });

            encode_impl.extend(quote_spanned! { span=>
                self.#field_ident.encode_message(encoder);
            });

            oneof_match = quote_spanned! { span=>
                if #ty::matches_tag(tag) {
                    self.#field_ident = Some(#ty::decode_field(tag, decoder)?);
                    return Ok(());
                }
            };
        } else if let Some(inner) = ty.is_option() {
            let scalar_ty = match field.scalar {
                Some(scalar) => scalar.scalar_token(root),
                None => inner.scalar_token(root),
            };

            builder_fields.extend(quote_spanned! { span=>
                #field_ident: #ty,
            });
            builder_destructuring.extend(quote_spanned! { span=>
                #field_ident,
            });
            finish_field.extend(quote_spanned! { span=>
                #field_ident,
            });
            decode_field.extend(quote_spanned! { span=>
                #id => self.#field_ident = Some(<#inner as Scalar::<#scalar_ty>>::decode(decoder)?),
            });

            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = &self.#field_ident {
                    <#inner as Scalar::<#scalar_ty>>::encode_field(value, #id, encoder);
                }
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

            builder_fields.extend(quote_spanned! { span=>
                #field_ident: #ty,
            });
            builder_destructuring.extend(quote_spanned! { span=>
                #field_ident,
            });
            finish_field.extend(quote_spanned! { span=>
                #field_ident,
            });

            if packed {
                encode_impl.extend(quote_spanned! { span=>
                    <Vec<#inner> as #root::Packed::<#scalar_ty>>::encode(&self.#field_ident, #id, encoder);
                });
                decode_field.extend(quote_spanned! { span=>
                    #id => <Vec<#inner> as #root::Packed<#scalar_ty>>::decode(decoder, &mut self.#field_ident)?,
                });
            } else {
                encode_impl.extend(quote_spanned! { span=>
                    <Vec<#inner> as #root::Unpacked::<#scalar_ty>>::encode(
                        &self.#field_ident,
                        #root::Tag::from_parts(#id, #root::WIRE_TYPE_LENGTH_ENCODED),
                        encoder,
                    );
                });
                decode_field.extend(quote_spanned! { span=>
                    #id => self.#field_ident.push(<#inner as #root::Scalar<#scalar_ty>>::decode(decoder)?),
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

            builder_fields.extend(quote_spanned! { span=>
                #field_ident: #ty,
            });
            builder_destructuring.extend(quote_spanned! { span=>
                #field_ident,
            });
            finish_field.extend(quote_spanned! { span=>
                #field_ident,
            });
            decode_field.extend(quote_spanned! { span=>
                #id => #root::Map::<#key_scalar_ty, #value_scalar_ty>::decode(decoder, &mut self.#field_ident)?,
            });

            encode_impl.extend(quote_spanned! { span=>
                #root::Map::<#key_scalar_ty, #value_scalar_ty>::encode(&self.#field_ident, #id, encoder);
            });
        } else {
            let scalar_ty = match field.scalar {
                Some(scalar) => scalar.scalar_token(root),
                None => ty.scalar_token(root),
            };

            builder_fields.extend(quote_spanned! { span=>
                #field_ident: Option<#ty>,
            });
            builder_destructuring.extend(quote_spanned! { span=>
                #field_ident,
            });
            finish_field.extend(quote_spanned! { span=>
                #field_ident: #field_ident.ok_or(#root::gin_tonic_core::ProtoError::MissingField(#id))?,
            });
            decode_field.extend(quote_spanned! { span=>
                #id => self.#field_ident = Some(Scalar::<#scalar_ty>::decode(decoder)?),
            });

            encode_impl.extend(quote_spanned! { span=>
                <#ty as Scalar::<#scalar_ty>>::encode_field(&self.#field_ident, #id, encoder);
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

            fn decode_message(decoder: &mut impl #root::Decode) -> Result<Self, #root::gin_tonic_core::ProtoError>
            where
                Self: Sized,
            {
                let mut builder = #builder_ident::default();

                while !decoder.eof() {
                    let tag = decoder.decode_tag()?;
                    builder.decode_field(tag, decoder)?;
                }

                builder.finish()
            }
        }

        #[derive(Default)]
        struct #builder_ident {
            #builder_fields
        }

        #[allow(unused_imports)]
        impl #builder_ident {
            fn finish(self) -> Result<#ty, #root::ProtoError> {
                let Self {
                    #builder_destructuring
                } = self;
                Ok(#ty {
                    #finish_field
                })
            }

            fn decode_field(
                &mut self,
                tag: #root::Tag,
                decoder: &mut impl #root::Decode,
            ) -> Result<(), #root::ProtoError> {
                use #root::{Scalar, scalars::*};

                #oneof_match

                match tag.field_number() {
                    #decode_field
                    _ => {}
                }
                Ok(())
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
            Self::#var_ident(value) => <#field_ty as #root::Scalar::<#scalar_ty>>::encode_field(value, #id, encoder),
        });

        decode_impl.extend(quote_spanned! {span=>
            #id => return Ok(Self::#var_ident(<#field_ty as Scalar<#scalar_ty>>::decode(decoder)?)),
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

            fn decode_message(
                decoder: &mut impl #root::Decode,
            ) -> Result<Self, #root::ProtoError>
            where
                Self: Sized,
            {
                let mut slf = None;

                while !decoder.eof() {
                    let tag = decoder.decode_tag()?;

                    slf = Some(Self::decode_field(tag, decoder)?);
                }

                slf.ok_or(#root::ProtoError::MissingOneOf(&[#ids]))
            }
        }

        #[allow(unused_imports)]
        impl #ty {
            fn matches_tag(tag: #root::Tag,)  -> bool {
                [#ids].contains(&tag.field_number())
            }

            fn decode_field(
                tag: #root::Tag,
                decoder: &mut impl crate::Decode,
            ) -> Result<Self, #root::ProtoError> {
                use #root::{Scalar, scalars::*};

                match tag.field_number() {
                    #decode_impl
                    _ => {}
                }

                Err(#root::ProtoError::MissingOneOf(&[#ids]))
            }
        }
    }
}
