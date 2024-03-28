use protocol::{
    get_response, radiant_server::Radiant, Data, DelRequest, DelResponse, Error, GetRequest,
    GetResponse, MaybeData, PingRequest, PingResponse, SetRequest, SetResponse,
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

impl Default for Service {
    fn default() -> Self {
        Self::new()
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
        let data = db.get(&request.key);

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
        db.set(request.key, request.data.into());

        Ok(Response::new(SetResponse { error: None }))
    }

    async fn del(&self, request: Request<DelRequest>) -> Result<Response<DelResponse>, Status> {
        let request = request.into_inner();
        let mut db = self.db.write().await;

        if db.del(request.key).is_some() {
            Ok(Response::new(DelResponse { error: None }))
        } else {
            Ok(Response::new(DelResponse {
                error: Some(Error {
                    reason: "Key does not exist".to_string(),
                }),
            }))
        }
    }
}
