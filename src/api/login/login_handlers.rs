use axum::{Json, extract::State};
use tracing::error;
use reqwest::StatusCode;

use crate::{
    AppState,
    api::{
        login::models::{ClaimsDTO, LoginByKey},
        types::JsonError,
        user::models::CreateUserDTO,
    },
    application::services::user_service::UserServiceError,
    domain::traits::crypto::crypto::ICrypto,
    infrastructure::utils::password::Argon2Crypto,
};

pub async fn register(
    State(state): State<AppState>,
    Json(dto): Json<CreateUserDTO>,
) -> Result<Json<(String, (String, String))>, JsonError> {
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

    let claims = ClaimsDTO { sub: res.id };

    let client = reqwest::Client::new();

    let resp = client
        .post(format!(
            "{}/generate",
            std::env::var("AUTH_ADDRESS").map_err(|error| match error {
                _ => {
                    error!("{}", error.to_string());
                    (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't validate AUTH_ADDRESS")
                }
            })?
        ))
        .json(&claims)
        .send()
        .await.map_err(|error| match error {
            _ => {
                error!("{}", error.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        })?;

    let (access_token, (encrypted_refresh_token, nonce)) =
        resp.json::<(String, (String, String))>().await.map_err(|error| match error {
                _ => {
                    error!("{}", error.to_string());
                    (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't convert response")
                }
            })?;

    Ok(Json((access_token, (encrypted_refresh_token, nonce))))
}

pub async fn login_by_key(
    State(state): State<AppState>,
    Json(dto): Json<LoginByKey>,
) -> Result<Json<(String, (String, String))>, JsonError> {
    let user = state.user_service.get_by_key(&dto.key).await.map_err(|error| match error {
        UserServiceError::Unknown | UserServiceError::Crypto(_) | UserServiceError::Database(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
        UserServiceError::KeyAlreadyExists(_) | UserServiceError::InvalidKey(_) | UserServiceError::InvalidPassword(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
        UserServiceError::NotFound(_) => {
            (StatusCode::NOT_FOUND, "User not found")
        }
    })?;

    let hasher = Argon2Crypto;

    hasher
        .verify(&dto.plain_password, &user.hashed_password)
        .map_err(|error| match error {
            _ => {
                error!("{}", error.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        })?;

    let claims = ClaimsDTO { sub: user.id };

    let client = reqwest::Client::new();

    let resp = client
        .post(format!(
            "{}/generate",
            std::env::var("AUTH_ADDRESS").map_err(|error| match error {
                _ => {
                    error!("{}", error.to_string());
                    (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't validate AUTH_ADDRESS")
                }
            })?
        ))
        .json(&claims)
        .send()
        .await.map_err(|error| match error {
            _ => {
                error!("{}", error.to_string());
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        })?;

    let (access_token, (encrypted_refresh_token, nonce)) =
        resp.json::<(String, (String, String))>().await.map_err(|error| match error {
                _ => {
                    error!("{}", error.to_string());
                    (StatusCode::INTERNAL_SERVER_ERROR, "Couldn't convert response")
                }
            })?;

    Ok(Json((access_token, (encrypted_refresh_token, nonce))))
}
