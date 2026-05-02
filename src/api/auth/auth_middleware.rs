use axum::{
    extract::{Request, State},
    http::header,
    middleware::Next,
    response::Response,
};
use reqwest::StatusCode;

use crate::{
    AppState, api::types::JsonError, application::services::auth_service::AuthServiceError,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, JsonError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Invalid bearer token"))?;

    let claims = state
        .auth_service
        .validate_token(&token)
        .await
        .map_err(|error| match error {
            AuthServiceError::AuthCommunicator(_)
            | AuthServiceError::IO(_)
            | AuthServiceError::Unknown
            | AuthServiceError::FromUtf8(_)
            | AuthServiceError::Validation(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AuthServiceError::Expired(_) => (StatusCode::UNAUTHORIZED, "Token is expired"),
        })?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
