use protocol::{
    radiant_server::{Radiant, RadiantServer},
    GetRequest, GetResponse, PingRequest, PingResponse, SetRequest, SetResponse,
};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct RS {}

#[tonic::async_trait]
impl Radiant for RS {
    async fn ping(&self, _: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        let reply = PingResponse { error: None };

        Ok(Response::new(reply))
    }

    async fn get(&self, _request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        todo!()
    }

    async fn set(&self, _request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let server = RS::default();
    Server::builder()
        .add_service(RadiantServer::new(server))
        .serve("127.0.0.1:3000".parse().unwrap())
        .await
        .unwrap()
}
