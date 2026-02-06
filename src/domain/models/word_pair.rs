use chrono::{DateTime, Utc};
use heck::ToTitleCase;
use uuid::Uuid;

use crate::domain::types::ID;

#[derive(Debug, Clone)]
pub struct WordPair {
    pub id: ID,
    pub user_id: ID,

    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,

    pub created_at: DateTime<Utc>,
}

impl WordPair {
    pub fn new(
        user_id: ID,
        target_text: String,
        source_text: String,
        target_language: String,
        source_language: String,
        created_at: Option<DateTime<Utc>>,
    ) -> Self {
        let id = Uuid::new_v4();

        let created_time: DateTime<Utc>;

        match created_at {
            None => {
                created_time = Utc::now();
            }
            Some(time) => {
                created_time = time;
            }
        }

        Self {
            id: id,
            user_id: user_id,
            target_text: target_text.to_title_case(),
            source_text: source_text.to_title_case(),
            target_language: target_language.to_lowercase(),
            source_language: source_language.to_lowercase(),
            created_at: created_time,
        }
    }

    pub fn new_simple(
        user_id: ID,
        target_text: String,
        source_text: String,
        target_language: String,
        source_language: String,
    ) -> Self {
        let id = Uuid::new_v4();

        let created_time = Utc::now();

        Self {
            id: id,
            user_id: user_id,
            target_text: target_text.to_title_case(),
            source_text: source_text.to_title_case(),
            target_language: target_language.to_lowercase(),
            source_language: source_language.to_lowercase(),
            created_at: created_time,
        }
    }
}
