use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    AppState,
    api::{
        models::user::{CreateUserDTO, UpdateUserDTO, UserDTO},
        types::HandlerError,
    },
    application::services::user_service::UserServiceError,
};

#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(dto): Json<CreateUserDTO>,
) -> Result<Json<UserDTO>, HandlerError> {
    let res = state
        .user_service
        .create(&dto)
        .await
        .map_err(|error| match error {
            UserServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            UserServiceError::InvalidKey(_) => (StatusCode::UNPROCESSABLE_ENTITY, "Key is invalid"),
            UserServiceError::InvalidPassword(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Password is too easy")
            }
            UserServiceError::KeyAlreadyExists(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Key already exists")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    let user = UserDTO::from(res);

    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<UserDTO>, HandlerError> {
    let res = state
        .user_service
        .get_by_id(&id)
        .await
        .map_err(|error| match error {
            UserServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            UserServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "User not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    let user = UserDTO::from(res);

    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn get_user_by_key(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<UserDTO>, HandlerError> {
    let res = state
        .user_service
        .get_by_key(&key)
        .await
        .map_err(|error| match error {
            UserServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            UserServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "User not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    let user = UserDTO::from(res);

    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn update_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(dto): Json<UpdateUserDTO>,
) -> Result<Json<UserDTO>, HandlerError> {
    let res = state
        .user_service
        .update_by_id(&id, &dto)
        .await
        .map_err(|error| match error {
            UserServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            UserServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "User not found"),
            UserServiceError::InvalidKey(_) => (StatusCode::UNPROCESSABLE_ENTITY, "Key is invalid"),
            UserServiceError::InvalidPassword(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Password is too easy")
            }
            UserServiceError::KeyAlreadyExists(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Key already exists")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    let user = UserDTO::from(res);

    Ok(Json(user))
}

#[axum::debug_handler]
pub async fn delete_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(), HandlerError> {
    state
        .user_service
        .delete_by_id(&id)
        .await
        .map_err(|error| match error {
            UserServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            UserServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "User not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    Ok(())
}
