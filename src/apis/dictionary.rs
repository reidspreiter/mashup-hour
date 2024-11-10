use super::base::{request, APIResult, RequestMethod};
use crate::Result;
use serde::{Deserialize, Serialize};

const DICTIONARY_URL: &str = "https://api.dictionaryapi.dev/api/v2/";

pub type Words = Vec<Word>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Word {
    pub word: String,
    pub origin: Option<String>,
    pub meanings: Option<Vec<Meaning>>,
}

impl Word {
    pub fn unknown(word: String) -> Self {
        Self {
            word,
            origin: None,
            meanings: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meaning {
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Definition {
    pub definition: String,
    pub example: Option<String>,
}

pub async fn search_dictionary(word: &str) -> Result<APIResult<Words>> {
    let url = format!("{DICTIONARY_URL}entries/en/{word}");
    request::<Words>(RequestMethod::GET, &url, None).await
}
