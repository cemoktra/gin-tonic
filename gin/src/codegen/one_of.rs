use protox::prost_reflect::{FieldDescriptor, MessageDescriptor, OneofDescriptor};

use crate::codegen::{case, enums, messages, module, utils, Context};

pub(crate) fn generate(
    ctx: &Context,
    parent: &mut module::Module,
    module_path: &str,
    ty: OneofDescriptor,
) {
    let fields = ty
        .fields()
        .filter(|field| !field.field_descriptor_proto().proto3_optional())
        .collect::<Vec<_>>();

    if fields.is_empty() {
        return;
    }

    let qualified_name = ty.full_name();
    if !ctx.filter(qualified_name) {
        return;
    }

    tracing::info!("generating wrapped one of: {qualified_name}");

    let attributes = ctx.attributes(qualified_name);

    let module = module::create_child(parent, module_path);

    let ty_name = case::convert(ty.name(), case::Case::Pascal);
    let ty_name = quote::format_ident!("{}", ty_name);

    let mut body = quote::quote!();

    for variant in ty.fields() {
        let tag = variant.number();

        let variant_name = case::convert(variant.name(), case::Case::Pascal);
        let variant_name = quote::format_ident!("{}", variant_name);

        let variant_type = utils::field_type(ctx, qualified_name, &variant);

        if utils::is_unit_type(&variant_type) {
            body.extend(quote::quote! {
                #variant_name,
            });
        } else {
            body.extend(quote::quote! {
                #[gin(tag = #tag)]
                #variant_name(#variant_type),
            });
        };
    }

    module.extend(quote::quote! {
        #[derive(Clone, Debug, Eq, PartialEq, OneOf)]
        #attributes
        pub enum #ty_name {
            #body
        }
    });
}

pub(crate) fn generate_unwrapped(
    ctx: &Context,
    parent: &mut module::Module,
    module_path: &str,
    ty: OneofDescriptor,
) {
    let parent_message = ty.parent_message();
    let qualified_name = parent_message.full_name();

    tracing::info!("generating unwrapped one of: {qualified_name}");

    let attributes = ctx.attributes(qualified_name);

    let module = module::create_child(parent, module_path);

    let ty_name = case::convert(parent_message.name(), case::Case::Pascal);
    let ty_name = quote::format_ident!("{}", ty_name);

    let mut body = quote::quote!();

    for variant in ty.fields() {
        let tag = variant.number();
        let variant_name = case::convert(variant.name(), case::Case::Pascal);
        let variant_name = quote::format_ident!("{}", variant_name);

        let variant_type = utils::field_type(ctx, qualified_name, &variant);

        if utils::is_unit_type(&variant_type) {
            body.extend(quote::quote! {
                #[gin(tag = #tag)]
                #variant_name,
            });
        } else {
            body.extend(quote::quote! {
                #[gin(tag = #tag)]
                #variant_name(#variant_type),
            });
        }
    }

    let item: syn::ItemEnum = syn::parse_quote! {
        #[derive(Clone, Debug, ::gin_tonic::Message)]
        #attributes
        pub enum #ty_name {
            #body
        }
    };

    module.extend(quote::quote! {
        #item
    });

    // Generate the parent's children, because we replaced the parent type.
    {
        let ty = ty.parent_message();

        let module_path = ty.name();

        for child in ty.child_enums() {
            enums::generate(ctx, module, module_path, child);
        }
        for child in ty.child_messages() {
            messages::generate(ctx, module, module_path, child);
        }
    }
}

/// an unwrappable one of is a message (struct) containing only the oneof field. in this case
/// we can skip generating the wrapper struct (therefore unwrap the oneof)
pub(crate) fn is_unwrappable_one_of(ty: &MessageDescriptor) -> Option<OneofDescriptor> {
    let field_count = ty.fields().count();

    for one_of in ty.oneofs() {
        // Optional fields are encoded as one_ofs, so we have to skip past those.
        let fields = one_of
            .fields()
            .filter(|field| !field.field_descriptor_proto().proto3_optional())
            .collect::<Vec<_>>();

        if fields.is_empty() {
            continue;
        }

        return (field_count == fields.len()).then_some(one_of);
    }

    None
}

pub(crate) fn fetch_one_of(field: &FieldDescriptor) -> Option<OneofDescriptor> {
    if !field.field_descriptor_proto().proto3_optional() {
        field.containing_oneof()
    } else {
        None
    }
}
