use axum::{Json, extract::State, http::StatusCode};

use crate::{
    AppState,
    api::{
        models::user::{CreateUserDTO, UserDTO},
        types::HandlerError,
    },
    application::services::user_service::UserServiceError,
};

#[axum::debug_handler]
pub async fn make_user(
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

    Ok(Json(UserDTO {
        id: res.id,
        key: res.key,
        name: res.name,
    }))
}
