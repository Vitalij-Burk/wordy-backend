use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslatedDTO {
    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslateDTO {
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}
