use protox::prost_reflect::EnumDescriptor;

use crate::codegen::{case, module, Context};

/// generate code for enumerations
pub(crate) fn generate(
    ctx: &Context,
    parent: &mut module::Module,
    module_path: &str,
    ty: EnumDescriptor,
) {
    let qualified_name = ty.full_name();

    // external types are not generated
    if ctx.resolve_ident(qualified_name).is_some() {
        return;
    }
    if !ctx.filter(qualified_name) {
        return;
    }

    tracing::info!("generating enumeration: {qualified_name}");

    let attributes = ctx.attributes(qualified_name);

    let module = module::create_child(parent, module_path);

    let ty_name = case::convert(ty.name(), case::Case::Pascal);
    let name = quote::format_ident!("{}", ty_name);

    let mut body = quote::quote!();

    for value in ty.values() {
        let tag = value.number();
        let (_package, value) = value
            .full_name()
            .rsplit_once('.')
            .expect("Enum values should be in a package");
        if value.ends_with("UNSPECIFIED") {
            // TODO: add support for UNSPECIFIED variant
            continue;
        }
        let value = case::convert(value, case::Case::Pascal);
        let value = value
            .strip_prefix(ty_name.as_ref())
            .unwrap_or(value.as_ref());

        let value_name = quote::format_ident!("{}", value);

        body.extend(quote::quote! {
            #[gin(tag = #tag)]
            #value_name,
        });
    }

    let item: syn::ItemEnum = syn::parse_quote! {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Enumeration)]
        #attributes
        pub enum #name {
            #body
        }
    };

    module.extend(quote::quote! {
        #item
    });
}
