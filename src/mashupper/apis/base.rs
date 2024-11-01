use crate::{Error, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;

pub trait Pagination: DeserializeOwned {
    fn page_limit(&self) -> u64;
    fn get_pagination_url(&self, url: &str, page_index: &u64) -> String;
    fn next(&self) -> &String;
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
        request(&url).await
    }

    pub async fn next_page(&self) -> Result<APIResult<T>> {
        let url = self.response.next();
        request(&url).await
    }
}

pub async fn request<T>(url: &str) -> Result<APIResult<T>>
where
    T: DeserializeOwned,
{
    let client = Client::new();
    let res = client.get(url).send().await?;

    if res.status().is_success() {
        let text = res.text().await?;
        let response: T = serde_json::from_str(&text)?;
        return Ok(APIResult {
            url: url.to_string(),
            response,
        });
    }
    Err(Error::ResponseError {
        status_code: res.status().as_u16(),
        message: res.text().await?,
    })
}
