use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_qs::web::QsQuery;

use crate::{
    AppState,
    api::{
        translate::models::TranslateDTO,
        word_pair::models::{CreateWordPairDTO, WordPairDTO},
    },
    application::services::{
        translate_service::TranslateServiceError, user_service::UserServiceError,
        word_pair_service::WordPairServiceError,
    },
    domain::models::{sort::GetWordPairsQueryList, word_pair::WordPair},
};
use crate::{api::types::HandlerError, domain::types::ID};

pub async fn translate_and_add_word_pair_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<ID>,
    Json(dto): Json<TranslateDTO>,
) -> Result<Json<WordPairDTO>, HandlerError> {
    let translated =
        state
            .translate_service
            .translate_text(&dto)
            .await
            .map_err(|error| match error {
                TranslateServiceError::TranslatorError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Translation failed")
                }
                TranslateServiceError::NotFoundLanguage(_) => {
                    (StatusCode::BAD_REQUEST, "Language not found")
                }
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
            })?;

    let create_word_pair_dto = CreateWordPairDTO {
        target_text: translated.target_text,
        source_text: translated.source_text,
        target_language: translated.target_language,
        source_language: translated.source_language,
    };

    let res = state
        .word_pair_service
        .create(&user_id, &create_word_pair_dto)
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

pub async fn translate_and_add_word_pair_by_user_key(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(dto): Json<TranslateDTO>,
) -> Result<Json<WordPairDTO>, HandlerError> {
    let translated =
        state
            .translate_service
            .translate_text(&dto)
            .await
            .map_err(|error| match error {
                TranslateServiceError::TranslatorError(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Translation failed")
                }
                TranslateServiceError::NotFoundLanguage(_) => {
                    (StatusCode::BAD_REQUEST, "Language not found")
                }
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
            })?;

    let create_word_pair_dto = CreateWordPairDTO {
        target_text: translated.target_text,
        source_text: translated.source_text,
        target_language: translated.target_language,
        source_language: translated.source_language,
    };

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
        .create(&user.id, &create_word_pair_dto)
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

pub async fn add_word_pair_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<ID>,
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

    let dto = WordPairDTO::from(res);

    Ok(Json(dto))
}

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

pub async fn get_word_pair_by_id(
    State(state): State<AppState>,
    Path(id): Path<ID>,
) -> Result<Json<WordPairDTO>, HandlerError> {
    let res = state
        .word_pair_service
        .get_by_id(&id)
        .await
        .map_err(|error| match error {
            WordPairServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            WordPairServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "Word pair not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    let word_pair = WordPairDTO::from(res);

    Ok(Json(word_pair))
}

pub async fn get_word_pairs_by_user_id(
    State(state): State<AppState>,
    Path(user_id): Path<ID>,
    QsQuery(query): QsQuery<Option<GetWordPairsQueryList>>,
) -> Result<Json<Vec<WordPairDTO>>, HandlerError> {
    let res: Vec<WordPair>;

    if let Some(query_params) = query {
        res = state
            .word_pair_service
            .get_by_user_id_with_query_data(&user_id, query_params)
            .await
            .map_err(|error| match error {
                WordPairServiceError::Database(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                }
                WordPairServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "Word pair not found"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
            })?;
    } else {
        res = state
            .word_pair_service
            .get_by_user_id(&user_id)
            .await
            .map_err(|error| match error {
                WordPairServiceError::Database(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                }
                WordPairServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "Word pair not found"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
            })?;
    }

    let mut dtos: Vec<WordPairDTO> = vec![];

    for res_item in res.into_iter() {
        dtos.push(WordPairDTO::from(res_item))
    }

    Ok(Json(dtos))
}

pub async fn get_word_pairs_by_user_key(
    State(state): State<AppState>,
    Path(key): Path<String>,
    QsQuery(query): QsQuery<Option<GetWordPairsQueryList>>,
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

    let res: Vec<WordPair>;

    if let Some(query_params) = query {
        res = state
            .word_pair_service
            .get_by_user_id_with_query_data(&user.id, query_params)
            .await
            .map_err(|error| match error {
                WordPairServiceError::Database(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                }
                WordPairServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "Word pair not found"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
            })?;
    } else {
        res = state
            .word_pair_service
            .get_by_user_id(&user.id)
            .await
            .map_err(|error| match error {
                WordPairServiceError::Database(_) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                }
                WordPairServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "Word pair not found"),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
            })?;
    }

    let mut dtos: Vec<WordPairDTO> = vec![];

    for res_item in res.into_iter() {
        dtos.push(WordPairDTO::from(res_item))
    }

    Ok(Json(dtos))
}

pub async fn delete_word_pair_by_id(
    State(state): State<AppState>,
    Path(id): Path<ID>,
) -> Result<(), HandlerError> {
    state
        .word_pair_service
        .delete_by_id(&id)
        .await
        .map_err(|error| match error {
            WordPairServiceError::Database(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            WordPairServiceError::NotFound(_) => (StatusCode::NOT_FOUND, "Word pair not found"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error"),
        })?;

    Ok(())
}
