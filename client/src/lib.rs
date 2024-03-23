use anyhow::{bail, Context, Result};
use protocol::{radiant_client::RadiantClient, GetRequest, PingRequest, PingResponse, SetRequest};
use serde::{de::DeserializeOwned, Serialize};

pub trait FromBytes {
    fn from_bytes(b: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

impl<T: DeserializeOwned> FromBytes for T {
    fn from_bytes(b: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(bincode::deserialize(b)?)
    }
}

pub trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>>;
}

impl<T: Serialize> ToBytes for T {
    fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(bincode::serialize(self)?)
    }
}

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

    pub async fn get<T: FromBytes>(&mut self, key: impl Into<String>) -> Result<Option<T>> {
        let request = tonic::Request::new(GetRequest { key: key.into() });
        let response = self.inner.get(request).await?.into_inner();

        // It's not 100% clear when this will be None. According to prost docs
        // "`oneof` fields are always wrapped in an `Option`".
        // https://github.com/tokio-rs/prost?tab=readme-ov-file#oneof-fields
        match response.result.unwrap() {
            protocol::get_response::Result::MaybeData(maybe_data) => match maybe_data.data {
                Some(d) => {
                    let t: T = T::from_bytes(d.data.as_ref())?;
                    Ok(Some(t))
                }
                None => Ok(None),
            },
            protocol::get_response::Result::Error(e) => {
                bail!("Failed to get value: {}", e.reason)
            }
        }
    }

    pub async fn set<T: ToBytes>(&mut self, key: impl Into<String>, value: T) -> Result<()> {
        let data = value.to_bytes()?;
        let request = tonic::Request::new(SetRequest {
            key: key.into(),
            data,
        });
        let response = self.inner.set(request).await?.into_inner();

        if let Some(e) = response.error {
            bail!("Failed to set value: {}", e.reason)
        } else {
            Ok(())
        }
    }
}
