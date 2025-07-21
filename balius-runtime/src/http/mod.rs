use std::{error::Error, time::Duration};

use crate::wit::balius::app::http as wit;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Method,
};

#[derive(Clone)]
pub enum Http {
    Mock,
    Reqwest(reqwest::Client),
}

impl wit::Host for Http {
    async fn request(
        &mut self,
        request: wit::OutgoingRequest,
        options: Option<wit::RequestOptions>,
    ) -> Result<wit::IncomingResponse, wit::ErrorCode> {
        match self {
            Self::Mock => Ok(wit::IncomingResponse {
                status: 200,
                headers: vec![],
                body: vec![],
            }),
            Self::Reqwest(client) => {
                let scheme = match &request.scheme {
                    Some(wit::Scheme::Http) => "http",
                    Some(wit::Scheme::Https) => "https",
                    Some(wit::Scheme::Other(scheme)) => scheme,
                    None => "http",
                };
                let uri = match (
                    request.authority.as_deref(),
                    request.path_and_query.as_deref(),
                ) {
                    (None, None) => return Err(wit::ErrorCode::HttpRequestUriInvalid),
                    (auth, path) => format!(
                        "{scheme}://{}{}",
                        auth.unwrap_or_default(),
                        path.unwrap_or_default()
                    ),
                };

                let method = match request.method {
                    wit::Method::Get => Method::GET,
                    wit::Method::Head => Method::HEAD,
                    wit::Method::Post => Method::POST,
                    wit::Method::Put => Method::PUT,
                    wit::Method::Delete => Method::DELETE,
                    wit::Method::Connect => Method::CONNECT,
                    wit::Method::Options => Method::OPTIONS,
                    wit::Method::Trace => Method::TRACE,
                    wit::Method::Patch => Method::PATCH,
                    wit::Method::Other(name) => Method::from_bytes(name.as_bytes())
                        .map_err(|_| wit::ErrorCode::HttpRequestMethodInvalid)?,
                };

                let mut header_map = HeaderMap::new();
                for (key, value) in request.headers {
                    let header_name = HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
                        wit::ErrorCode::InternalError(Some(format!(
                            "Invalid header name \"{key}\": {e}"
                        )))
                    })?;
                    let header_value = HeaderValue::from_bytes(&value).map_err(|e| {
                        wit::ErrorCode::InternalError(Some(format!(
                            "Invalid header value for \"{key}\": {e}"
                        )))
                    })?;
                    header_map.append(header_name, header_value);
                }

                let mut builder = client.request(method, uri).headers(header_map);
                if let Some(body) = request.body {
                    builder = builder.body(body);
                }

                let mut request = builder
                    .build()
                    .map_err(|_| wit::ErrorCode::HttpRequestUriInvalid)?;

                if let Some(timeout) = options.and_then(|o| o.between_bytes_timeout) {
                    request.timeout_mut().replace(Duration::from_nanos(timeout));
                }

                let response = client
                    .execute(request)
                    .await
                    .map_err(map_reqwest_response_err)?;

                let status = response.status().as_u16();
                let headers = response
                    .headers()
                    .into_iter()
                    .map(|(header, value)| {
                        let key = header.to_string();
                        let val = value.as_bytes().to_vec();
                        (key, val)
                    })
                    .collect();
                let body = response
                    .bytes()
                    .await
                    .map_err(map_reqwest_response_err)?
                    .to_vec();

                Ok(wit::IncomingResponse {
                    status,
                    headers,
                    body,
                })
            }
        }
    }
}

fn map_reqwest_response_err(e: reqwest::Error) -> wit::ErrorCode {
    if e.is_timeout() {
        wit::ErrorCode::HttpResponseTimeout
    } else {
        let message = match e.source() {
            Some(source) => format!("{}: {}", e, source),
            None => e.to_string(),
        };
        wit::ErrorCode::InternalError(Some(message))
    }
}
