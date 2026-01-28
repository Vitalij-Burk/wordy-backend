use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::word_pair::WordPair;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WordPairDTO {
    pub id: i32,
    pub user_id: i32,

    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}

impl From<WordPair> for WordPairDTO {
    fn from(value: WordPair) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            target_text: value.target_text,
            source_text: value.source_text,
            target_language: value.target_language,
            source_language: value.source_language,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct CreateWordPairDTO {
    pub target_text: String,
    #[validate(length(min = 1, max = 100))]
    pub source_text: String,
    #[validate(length(min = 1, max = 5))]
    pub target_language: String,
    #[validate(length(min = 1, max = 5))]
    pub source_language: String,
}
