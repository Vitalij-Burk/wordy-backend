use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::api::{
    models::word_pair::{CreateWordPairDTO, GetWordPairDTO, WordPairDTO},
    types::HandlerError,
};
use crate::{
    AppState,
    application::services::{
        user_service::UserServiceError, word_pair_service::WordPairServiceError,
    },
};

#[axum::debug_handler]
pub async fn add_word_pair_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(dto): Json<CreateWordPairDTO>,
) -> Result<Json<WordPairDTO>, HandlerError> {
    let res = state
        .word_pair_service
        .create(&user_id, &dto)
        .await
        .map_err(|error| match error {
            WordPairServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            WordPairServiceError::WordPairAlreadyExists(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Word pair exists")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    Ok(Json(WordPairDTO {
        id: res.id,
        user_id: res.user_id,
        target_text: res.target_text,
        source_text: res.source_text,
        target_language: res.target_language,
        source_language: res.source_language,
    }))
}

#[axum::debug_handler]
pub async fn add_word_pair_by_user_key(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(dto): Json<CreateWordPairDTO>,
) -> Result<Json<WordPairDTO>, HandlerError> {
    let user = state
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

    let res = state
        .word_pair_service
        .create(&user.id, &dto)
        .await
        .map_err(|error| match error {
            WordPairServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            WordPairServiceError::WordPairAlreadyExists(_) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Word pair exists")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    let dto = WordPairDTO::from(res);

    Ok(Json(dto))
}

#[axum::debug_handler]
pub async fn get_word_pairs_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<WordPairDTO>>, HandlerError> {
    let res = state
        .word_pair_service
        .get(GetWordPairDTO::ByUserId { user_id: user_id })
        .await
        .map_err(|error| match error {
            WordPairServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            WordPairServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "Word pair not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    let mut dtos: Vec<WordPairDTO> = vec![];

    for res_item in res.into_iter() {
        dtos.push(WordPairDTO::from(res_item))
    }

    Ok(Json(dtos))
}

#[axum::debug_handler]
pub async fn get_word_pairs_by_user_key(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<Vec<WordPairDTO>>, HandlerError> {
    let user = state
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

    let res = state
        .word_pair_service
        .get(GetWordPairDTO::ByUserId { user_id: user.id })
        .await
        .map_err(|error| match error {
            WordPairServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            WordPairServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "Word pair not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    let mut dtos: Vec<WordPairDTO> = vec![];

    for res_item in res.into_iter() {
        dtos.push(WordPairDTO::from(res_item))
    }

    Ok(Json(dtos))
}
