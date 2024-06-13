use crate::ast::{Cardinality, Kind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};

pub(crate) fn expand_message(input: crate::ast::MessageInput) -> TokenStream {
    let ty = input.ident;
    let span = ty.span();

    let fields = input
        .data
        .take_struct()
        .expect("Message derive only works on named structs");

    let mut serialize_impl = TokenStream::new();
    let mut deserialize_impl = TokenStream::new();
    let mut deserialize_set = TokenStream::new();
    let mut size_hint_impl = TokenStream::new();
    let mut size_hint_sum = quote! { 0 };

    for field in fields {
        let tag = field.tag;
        let ty = field.ty;
        let field_ident = field
            .ident
            .clone()
            .expect("named struct fields have idents");
        let span = field_ident.span();
        let field_size_ident = format_ident!("{}_size", field_ident);

        size_hint_sum.extend(quote_spanned! { span=>
            + #field_size_ident
        });

        deserialize_set.extend(quote_spanned! { span=>
            #field_ident,
        });

        match field.cardinality {
            Cardinality::Required => match field.kind {
                Kind::Primitive => {
                    serialize_impl.extend(quote_spanned! { span=>
                        let wire_type = self.#field_ident.into_wire();
                        written += wire_type.serialize(#tag, writer)?;
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        let wire_type = tag_map
                            .remove(&#tag)
                            .ok_or(gin_tonic_core::protobuf::Error::MissingField(#tag))?
                            .into_iter()
                            .nth(0)
                            .ok_or(gin_tonic_core::protobuf::Error::MissingField(#tag))?;
                        let #field_ident = #ty::from_wire(wire_type)?;
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident = self.#field_ident.size_hint(#tag);
                    });
                }
                Kind::Message => {
                    serialize_impl.extend(quote_spanned! { span=>
                        let wire_type = self.#field_ident.into_wire();
                        written += wire_type.serialize(#tag, writer)?;
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        let wire_type = tag_map
                            .remove(&#tag)
                            .ok_or(gin_tonic_core::protobuf::Error::MissingField(#tag))?
                            .into_iter()
                            .nth(0)
                            .ok_or(gin_tonic_core::protobuf::Error::MissingField(#tag))?;
                        let #field_ident = #ty::from_wire(wire_type)?;
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident = ::gin_tonic_core::protobuf::nested::size_hint(#tag, &self.#field_ident);
                    });
                }
                Kind::OneOf => {}
                Kind::Map => {}
            },
            Cardinality::Optional => match field.kind {
                Kind::Primitive => {
                    serialize_impl.extend(quote_spanned! { span=>
                        if let Some(value) = self.#field_ident {
                            let wire_type = value.into_wire();
                            written += wire_type.serialize(#tag, writer)?;
                        }
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        let #field_ident = tag_map
                            .remove(&#tag)
                            .map(|wire| gin_tonic_core::protobuf::FromWire::from_wire(
                                wire
                                    .into_iter()
                                    .nth(0)
                                    .ok_or(gin_tonic_core::protobuf::Error::MissingField(#tag))?
                                )
                            )
                            .transpose()?;
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident = self.#field_ident.map(|value| value.size_hint(#tag)).unwrap_or_default();
                    });
                }
                Kind::Message => {}
                Kind::OneOf => {}
                Kind::Map => {}
            },
            Cardinality::Repeated => match field.kind {
                Kind::Primitive => {
                    serialize_impl.extend(quote_spanned! { span=>
                        for item in self.#field_ident {
                            let wire_type = item.into_wire();
                            written += wire_type.serialize(#tag, writer)?;
                        }
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        let mut #field_ident = vec![];
                        if let Some(wire_types) = tag_map.remove(&#tag) {
                            for wire_type in wire_types {
                                protocols.push(gin_tonic_core::protobuf::FromWire::from_wire(wire_type)?)
                            }
                        }
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident: usize = self.#field_ident.iter().map(|item| item.size_hint(#tag)).sum();
                    });
                }
                Kind::Message => {}
                Kind::OneOf => {}
                Kind::Map => {}
            },
        }
    }

    quote_spanned! {span=>
        #[automatically_derived]
        impl ::gin_tonic_core::protobuf::Message for #ty {
            fn serialize(self, writer: &mut impl std::io::Write) -> Result<usize, ::gin_tonic_core::protobuf::Error> {
                use ::gin_tonic_core::protobuf::IntoWire;

                let mut written = 0;

                #serialize_impl

                Ok(written)
            }

            fn deserialize_tags(tag_map: &mut std::collections::HashMap<u32, Vec<::gin_tonic_core::protobuf::WireTypeView>>) -> Result<Self, ::gin_tonic_core::protobuf::Error> {
                use ::gin_tonic_core::protobuf::FromWire;

                #deserialize_impl

                Ok(Self {
                    #deserialize_set
                })
            }

            fn size_hint(&self) -> usize {
                use ::gin_tonic_core::protobuf::IntoWire;
                use ::gin_tonic_core::export::VarInt;

                #size_hint_impl

                #size_hint_sum
            }
        }
    }
}

pub(crate) fn expand_enumeration(input: crate::ast::EnumerationInput) -> TokenStream {
    let ty = input.ident;
    let span = ty.span();

    let variants = input
        .data
        .take_enum()
        .expect("Enumeration derive only works on unit enumerations");

    let mut into_impl = TokenStream::new();
    let mut size_hint_impl = TokenStream::new();
    let mut from_impl = TokenStream::new();

    for variant in variants {
        let var_ident = variant.ident;
        let span = var_ident.span();
        let tag = variant.tag;

        into_impl.extend(quote_spanned! {span=>
            #ty::#var_ident => {
                let tag: u32 = #tag;
                tag.into_wire()
            },
        });

        size_hint_impl.extend(quote_spanned! {span=>
            #ty::#var_ident => {
                let tag: u32 = #tag;
                tag.required_space()
            }
        });

        from_impl.extend(quote_spanned! {span=>
            #tag => Ok(#ty::#var_ident),
        });
    }

    quote_spanned! {span=>
        #[automatically_derived]
        impl ::gin_tonic_core::protobuf::IntoWire for #ty {
            fn into_wire(self) -> ::gin_tonic_core::protobuf::WireType {
                match self {
                    #into_impl
                }
            }

            fn size_hint(&self, tag: u32) -> usize {
                tag.required_space()
                    + match self {
                        #size_hint_impl
                    }
            }
        }

        #[automatically_derived]
        impl ::gin_tonic_core::protobuf::FromWire for #ty {
            fn from_wire(wire: ::gin_tonic_core::protobuf::WireTypeView) -> Result<Self, ::gin_tonic_core::protobuf::Error>
            where
                Self: Sized,
            {
                match u32::from_wire(wire)? {
                    #from_impl
                    n => Err(::gin_tonic_core::protobuf::Error::UnknownEnumVariant(n)),
                }
            }
        }
    }
}
