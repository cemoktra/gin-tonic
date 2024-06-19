use crate::codegen::{case, module};
use protox::prost_reflect::ServiceDescriptor;
use tonic_build::CodeGenBuilder;

pub(crate) fn generate(parent: &mut module::Module, module_path: &str, svc: ServiceDescriptor) {
    let mut service = tonic_build::manual::Service::builder()
        .name(svc.name())
        .package(svc.package_name());

    let module = module::create_child(parent, module_path);

    for method in svc.methods() {
        let route_name = case::convert(method.name(), case::Case::Snake);

        let method = tonic_build::manual::Method::builder()
            .name(route_name)
            .route_name(method.name())
            .input_type(format!("super::{}", method.input().name()))
            .output_type(format!("super::{}", method.output().name()))
            .codec_path("::gin_tonic_core::codec::GinCodec")
            .build();

        service = service.method(method);
    }

    let service = service.build();

    let mut codegen = CodeGenBuilder::new();
    codegen.build_transport(false);

    let server_ts = codegen.generate_server(&service, "");
    let client_ts = codegen.generate_client(&service, "");

    module.extend(server_ts);
    module.extend(client_ts);
}
