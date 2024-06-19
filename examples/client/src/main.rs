use shared::example::example_client::ExampleClient;
use shared::example::EchoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = tonic::transport::Endpoint::new("http://[::1]:50051")?
        .connect()
        .await?;
    let mut client = ExampleClient::new(conn);

    let request = tonic::Request::new(EchoRequest {
        echo: "gin-tonic ftw!!!".to_string(),
    });

    let response = client.echo(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
