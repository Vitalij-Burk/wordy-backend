use std::env::VarError;

use thiserror::Error;
use tracing::error;

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
                std::env::var("AUTH_ADDRESS").map_err(|error| match error {
                    _ => {
                        error!("{}", error.to_string());
                        error
                    }
                })?
            ))
            .send()
            .await
            .map_err(|error| match error {
                _ => {
                    error!("{}", error.to_string());
                    error
                }
            })?;

        let public_pem: String = resp.text().await.map_err(|error| match error {
            _ => {
                error!("{}", error.to_string());
                error
            }
        })?;

        Ok(public_pem)
    }
}
