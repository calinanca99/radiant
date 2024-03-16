use protocol::{
    get_response, radiant_server::Radiant, Data, GetRequest, GetResponse, MaybeData, PingRequest,
    PingResponse, SetRequest, SetResponse,
};
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

use crate::Db;

#[derive(Debug)]
pub struct Service {
    db: RwLock<Db>,
}

impl Service {
    pub fn new() -> Self {
        Self {
            db: RwLock::new(Db::new()),
        }
    }
}

#[tonic::async_trait]
impl Radiant for Service {
    async fn ping(&self, _: Request<PingRequest>) -> Result<Response<PingResponse>, Status> {
        let reply = PingResponse { error: None };

        Ok(Response::new(reply))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let request = request.into_inner();
        let db = self.db.read().await;
        let data = db.get(&request.key).await;

        let data = match data {
            Some(d) => MaybeData {
                data: Some(Data {
                    key: request.key,
                    data: d.to_vec(),
                }),
            },
            None => MaybeData { data: None },
        };
        let result = get_response::Result::MaybeData(data);

        Ok(Response::new(GetResponse {
            result: Some(result),
        }))
    }

    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        let request = request.into_inner();
        let mut db = self.db.write().await;
        let _ = db.set(request.key, request.data.into()).await;

        Ok(Response::new(SetResponse { error: None }))
    }
}
