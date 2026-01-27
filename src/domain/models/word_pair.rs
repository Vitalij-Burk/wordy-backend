use heck::ToTitleCase;
use rand::Rng;

use crate::infrastructure::storage::database::models::word_pair::WordPairEntity;

#[derive(Debug, Clone)]
pub struct WordPair {
    pub id: i32,
    pub user_id: i32,

    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}

impl WordPair {
    pub fn new(
        user_id: &i32,
        target_text: &str,
        source_text: &str,
        target_language: &str,
        source_language: &str,
    ) -> Self {
        let id = rand::rng().random::<i32>();

        Self {
            id: id,
            user_id: *user_id,
            target_text: target_text.to_string().to_title_case(),
            source_text: source_text.to_string().to_title_case(),
            target_language: target_language.to_string().to_lowercase(),
            source_language: source_language.to_string().to_lowercase(),
        }
    }
}

impl From<WordPairEntity> for WordPair {
    fn from(value: WordPairEntity) -> Self {
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
