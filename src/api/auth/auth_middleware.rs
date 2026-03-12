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
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    if !auth_header.starts_with("Bearer") {
        return Err((StatusCode::UNAUTHORIZED, "User unauthorized"))
    }

    let token = auth_header.strip_prefix("Bearer ").unwrap_or("").to_string();

    if token.is_empty() {
        return Err((StatusCode::UNAUTHORIZED, "User unauthorized"))
    }

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
        })?;

    let is_expired = state.auth_service.check_exp(&claims);

    if is_expired {
        return Err((StatusCode::UNAUTHORIZED, "User unauthorized"))
    }

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
