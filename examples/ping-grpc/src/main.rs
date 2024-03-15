use protocol::{radiant_client::RadiantClient, PingRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RadiantClient::connect("http://127.0.0.1:3000").await?;

    let request = tonic::Request::new(PingRequest {});

    let response = client.ping(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
