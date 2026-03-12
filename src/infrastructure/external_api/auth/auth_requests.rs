use std::env::VarError;

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
            .post(format!("{}/key/public", std::env::var("AUTH_ADDRESS")?))
            .send()
            .await?;

        let public_pem: String = resp.json::<String>().await?;

        Ok(public_pem)
    }
}
