use super::base::{request_builder, ContentType, RequestBuilder, RequestMethod};
use crate::Result;
use serde::{de::DeserializeOwned, Serialize};
use std::env;

fn supabase_request_builder(method: RequestMethod, url: &str, key: &str) -> Result<RequestBuilder> {
    let mut builder = request_builder(method, url);
    builder = builder
        .header("apikey", key)
        .bearer(key)
        .content_type(ContentType::JSON)
        .header("Prefer", "return=representation");
    Ok(builder)
}

pub enum OrderDirection {
    DESC,
}

impl OrderDirection {
    fn as_str(&self) -> &'static str {
        match self {
            OrderDirection::DESC => "desc",
        }
    }
}

pub struct SupabaseClient {
    base_url: String,
    key: String,
}

impl SupabaseClient {
    pub fn new() -> Result<Self> {
        let base_url = env::var("SUPABASE_URL")?;
        let key = env::var("SUPABASE_RLS_KEY")?;
        Ok(Self { base_url, key })
    }

    pub fn from(self, table_name: &str) -> SupabaseTable {
        SupabaseTable {
            client: self,
            table_name: table_name.to_string(),
        }
    }
}
pub struct SupabaseTable {
    client: SupabaseClient,
    table_name: String,
}

impl SupabaseTable {
    fn table_url(&self) -> String {
        format!("{}/{}", self.client.base_url, self.table_name)
    }

    pub fn select(self) -> SelectBuilder {
        SelectBuilder::new(self)
    }

    pub fn insert<J>(self, row: J) -> InsertBuilder<J>
    where
        J: Serialize + DeserializeOwned,
    {
        let rows = vec![row];
        InsertBuilder::new(self, rows)
    }

    pub fn delete(self) -> DeleteBuilder {
        DeleteBuilder::new(self)
    }
}

pub struct SelectBuilder {
    table: SupabaseTable,
    columns: Option<&'static str>,
    limit: Option<u64>,
    order_column: Option<&'static str>,
    order_direction: Option<OrderDirection>,
}

impl SelectBuilder {
    pub fn new(table: SupabaseTable) -> Self {
        Self {
            table,
            columns: None,
            limit: None,
            order_column: None,
            order_direction: None,
        }
    }

    pub fn order(mut self, column: &'static str, direction: OrderDirection) -> Self {
        self.order_column = Some(column);
        self.order_direction = Some(direction);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub async fn request<T>(&self) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let columns = self.columns.unwrap_or("*");
        let mut url = format!("{}?select={}", self.table.table_url(), columns);

        if let (Some(column), Some(direction)) = (self.order_column, &self.order_direction) {
            url.push_str(&format!("&order={}.{}", column, direction.as_str()));
        }
        if let Some(l) = self.limit {
            url.push_str(&format!("&limit={l}"));
        }
        Ok(
            supabase_request_builder(RequestMethod::GET, &url, &self.table.client.key)?
                .request_model::<Vec<T>>()
                .await?
                .response,
        )
    }
}

pub struct InsertBuilder<J>
where
    J: Serialize + DeserializeOwned,
{
    table: SupabaseTable,
    rows: Vec<J>,
}

impl<J> InsertBuilder<J>
where
    J: Serialize + DeserializeOwned,
{
    pub fn new(table: SupabaseTable, rows: Vec<J>) -> Self {
        Self { table, rows }
    }

    pub async fn request(&self) -> Result<Vec<J>> {
        let url = self.table.table_url();
        Ok(
            supabase_request_builder(RequestMethod::POST, &url, &self.table.client.key)?
                .json(&self.rows)
                .request_model::<Vec<J>>()
                .await?
                .response,
        )
    }
}

pub struct DeleteBuilder {
    table: SupabaseTable,
    notin_column: Option<&'static str>,
    notin_values: Option<Vec<String>>,
}

impl DeleteBuilder {
    pub fn new(table: SupabaseTable) -> Self {
        Self {
            table,
            notin_column: None,
            notin_values: None,
        }
    }

    pub fn notin<I>(mut self, column: &'static str, values: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        let notin_values: Vec<String> = values.into_iter().collect();
        self.notin_values = Some(notin_values);
        self.notin_column = Some(column);
        self
    }

    pub async fn request<T>(&self) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let mut url = self.table.table_url();

        if let (Some(column), Some(values)) = (self.notin_column, &self.notin_values) {
            url.push_str(&format!("?{}=not.in.({})", column, values.join(",")));
        }
        Ok(
            supabase_request_builder(RequestMethod::DELETE, &url, &self.table.client.key)?
                .request_model::<Vec<T>>()
                .await?
                .response,
        )
    }
}
