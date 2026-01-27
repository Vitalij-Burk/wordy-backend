use crate::domain::models::word_pair::WordPair;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WordPairEntity {
    pub id: i32,
    pub user_id: i32,

    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}

impl From<WordPair> for WordPairEntity {
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
