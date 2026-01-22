use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct CreateUserDTO {
    #[validate(length(min=3, max=30))]
    pub key: String,
    #[validate(length(min=2, max=20))]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct UpdateUserDTO {
    #[validate(length(min=3, max=30))]
    pub key: Option<String>,
    #[validate(length(min=2, max=20))]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DeleteUserDTO {
    ById { id: i32 },
    ByKey { key: String },
}
