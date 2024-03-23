use anyhow::{Context, Result};
use protocol::{radiant_client::RadiantClient, PingRequest, PingResponse};

pub struct Client {
    inner: RadiantClient<tonic::transport::Channel>,
}

impl Client {
    pub async fn new(addr: &'static str) -> Result<Self> {
        let inner = RadiantClient::connect(addr)
            .await
            .context("Establishing connection")?;

        Ok(Self { inner })
    }

    pub async fn ping(&mut self) -> Result<PingResponse> {
        let request = tonic::Request::new(PingRequest {});
        let response = self.inner.ping(request).await?.into_inner();
        Ok(response)
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
