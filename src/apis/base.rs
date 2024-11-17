use crate::{Error, Result};
use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use reqwest::{Client, Method, RequestBuilder as ReqwestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};

pub enum RequestMethod {
    GET,
    POST,
    DELETE,
}

impl RequestMethod {
    fn as_reqwest_method(&self) -> Method {
        match self {
            RequestMethod::GET => Method::GET,
            RequestMethod::POST => Method::POST,
            RequestMethod::DELETE => Method::DELETE,
        }
    }
}

pub enum ContentType {
    JSON,
}

impl ContentType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::JSON => "application/json",
        }
    }
}

pub trait Pagination: DeserializeOwned {
    fn page_limit(&self) -> u64;
    fn get_pagination_url(&self, url: &str, page_index: &u64) -> String;
    fn next(&self) -> &Option<String>;
}

pub struct APIResult<T> {
    pub url: String,
    pub response: T,
}

impl<T> APIResult<T>
where
    T: Pagination,
{
    pub async fn get_page_from_index(&self, entry_index: &u64) -> Result<APIResult<T>> {
        let page_index = entry_index / self.response.page_limit();
        let url = self.response.get_pagination_url(&self.url, &page_index);
        request_builder(RequestMethod::GET, &url)
            .request_model()
            .await
    }

    pub async fn next_page(&self) -> Result<Option<APIResult<T>>> {
        match self.response.next() {
            Some(url) => Ok(Some(
                request_builder(RequestMethod::GET, &url)
                    .request_model()
                    .await?,
            )),
            None => Ok(None),
        }
    }
}

pub struct RequestBuilder {
    url: String,
    request: ReqwestBuilder,
}

impl RequestBuilder {
    pub fn new(method: RequestMethod, url: &str) -> Self {
        let client = Client::new();
        let request = client.request(method.as_reqwest_method(), url);
        Self {
            url: url.to_string(),
            request,
        }
    }

    pub fn json<T>(mut self, json: &T) -> Self
    where
        T: Serialize,
    {
        self.request = self.request.json(json);
        self
    }

    pub fn header(mut self, key: &str, val: &str) -> Self {
        self.request = self.request.header(key, val);
        self
    }

    pub fn content_type(mut self, content_type: ContentType) -> Self {
        self.request = self.request.header("Content-Type", content_type.as_str());
        self
    }

    pub fn bearer(mut self, token: &str) -> Self {
        self.request = self.request.bearer_auth(token);
        self
    }

    pub async fn request(self) -> Result<Response> {
        let res = self.request.send().await?;
        if res.status().is_success() {
            return Ok(res);
        }
        Err(Error::ResponseError {
            status_code: res.status().as_u16(),
            message: res.text().await?,
        })
    }

    pub async fn request_model<T>(self) -> Result<APIResult<T>>
    where
        T: DeserializeOwned,
    {
        let url = self.url.clone();
        let res = self.request().await?;
        let text = res.text().await?;
        let model: T = serde_json::from_str(&text)?;
        Ok(APIResult {
            url,
            response: model,
        })
    }

    pub async fn request_bytes(self) -> Result<Bytes> {
        let res = self.request().await?;
        let bytes = res.bytes().await?;
        Ok(bytes)
    }

    pub async fn request_b64(self) -> Result<String> {
        let bytes = self.request_bytes().await?;
        let b64 = general_purpose::STANDARD.encode(bytes);
        Ok(b64)
    }
}

pub fn request_builder(method: RequestMethod, url: &str) -> RequestBuilder {
    RequestBuilder::new(method, url)
}
