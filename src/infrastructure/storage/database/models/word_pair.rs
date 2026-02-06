use crate::{
    domain::{models::word_pair::WordPair, types::ID},
    infrastructure::utils::convert::{
        datetime_to_primitive::convert_datetime_utc_to_primitive,
        primitive_to_datetime::convert_primitive_to_datetime_utc,
    },
};
use sqlx::types::time::PrimitiveDateTime;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WordPairEntity {
    pub id: ID,
    pub user_id: ID,

    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,

    pub created_at: PrimitiveDateTime,
}

impl From<&WordPair> for WordPairEntity {
    fn from(value: &WordPair) -> Self {
        let created_at = convert_datetime_utc_to_primitive(&value.created_at);

        Self {
            id: value.id,
            user_id: value.user_id,
            target_text: value.target_text.to_owned(),
            source_text: value.source_text.to_owned(),
            target_language: value.target_language.to_owned(),
            source_language: value.source_language.to_owned(),
            created_at: created_at,
        }
    }
}

impl From<&WordPairEntity> for WordPair {
    fn from(value: &WordPairEntity) -> Self {
        let created_datetime = convert_primitive_to_datetime_utc(&value.created_at);

        Self {
            id: value.id,
            user_id: value.user_id,
            target_text: value.target_text.to_owned(),
            source_text: value.source_text.to_owned(),
            target_language: value.target_language.to_owned(),
            source_language: value.source_language.to_owned(),
            created_at: created_datetime,
        }
    }
}
