use anyhow::Result;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

#[async_trait]
pub(crate) trait OAuthRequest<T: DeserializeOwned = SerdeMap> {
    async fn request_data(endpoint: &'static str, token: &str) -> Result<T>;
}

type SerdeMap = Map<String, Value>;
