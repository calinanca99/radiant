use radiant_protocol::radiant_server::RadiantServer;
use radiant_server::Service;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    let server = Service::new();
    Server::builder()
        .add_service(RadiantServer::new(server))
        .serve("127.0.0.1:3000".parse().unwrap())
        .await
        .unwrap()
}
