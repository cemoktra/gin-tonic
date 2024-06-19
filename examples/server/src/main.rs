use shared::example::echo_response::Echo;
use shared::example::example_server::{Example, ExampleServer};
use shared::example::{EchoRequest, EchoResponse};
use tonic::async_trait;

struct ServerImpl;

#[async_trait]
impl Example for ServerImpl {
    async fn echo(
        &self,
        request: tonic::Request<EchoRequest>,
    ) -> Result<tonic::Response<EchoResponse>, tonic::Status> {
        let EchoRequest { echo, request_id } = request.into_inner();

        println!("handling request id {request_id}");

        Ok(tonic::Response::new(EchoResponse::Echo(Echo {
            echo,
            ip: std::net::Ipv4Addr::LOCALHOST,
        })))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = ExampleServer::new(ServerImpl);

    tonic::transport::Server::builder()
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
