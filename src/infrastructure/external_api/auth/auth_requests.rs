use std::env::VarError;

use crate::domain::error::error_handling::log_err;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct AuthCommunicator;

#[derive(Debug, Error)]
pub enum AuthCommunicatorError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Var error: {0}")]
    Var(#[from] VarError),
}

impl AuthCommunicator {
    pub async fn get_public_pem(&self) -> Result<String, AuthCommunicatorError> {
        let client = reqwest::Client::new();

        let resp = client
            .get(format!(
                "{}/key/public",
                std::env::var("AUTH_ADDRESS").map_err(log_err)?
            ))
            .send()
            .await
            .map_err(log_err)?;

        let public_pem: String = resp.text().await.map_err(log_err)?;

        Ok(public_pem)
    }
}
