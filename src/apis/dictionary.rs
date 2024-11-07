use super::base::{request_model, APIResult, RequestMethod, SerializableNone};
use crate::Result;
use serde::Deserialize;

pub type Words = Vec<Word>;

const DICTIONARY_URL: &str = "https://api.dictionaryapi.dev/api/v2/";

#[derive(Debug, Deserialize)]
pub struct Word {
    pub word: String,
    pub origin: String,
    pub meanings: Vec<Meaning>,
}

#[derive(Debug, Deserialize)]
pub struct Meaning {
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    pub definition: String,
    pub example: String,
}

pub async fn search_dictionary(word: &str) -> Result<APIResult<Words>> {
    let url = format!("{DICTIONARY_URL}entries/en/{word}");
    request_model::<Words, SerializableNone>(
        RequestMethod::GET,
        &url,
        None,
        None::<&SerializableNone>,
    )
    .await
}
