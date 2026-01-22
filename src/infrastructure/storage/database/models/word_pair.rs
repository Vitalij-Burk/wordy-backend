#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WordPairEntity {
    pub id: i32,
    pub user_id: i32,

    pub target_text: String,
    pub source_text: String,
    pub target_language: String,
    pub source_language: String,
}
