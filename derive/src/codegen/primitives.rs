use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use syn::{Ident, LitInt, Type};

use crate::ast::{MessageField, Primitive};

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
    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = None;
    });
    decode_set.extend(quote_spanned! { span=>
        #field_ident: #field_ident.ok_or(#root::DecodeError::MissingField(#tag))?,
    });

    match protobuf_type {
        Some(Primitive::Int32) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, #root::types::Int32, self.#field_ident, encoder, Encode::encode_int32);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::Int32, #field_ident, wire_type, decoder, Decode::decode_int32);
                },
            });
        }
        Some(Primitive::Int64) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, #root::types::Int64, self.#field_ident, encoder, Encode::encode_int64);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::Int64, #field_ident, wire_type, decoder, Decode::decode_int64);
                },
            });
        }
        Some(Primitive::Uint32) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, #root::types::UInt32, self.#field_ident, encoder, Encode::encode_uint32);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::UInt32, #field_ident, wire_type, decoder, Decode::decode_uint32);
                },
            });
        }
        Some(Primitive::Uint64) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, #root::types::UInt64, self.#field_ident, encoder, Encode::encode_uint64);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::UInt64, #field_ident, wire_type, decoder, Decode::decode_uint64);
                },
            });
        }
        Some(Primitive::Sint32) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, #root::types::SInt32, self.#field_ident, encoder, Encode::encode_sint32);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::SInt32, #field_ident, wire_type, decoder, Decode::decode_sint32)
                },
            });
        }
        Some(Primitive::Sint64) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, #root::types::SInt64, self.#field_ident, encoder, Encode::encode_sint64);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::SInt64, #field_ident, wire_type, decoder, Decode::decode_sint64);
                },
            });
        }
        Some(Primitive::Fixed32) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, #root::types::Fixed32, self.#field_ident, encoder, Encode::encode_fixed32);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::Fixed32, #field_ident, wire_type, decoder, Decode::decode_fixed32);
                },
            });
        }
        Some(Primitive::Fixed64) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, #root::types::Fixed64, self.#field_ident, encoder, Encode::encode_fixed64);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::Fixed64, #field_ident, wire_type, decoder, Decode::decode_fixed64);
                },
            });
        }
        Some(Primitive::Sfixed32) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, #root::types::SFixed32, self.#field_ident, encoder, Encode::encode_sfixed32);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::SFixed32, #field_ident, wire_type, decoder, Decode::decode_sfixed32);
                },
            });
        }
        Some(Primitive::Sfixed64) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, #root::types::SFixed64, self.#field_ident, encoder, Encode::encode_sfixed64);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::SFixed64, #field_ident, wire_type, decoder, Decode::decode_sfixed64);
                },
            });
        }
        Some(Primitive::String) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, String, &self.#field_ident, encoder, Encode::encode_str);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(String, #field_ident, wire_type, decoder, Decode::decode_string);
                },
            });
        }
        Some(Primitive::Bool) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, bool, self.#field_ident, encoder, Encode::encode_bool);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(bool, #field_ident, wire_type, decoder, Decode::decode_bool);
                },
            });
        }
        None => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_field!(#tag, #ty, &self.#field_ident, encoder, Encode::encode_type);

            });
            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#ty, #field_ident, wire_type, decoder, Decode::decode_type);
                },
            });
        }
    }
}

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
    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = None;
    });
    decode_set.extend(quote_spanned! { span=>
        #field_ident,
    });

    match protobuf_type {
        Some(Primitive::Int32) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, #root::types::Int32, value, encoder, Encode::encode_int32);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::Int32, #field_ident, wire_type, decoder, Decode::decode_int32)
                },
            });
        }
        Some(Primitive::Int64) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, #root::types::Int64, value, encoder, Encode::encode_int64);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::Int64, #field_ident, wire_type, decoder, Decode::decode_int64)
                },
            });
        }
        Some(Primitive::Uint32) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, #root::types::UInt32, value, encoder, Encode::encode_uint32);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::UInt32, #field_ident, wire_type, decoder, Decode::decode_uint32)
                },
            });
        }
        Some(Primitive::Uint64) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, #root::types::UInt64, value, encoder, Encode::encode_uint64);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::UInt64, #field_ident, wire_type, decoder, Decode::decode_uint64)
                },
            });
        }
        Some(Primitive::Sint32) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, #root::types::SInt32, value, encoder, Encode::encode_sint32);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::SInt32, #field_ident, wire_type, decoder, Decode::decode_sint32)
                },
            });
        }
        Some(Primitive::Sint64) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, #root::types::SInt64, value, encoder, Encode::encode_sint64);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::SInt64, #field_ident, wire_type, decoder, Decode::decode_sint64)
                },
            });
        }
        Some(Primitive::Fixed32) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, #root::types::Fixed32, value, encoder, Encode::encode_fixed32);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::Fixed32, #field_ident, wire_type, decoder, Decode::decode_fixed32)
                },
            });
        }
        Some(Primitive::Fixed64) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, #root::types::Fixed64, value, encoder, Encode::encode_fixed64);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::Fixed64, #field_ident, wire_type, decoder, Decode::decode_fixed64)
                },
            });
        }
        Some(Primitive::Sfixed32) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, #root::types::SFixed32, value, encoder, Encode::encode_sfixed32);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::SFixed32, #field_ident, wire_type, decoder, Decode::decode_sfixed32)
                },
            });
        }
        Some(Primitive::Sfixed64) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, #root::types::SFixed64, value, encoder, Encode::encode_sfixed64);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#root::types::SFixed64, #field_ident, wire_type, decoder, Decode::decode_sfixed64)
                },
            });
        }
        Some(Primitive::String) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = &self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, String, value, encoder, Encode::encode_str);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(String, #field_ident, wire_type, decoder, Decode::decode_string)
                },
            });
        }
        Some(Primitive::Bool) => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, bool, value, encoder, Encode::encode_bool);
                }
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(bool, #field_ident, wire_type, decoder, Decode::decode_bool)
                },
            });
        }
        None => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_field!(#tag, #ty, &value, encoder, Encode::encode_type);
                }
            });
            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_field!(#ty, #field_ident, wire_type, decoder, Decode::decode_type)
                },
            });
        }
    }
}

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
    decode_init.extend(quote_spanned! { span=>
        let mut #field_ident = vec![];
    });
    decode_set.extend(quote_spanned! { span=>
        #field_ident,
    });

    match protobuf_type {
        Some(Primitive::Int32) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, Encode::encode_int32);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(#root::types::Int32, &mut #field_ident, wire_type, decoder, Decode::decode_int32)
                }
            });
        }
        Some(Primitive::Int64) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, Encode::encode_int64);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(#root::types::Int64, &mut #field_ident, wire_type, decoder, Decode::decode_int64)
                }
            });
        }
        Some(Primitive::Uint32) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, Encode::encode_uint32);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(#root::types::UInt32, &mut #field_ident, wire_type, decoder, Decode::decode_uint32)
                }
            });
        }
        Some(Primitive::Uint64) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, Encode::encode_uint64);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(#root::types::UInt64, &mut #field_ident, wire_type, decoder, Decode::decode_uint64)
                }
            });
        }
        Some(Primitive::Sint32) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, Encode::encode_sint32);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(#root::types::SInt32, &mut #field_ident, wire_type, decoder, Decode::decode_sint32)
                }
            });
        }
        Some(Primitive::Sint64) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, Encode::encode_sint64);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(#root::types::SInt64, &mut #field_ident, wire_type, decoder, Decode::decode_sint64)
                }
            });
        }
        Some(Primitive::Fixed32) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, Encode::encode_fixed32);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(#root::types::Fixed32, &mut #field_ident, wire_type, decoder, Decode::decode_fixed32)
                }
            });
        }
        Some(Primitive::Fixed64) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, Encode::encode_fixed64);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(#root::types::Fixed64, &mut #field_ident, wire_type, decoder, Decode::decode_fixed64)
                }
            });
        }
        Some(Primitive::Sfixed32) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, Encode::encode_sfixed32);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(#root::types::SFixed32, &mut #field_ident, wire_type, decoder, Decode::decode_sfixed32)
                }
            });
        }
        Some(Primitive::Sfixed64) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, Encode::encode_sfixed64);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(#root::types::SFixed64, &mut #field_ident, wire_type, decoder, Decode::decode_sfixed64)
                }
            });
        }
        Some(Primitive::String) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_unpacked!(#tag, String, &self.#field_ident, encoder, Encode::encode_string);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(String, &mut #field_ident, wire_type, decoder, Decode::decode_string)
                }
            });
        }
        Some(Primitive::Bool) => {
            encode_impl.extend(quote_spanned! { span=>
                #root::gin_tonic_core::encode_vector_packed!(#tag, &self.#field_ident, encoder, Encode::encode_bool);
            });

            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(bool, &mut #field_ident, wire_type, decoder, Decode::decode_bool)
                }
            });
        }
        None => {
            encode_impl.extend(quote_spanned! { span=>
                if let Some(value) = self.#field_ident {
                    #root::gin_tonic_core::encode_vector_unpacked!(#tag, #ty, &value, encoder, Encode::encode_type);
                }
            });
            decode_impl.extend(quote_spanned! { span=>
                #tag => {
                    #root::gin_tonic_core::decode_vector!(#ty, #field_ident, wire_type, decoder, Decode::decode_type)
                },
            });
        }
    }
}
