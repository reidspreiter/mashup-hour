use crate::{Error, Result};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;

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
        request_model(&url).await
    }

    pub async fn next_page(&self) -> Result<Option<APIResult<T>>> {
        match self.response.next() {
            Some(url) => Ok(Some(request_model(&url).await?)),
            None => Ok(None),
        }
    }
}

async fn request(url: &str) -> Result<Response> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    if res.status().is_success() {
        return Ok(res);
    }
    Err(Error::ResponseError {
        status_code: res.status().as_u16(),
        message: res.text().await?,
    })
}

pub async fn request_model<T>(url: &str) -> Result<APIResult<T>>
where
    T: DeserializeOwned,
{
    let res = request(url).await?;
    let text = res.text().await?;
    let model: T = serde_json::from_str(&text)?;
    Ok(APIResult {
        url: url.to_string(),
        response: model,
    })
}

pub async fn request_bytes(url: &str) -> Result<APIResult<bytes::Bytes>> {
    let res = request(url).await?;
    let bytes = res.bytes().await?;
    Ok(APIResult {
        url: url.to_string(),
        response: bytes,
    })
}
