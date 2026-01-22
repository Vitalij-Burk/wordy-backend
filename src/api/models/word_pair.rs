use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct CreateWordPairDTO {
    pub user_id: i32,
    pub target_text: String,
    #[validate(length(min=1, max=100))]
    pub source_text: String,
    #[validate(length(min=1, max=5))]
    pub target_language: String,
    #[validate(length(min=1, max=5))]
    pub source_language: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DeleteWordPairDTO {
    ById { id: i32 },
}
