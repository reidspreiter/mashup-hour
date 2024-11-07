use crate::{Error, Result};
use reqwest::{Client, Method, Response};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Serialize)]
struct SerializableNone;

pub enum RequestMethod {
    GET,
    POST,
}

impl RequestMethod {
    fn as_reqwest_method(&self) -> Method {
        match self {
            RequestMethod::GET => Method::GET,
            RequestMethod::POST => Method::POST,
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
        request_model(RequestMethod::GET, &url, None, None::<&SerializableNone>).await
    }

    pub async fn next_page(&self) -> Result<Option<APIResult<T>>> {
        match self.response.next() {
            Some(url) => Ok(Some(
                request_model(RequestMethod::GET, &url, None, None::<&SerializableNone>).await?,
            )),
            None => Ok(None),
        }
    }
}

pub async fn request<T>(
    method: RequestMethod,
    url: &str,
    headers: Option<Vec<(&str, &str)>>,
) -> Result<APIResult<T>>
where
    T: DeserializeOwned,
{
    request_model::<T, SerializableNone>(method, url, headers, None::<&SerializableNone>).await
}

pub async fn request_with_json<T, Y>(
    method: RequestMethod,
    url: &str,
    headers: Option<Vec<(&str, &str)>>,
    json_body: Option<&Y>,
) -> Result<APIResult<T>>
where
    T: DeserializeOwned,
    Y: Serialize,
{
    request_model::<T, Y>(method, url, headers, json_body).await
}

async fn _request<T>(
    method: RequestMethod,
    url: &str,
    headers: Option<Vec<(&str, &str)>>,
    json_body: Option<&T>,
) -> Result<Response>
where
    T: Serialize,
{
    let client = Client::new();
    let mut request = client.request(method.as_reqwest_method(), url);

    if let Some(_headers) = headers {
        for (key, val) in _headers {
            request = request.header(key, val);
        }
    }

    if let Some(json) = json_body {
        request = request.json(json);
    }

    let res = request.send().await?;
    if res.status().is_success() {
        return Ok(res);
    }
    Err(Error::ResponseError {
        status_code: res.status().as_u16(),
        message: res.text().await?,
    })
}

async fn request_model<T, Y>(
    method: RequestMethod,
    url: &str,
    headers: Option<Vec<(&str, &str)>>,
    json_body: Option<&Y>,
) -> Result<APIResult<T>>
where
    T: DeserializeOwned,
    Y: Serialize,
{
    let res = _request(method, url, headers, json_body).await?;
    let text = res.text().await?;
    println!("{text}");
    let model: T = serde_json::from_str(&text)?;
    Ok(APIResult {
        url: url.to_string(),
        response: model,
    })
}
