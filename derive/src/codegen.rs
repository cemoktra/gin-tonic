use crate::ast::{Cardinality, Kind, MessageDeriveData, MessageField, OneOfVariant};
use darling::ast::Fields;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::Ident;

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

        match field.cardinality.unwrap_or_default() {
            Cardinality::Required => match field.kind.unwrap_or_default() {
                Kind::Primitive => {
                    serialize_impl.extend(quote_spanned! { span=>
                        let wire_type = self.#field_ident.into_wire();
                        written += wire_type.serialize(#tag, writer)?;
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        let wire_type = tag_map
                            .remove(&#tag)
                            .ok_or(#root::Error::MissingField(#tag))?
                            .into_iter()
                            .nth(0)
                            .ok_or(#root::Error::MissingField(#tag))?;
                        let #field_ident = #ty::from_wire(wire_type)?;
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident = #root::IntoWire::size_hint(&self.#field_ident, #tag);
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
                            .ok_or(#root::Error::MissingField(#tag))?
                            .into_iter()
                            .nth(0)
                            .ok_or(#root::Error::MissingField(#tag))?;
                        let #field_ident = #ty::from_wire(wire_type)?;
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident = #root::gin_tonic_core::nested_size_hint(#tag, &self.#field_ident);
                    });
                }
                Kind::OneOf => {
                    serialize_impl.extend(quote_spanned! { span=>
                        written += self.#field_ident.serialize(writer)?;
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        let #field_ident = #ty::deserialize_tags(tag_map)?;
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident = #root::gin_tonic_core::Message::size_hint(&self.#field_ident);
                    });
                }
                Kind::Map => {
                    serialize_impl.extend(quote_spanned! { span=>
                        for (key, value) in self.#field_ident {
                            let wire_type = #root::gin_tonic_core::map_into_wire(key, value)?;
                            written += wire_type.serialize(#tag, writer)?;
                        }
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        let mut #field_ident = HashMap::new();
                        if let Some(wire_types) = tag_map.remove(&#tag) {
                            for wire_type in wire_types {
                                let (key, value) = #root::gin_tonic_core::map_from_wire(wire_type)?;
                                #field_ident.insert(key, value);
                            }
                        }
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident: usize = self
                            .#field_ident
                            .iter()
                            .map(|(key, value)| {
                                let message_size = key.size_hint(1) + value.size_hint(2);
                                message_size + message_size.required_space() + #tag.required_space()
                            })
                            .sum();
                    });
                }
            },
            Cardinality::Optional => match field.kind.unwrap_or_default() {
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
                            .map(|wire| #root::FromWire::from_wire(
                                wire
                                    .into_iter()
                                    .nth(0)
                                    .ok_or(#root::Error::MissingField(#tag))?
                                )
                            )
                            .transpose()?;
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident = self.#field_ident.map(|value| value.size_hint(#tag)).unwrap_or_default();
                    });
                }
                Kind::Message => {
                    serialize_impl.extend(quote_spanned! { span=>
                        if let Some(value) = self.#field_ident {
                            let wire_type = value.into_wire();
                            written += wire_type.serialize(#tag, writer)?;
                        }
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        let #field_ident = tag_map
                            .remove(&#tag)
                            .map(|wire| #root::FromWire::from_wire(
                                wire
                                    .into_iter()
                                    .nth(0)
                                    .ok_or(#root::Error::MissingField(#tag))?
                                )
                            )
                            .transpose()?;
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident = self.#field_ident.as_ref().map(|value| #root::gin_tonic_core::nested_size_hint(#tag, value)).unwrap_or_default();
                    });
                }
                Kind::OneOf => {
                    serialize_impl.extend(quote_spanned! { span=>
                        if let Some(value) = self.#field_ident {
                            written += value.serialize(writer)?;
                        }
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        let #field_ident = match #root::gin_tonic_core::Message::deserialize_tags(tag_map) {
                            Ok(value) => Some(value),
                            Err(_) => None,
                        };
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident = self.#field_ident.as_ref().map(|value| #root::gin_tonic_core::Message::size_hint(value)).unwrap_or_default();
                    });
                }
                Kind::Map => {
                    serialize_impl.extend(quote_spanned! { span=>
                        if let Some(value) = self.#field_ident {
                            for (key, value) in value {
                                let wire_type = #root::gin_tonic_core::map_into_wire(key, value)?;
                                written += wire_type.serialize(#tag, writer)?;
                            }
                        }
                    });

                    deserialize_impl.extend(quote_spanned! { span=>
                        let #field_ident = if let Some(wire_types) = tag_map.remove(&#tag) {
                            let mut map = HashMap::new();
                            for wire_type in wire_types {
                                let (key, value) = #root::gin_tonic_core::map_from_wire(wire_type)?;
                                map.insert(key, value);
                            }
                            Some(map)
                        } else {
                            None
                        };
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident = if let Some(map) = self.#field_ident.as_ref() {
                            map
                                .iter()
                                .map(|(key, value)| {
                                    let message_size = key.size_hint(1) + value.size_hint(2);
                                    message_size + message_size.required_space() + #tag.required_space()
                                })
                                .sum()
                        } else {
                            0usize
                        };
                    });
                }
            },
            Cardinality::Repeated => match field.kind.unwrap_or_default() {
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
                                #field_ident.push(#root::FromWire::from_wire(wire_type)?)
                            }
                        }
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident: usize = self.#field_ident.iter().map(|item| item.size_hint(#tag)).sum();
                    });
                }
                Kind::Message => {
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
                                #field_ident.push(#root::FromWire::from_wire(wire_type)?)
                            }
                        }
                    });

                    size_hint_impl.extend(quote_spanned! { span=>
                        let #field_size_ident: usize = self.#field_ident.iter().map(|item| #root::gin_tonic_core::nested_size_hint(#tag, item)).sum();
                    });
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
        impl #root::gin_tonic_core::Message for #ty {
            fn serialize(self, writer: &mut impl std::io::Write) -> Result<usize, #root::Error> {
                use #root::IntoWire;

                let mut written = 0;

                #serialize_impl

                Ok(written)
            }

            fn deserialize_tags(tag_map: &mut std::collections::HashMap<u32, Vec<#root::gin_tonic_core::WireTypeView>>) -> Result<Self, #root::Error> {
                use #root::FromWire;

                #deserialize_impl

                Ok(Self {
                    #deserialize_set
                })
            }

            fn size_hint(&self) -> usize {
                use #root::IntoWire;
                use #root::export::VarInt;

                #size_hint_impl

                #size_hint_sum
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

    let mut serialize_impl = TokenStream::new();
    let mut deserialize_impl = TokenStream::new();
    let mut size_hint_impl = TokenStream::new();

    for variant in variants.into_iter() {
        let var_ident = variant.ident;
        let span = var_ident.span();
        let tag = variant.tag;

        serialize_impl.extend(quote_spanned! {span=>
            #ty::#var_ident(v) => {
                let wire_type = v.into_wire();
                written += wire_type.serialize(#tag, writer)?;
            }
        });

        deserialize_impl.extend(quote_spanned! {span=>
            if let Some(types) = tag_map.remove(&#tag) {
                let value = FromWire::from_wire(types.into_iter().nth(0).ok_or(#root::Error::InvalidOneOf)?)?;
                return Ok(#ty::#var_ident(value));
            }
        });

        size_hint_impl.extend(quote_spanned! {span=>
            #ty::#var_ident(v) => IntoWire::size_hint(v, #tag),
        });
    }

    quote_spanned! {span=>
        #[automatically_derived]
        #[allow(unused_imports)]
        impl #root::gin_tonic_core::Message for #ty {
            fn serialize(self, writer: &mut impl std::io::Write) -> Result<usize, #root::Error> {
                use #root::IntoWire;

                let mut written = 0;

                match self {
                    #serialize_impl
                }

                Ok(written)
            }

            fn deserialize_tags(tag_map: &mut std::collections::HashMap<u32, Vec<#root::gin_tonic_core::WireTypeView>>) -> Result<Self, #root::Error> {
                use #root::FromWire;

                #deserialize_impl

                Err(#root::Error::InvalidOneOf)
            }

            fn size_hint(&self) -> usize {
                use #root::IntoWire;
                use #root::export::VarInt;

                match self {
                    #size_hint_impl
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
        #[allow(unused_imports)]
        impl #root::IntoWire for #ty {
            fn into_wire(self) -> #root::gin_tonic_core::WireType {
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
        #[allow(unused_imports)]
        impl #root::FromWire for #ty {
            fn from_wire(wire: #root::gin_tonic_core::WireTypeView) -> Result<Self, #root::Error>
            where
                Self: Sized,
            {
                match u32::from_wire(wire)? {
                    #from_impl
                    n => Err(#root::Error::UnknownEnumVariant(n)),
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

    expand_unwrapped_oneof(root, ty, variants)
}
