use std::{path::PathBuf, string::FromUtf8Error};

use crate::{
    api::auth::models::Claims,
    infrastructure::{
        auth::token::jwks::jwks_validator::{JwksTokenValidator, JwksTokenValidatorError},
        external_api::auth::auth_requests::{AuthCommunicator, AuthCommunicatorError},
        utils::io::files::files_io::FileIO,
    },
};
use chrono::Utc;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Clone)]
pub struct AuthService {
    pub validator: JwksTokenValidator,
    pub http_client: reqwest::Client,
    pub public_pem_file_io: FileIO,
    pub auth_communicator: AuthCommunicator,
}

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Unknown error")]
    Unknown,

    #[error("From UTF-8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("Validation error: {0}")]
    Validation(#[from] JwksTokenValidatorError),

    #[error("Token expired: {0}")]
    Expired(String),

    #[error("Auth communicator error: {0}")]
    AuthCommunicator(#[from] AuthCommunicatorError),
}

impl AuthService {
    pub fn new() -> Result<Self, AuthServiceError> {
        let validator = JwksTokenValidator;
        let http_client = reqwest::Client::new();
        let keys_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("keys");

        if !keys_dir.exists() {
            let _ = std::fs::create_dir(&keys_dir).map_err(|error| match error {
                err => {
                    error!("Couldn't create keys directory: {}", &err);
                    err
                }
            });
        }

        let public_pem_file_io = FileIO::new(
            keys_dir
                .join("public.pem")
                .to_str()
                .ok_or_else(|| AuthServiceError::Unknown)?,
        );
        let auth_communicator = AuthCommunicator;

        Ok(Self {
            validator,
            http_client,
            public_pem_file_io,
            auth_communicator,
        })
    }

    pub async fn validate_token(&self, token: &str) -> Result<Claims, AuthServiceError> {
        let public_pem: String;

        match self.public_pem_file_io.read() {
            Ok(resp) if !resp.is_empty() => {
                public_pem = String::from_utf8(resp)?;
            }
            _ => {
                error!("Public pem file not working");
                public_pem = self.auth_communicator.get_public_pem().await?;

                match self.public_pem_file_io.write(&public_pem) {
                    Ok(_) => {}
                    _ => {
                        error!("Public pem file not working");
                    }
                }
            }
        }
        let claims = self.validator.verify(&token, &public_pem)?;

        let datetime_now = Utc::now();

        if claims.exp < datetime_now {
            return Err(AuthServiceError::Expired(claims.exp.to_string()));
        }

        Ok(claims)
    }
}
