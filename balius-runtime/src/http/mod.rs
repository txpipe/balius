use std::str::FromStr;

use crate::wit::balius::app::http as wit;
use async_trait::async_trait;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};
pub use wit::{Body, Header, HttpError};

#[derive(Clone)]
pub enum Http {
    Mock,
    Reqwest(reqwest::Client),
}

impl Http {
    async fn request(
        &mut self,
        method: Method,
        url: String,
        headers: Vec<Header>,
        body: Option<Body>,
    ) -> Result<Body, HttpError> {
        match self {
            Http::Mock => Ok(vec![]),
            Http::Reqwest(client) => {
                let mut header_map = HeaderMap::new();
                for (key, value) in headers {
                    let key = HeaderName::from_str(key.as_str()).map_err(|_| 1u32)?;
                    let value = HeaderValue::from_str(value.as_str()).map_err(|_| 1u32)?;
                    header_map.insert(key, value);
                }
                let mut req = client.request(method, url).headers(header_map);
                if let Some(body) = body {
                    req = req.body(body);
                }
                let req = req.build().map_err(|_| 1u32)?;
                let resp = client
                    .execute(req)
                    .await
                    .map_err(|e| e.status().map(|s| s.as_u16() as u32).unwrap_or(1u32))?;
                let body = resp.bytes().await.map_err(|_| 1u32)?.into();

                Ok(body)
            }
        }
    }
}

#[async_trait]
impl wit::Host for Http {
    async fn get(&mut self, url: String, headers: Vec<Header>) -> Result<Body, HttpError> {
        self.request(Method::GET, url, headers, None).await
    }

    async fn post(
        &mut self,
        url: String,
        headers: Vec<Header>,
        body: Body,
    ) -> Result<Body, HttpError> {
        self.request(Method::POST, url, headers, Some(body)).await
    }
    async fn patch(
        &mut self,
        url: String,
        headers: Vec<Header>,
        body: Body,
    ) -> Result<Body, HttpError> {
        self.request(Method::PATCH, url, headers, Some(body)).await
    }
    async fn delete(
        &mut self,
        url: String,
        headers: Vec<Header>,
        body: Body,
    ) -> Result<Body, HttpError> {
        self.request(Method::DELETE, url, headers, Some(body)).await
    }
}
