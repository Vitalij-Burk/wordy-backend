use std::sync::Arc;

use axum::{Json, extract::State};
use reqwest::StatusCode;

use crate::{
    AppState,
    api::{
        login::models::LoginByKey,
        types::JsonError,
        user::models::{CreateUserDTO, UserDTO},
    },
    application::services::user::user_service::UserServiceError,
    domain::{traits::crypto::crypto::ICrypto, types::ID},
    infrastructure::utils::password::Argon2Crypto,
};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<CreateUserDTO>,
) -> Result<Json<UserDTO>, JsonError> {
    let res = state
        .user_service
        .create(&dto)
        .await
        .map_err(|error| match error {
            UserServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            UserServiceError::Crypto(_) => (StatusCode::BAD_REQUEST, "Bad password for request"),
            UserServiceError::InvalidKey(_) => (StatusCode::UNPROCESSABLE_ENTITY, "Key is invalid"),
            UserServiceError::InvalidPassword(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Password is too easy")
            }
            UserServiceError::KeyAlreadyExists(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Key already exists")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    Ok(Json(UserDTO::from(res)))
}

pub async fn login_by_key(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<LoginByKey>,
) -> Result<Json<ID>, JsonError> {
    let user = state
        .user_service
        .get_by_key(&dto.key)
        .await
        .map_err(|error| match error {
            UserServiceError::Unknown
            | UserServiceError::Crypto(_)
            | UserServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            UserServiceError::KeyAlreadyExists(_)
            | UserServiceError::InvalidKey(_)
            | UserServiceError::InvalidPassword(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            UserServiceError::NotFound(_) => (StatusCode::UNAUTHORIZED, "User unauthorized"),
        })?;

    let hasher = Argon2Crypto;

    hasher
        .verify(&dto.password, &user.hashed_password)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "User unauthorized"))?;

    Ok(Json(user.id))
}
