use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Translation {
    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}

impl Translation {
    pub fn new(
        target_text: &str,
        source_text: &str,
        target_language: &str,
        source_language: &str,
    ) -> Self {
        Self {
            target_text: target_text.to_string(),
            source_text: source_text.to_string(),
            target_language: target_language.to_string(),
            source_language: source_language.to_string(),
        }
    }
}
