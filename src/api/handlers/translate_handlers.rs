use axum::{Json, extract::State};
use reqwest::StatusCode;

use crate::{
    AppState,
    api::{
        models::translate::{TranslateDTO, TranslatedDTO},
        types::HandlerError,
    },
    application::services::translate_service::TranslateServiceError,
};

#[axum::debug_handler]
pub async fn translate(
    State(state): State<AppState>,
    Json(dto): Json<TranslateDTO>,
) -> Result<Json<TranslatedDTO>, HandlerError> {
    let res = state
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

    let dto = TranslatedDTO {
        target_text: res.target_text,
        source_text: res.source_text,
        target_language: res.target_language,
        source_language: res.source_language,
    };

    Ok(Json(dto))
}
