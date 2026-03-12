use axum::{Json, extract::State};

use crate::{
    AppState,
    api::{
        login::models::{ClaimsDTO, LoginByKey},
        types::JsonError,
    },
    domain::traits::crypto::crypto::ICrypto,
    infrastructure::utils::password::Argon2Crypto,
};

pub async fn login_by_key(
    State(state): State<AppState>,
    Json(dto): Json<LoginByKey>,
) -> Result<(String, (String, String)), JsonError> {
    let user = state.user_service.get_by_key(&dto.key).await.unwrap();

    let hasher = Argon2Crypto;

    hasher
        .verify(&dto.plain_password, &user.hashed_password)
        .unwrap();

    let claims = ClaimsDTO { sub: user.id };

    let client = reqwest::Client::new();

    let resp = client
        .post(format!(
            "{}/generate",
            std::env::var("AUTH_ADDRESS").unwrap()
        ))
        .json(&claims)
        .send()
        .await
        .unwrap();

    let (access_token, (encrypted_refresh_token, nonce)) =
        resp.json::<(String, (String, String))>().await.unwrap();

    Ok((access_token, (encrypted_refresh_token, nonce)))
}
