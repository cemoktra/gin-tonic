use protox::prost_reflect::{Cardinality, Kind, MessageDescriptor};

use crate::codegen::{case, enums, module, one_of, utils, Context};

/// generate message
pub(crate) fn generate(
    ctx: &Context,
    parent: &mut module::Module,
    module_path: &str,
    ty: MessageDescriptor,
) {
    if ty.is_map_entry() {
        return;
    }

    let qualified_name = ty.full_name();

    // external types are not generated
    if ctx.resolve_ident(qualified_name).is_some() {
        return;
    }
    if !ctx.filter(qualified_name) {
        return;
    }

    if let Some(one_of) = one_of::is_unwrappable_one_of(&ty) {
        one_of::generate_unwrapped(ctx, parent, module_path, one_of);
        return;
    }

    let attributes = ctx.attributes(qualified_name);

    let module = module::create_child(parent, module_path);

    let ty_name = case::convert(ty.name(), case::Case::Pascal);
    let name = quote::format_ident!("{}", ty_name);

    let mut body = quote::quote!();

    for field in ty.fields() {
        let tag = field.number();
        let proto3_optional = field.field_descriptor_proto().proto3_optional();

        if let Some(one_of) = one_of::fetch_one_of(&field) {
            let Some(first) = one_of.fields().next() else {
                continue;
            };

            if first.name() != field.name() {
                continue;
            }

            let field_name = case::convert(one_of.name(), case::Case::Snake);
            let field_name = quote::format_ident!("{}", field_name);

            let field_type = utils::resolve_message(ctx, qualified_name, one_of.full_name());

            let cardinality = if proto3_optional {
                quote::quote! {
                    , cardinality = "optional"
                }
            } else {
                quote::quote! {}
            };

            body.extend(quote::quote! {
                #[gin(tag = 0, kind = "one_of" #cardinality )]
                pub #field_name: #field_type,
            });

            continue;
        }

        let cardinality = match field.cardinality() {
            Cardinality::Repeated => {
                if field.is_map() {
                    quote::quote! {}
                } else {
                    quote::quote! {
                        , cardinality = "repeated"
                    }
                }
            }
            _ => {
                if proto3_optional {
                    quote::quote! {
                        , cardinality = "optional"
                    }
                } else {
                    quote::quote! {}
                }
            }
        };

        let kind = if let Kind::Message(_) = field.kind() {
            quote::quote! {
                , kind = "message"
            }
        } else if field.is_map() {
            quote::quote! {
                , kind = "map"
            }
        } else {
            quote::quote! {}
        };

        let field_name = quote::format_ident!("{}", field.name());
        let field_type = utils::field_type(ctx, qualified_name, &field);

        body.extend(quote::quote! {
            #[gin(tag = #tag #cardinality #kind)]
            pub #field_name: #field_type,
        });
    }

    module.extend(quote::quote! {
        #[derive(Clone, Debug, Message)]
        #attributes
        pub struct #name {
            #body
        }
    });

    {
        let module_path = ty.name();

        for child in ty.child_enums() {
            enums::generate(ctx, module, module_path, child);
        }
        for child in ty.child_messages() {
            generate(ctx, module, module_path, child);
        }
        for one_of in ty.oneofs() {
            one_of::generate(ctx, module, module_path, one_of);
        }
    }
}
