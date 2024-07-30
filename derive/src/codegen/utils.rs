pub(crate) fn inner_type(ty: &syn::Type) -> syn::Type {
    match ty {
        syn::Type::Path(path) => {
            let Some(segment) = path.path.segments.first() else {
                panic!("optional/repeated must be Option<T>");
            };
            match &segment.arguments {
                syn::PathArguments::AngleBracketed(arguments) => {
                    let Some(argument) = arguments.args.first() else {
                        panic!("optional/repeated must be Option<T>");
                    };

                    match argument {
                        syn::GenericArgument::Type(ty) => ty.clone(),
                        _ => panic!("optional/repeated must be Option<T>"),
                    }
                }
                _ => panic!("optional/repeated must be Option<T>"),
            }
        }
        _ => panic!("optional/repeated must be Option<T>"),
    }
}
