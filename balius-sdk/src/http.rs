use std::time::Duration;

use crate::wit::balius::app::http as wit;
use serde::{Deserialize, Serialize};
use url::Url;
pub use wit::ErrorCode as HttpError;

#[derive(Clone)]
pub struct HttpRequest {
    pub method: wit::Method,
    pub url: Url,
    pub headers: Vec<(wit::FieldName, wit::FieldValue)>,
    pub body: Option<Vec<u8>>,
    pub timeout: Option<Duration>,
}

macro_rules! helper_method {
    ($name:ident, $method:expr) => {
        pub fn $name(url: Url) -> Self {
            Self::new($method, url)
        }
    };
}

impl HttpRequest {
    pub fn new(method: wit::Method, url: Url) -> Self {
        Self {
            method,
            url,
            headers: vec![],
            body: None,
            timeout: None,
        }
    }

    pub fn header(mut self, name: impl AsRef<str>, value: impl AsHeader) -> Self {
        self.headers.push((name.as_ref().to_string(), value.as_bytes()));
        self
    }

    pub fn json<T: Serialize>(self, body: &T) -> Result<Self, HttpError> {
        let body_bytes = serde_json::to_vec(body).map_err(|e| HttpError::InternalError(Some(format!("Invalid JSON: {e}"))))?;
        Ok(Self {
            body: Some(body_bytes),
            ..self.header("content-type", "application/json")
        })
    }

    pub fn send(self) -> Result<HttpResponse, HttpError> {
        let scheme = match self.url.scheme() {
            "http" => Some(wit::Scheme::Http),
            "https" => Some(wit::Scheme::Https),
            "" => None,
            other => Some(wit::Scheme::Other(other.to_string()))
        };
        let authority = match self.url.authority() {
            "" => None,
            auth => Some(auth.to_string())
        };
        let path_and_query = match (self.url.path(), self.url.query()) {
            ("", None) => None,
            (path, None) => Some(path.to_string()),
            (path, Some(query)) => Some(format!("{path}?{query}"))
        };
        let request = wit::OutgoingRequest {
            scheme,
            authority,
            path_and_query,
            method: self.method,
            headers: self.headers,
            body: self.body,
        };
        let options = self.timeout.map(|t| {
            wit::RequestOptions {
                connect_timeout: None,
                first_byte_timeout: None,
                between_bytes_timeout: Some(t.as_nanos() as u64),
            }
        });
        let response = wit::request(&request, options)?;
        Ok(HttpResponse {
            status: response.status,
            headers: response.headers,
            body: response.body,
        })
    }

    helper_method!(get, wit::Method::Get);
    helper_method!(put, wit::Method::Put);
    helper_method!(post, wit::Method::Post);
    helper_method!(patch, wit::Method::Patch);
    helper_method!(delete, wit::Method::Delete);
}

pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(wit::FieldName, wit::FieldValue)>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn is_ok(&self) -> bool {
        self.status >= 200 && self.status < 400
    }

    pub fn json<'a, T: Deserialize<'a>>(&'a self) -> Result<T, HttpError> {
        serde_json::from_slice(&self.body).map_err(|e| HttpError::InternalError(Some(format!("Invalid JSON: {e}"))))?
    }
}

pub trait AsHeader {
    fn as_bytes(self) -> Vec<u8>;
}

impl AsHeader for String {
    fn as_bytes(self) -> Vec<u8> {
        self.into_bytes()
    }
}

impl AsHeader for &str {
    fn as_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl AsHeader for Vec<u8> {
    fn as_bytes(self) -> Vec<u8> {
        self
    }
}

impl AsHeader for &[u8] {
    fn as_bytes(self) -> Vec<u8> {
        self.to_vec()
    }
}