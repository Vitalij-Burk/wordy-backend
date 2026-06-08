use std::{string::FromUtf8Error, sync::Arc};
use tokio::{sync::Mutex, task::JoinHandle};

use crate::{
    application::services::auth::key::key_manager::{KeyManager, KeyManagerError},
    domain::error::error_handling::log_err,
    infrastructure::{
        auth::token::jwks::jwks_validator::{JwksTokenValidator, JwksTokenValidatorError},
        external_api::auth::auth_requests::{AuthCommunicator, AuthCommunicatorError},
        queues::rabbitmq::rabbitmq::RabbitMQChannel,
    },
};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AuthService {
    pub validator: JwksTokenValidator,
    pub key_manager: KeyManager,
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

    #[error("Error: {0}")]
    Standard(#[from] Box<dyn std::error::Error>),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Key manager error: {0}")]
    KeyManager(#[from] KeyManagerError),
}

impl AuthService {
    pub async fn new(
        rabbit_channel: Arc<Mutex<RabbitMQChannel>>,
    ) -> Result<Self, AuthServiceError> {
        let validator = JwksTokenValidator::new();

        let auth_communicator = Arc::new(
            AuthCommunicator::new(
                rabbit_channel,
                &std::env::var("AUTH_SERVER").expect("No AUTH_SERVER"),
            )
            .await?,
        );

        let key_manager = KeyManager::new(auth_communicator);

        tracing::info!("AuthService initialized");

        Ok(Self {
            validator,
            key_manager,
        })
    }

    pub async fn validate_token(&self, token: &str) -> Result<(), AuthServiceError> {
        if let Some(key) = self.key_manager.get_latest().await {
            let _ = self.validator.verify(&token, &key).map_err(log_err)?;
        } else {
            let key = self.key_manager.fetch_latest().await.map_err(log_err)?;

            let _ = self.validator.verify(&token, &key).map_err(log_err)?;
        }

        Ok(())
    }

    pub async fn start_http_is_alive_checks(&self) -> JoinHandle<()> {
        let key_manager = self.key_manager.clone();

        tokio::spawn(async move { key_manager.auth_communicator.http_is_alive_checker().await })
    }

    pub async fn start_rabbitmq_is_alive_checks(&self) -> JoinHandle<()> {
        let key_manager = self.key_manager.clone();

        tokio::spawn(async move {
            key_manager
                .auth_communicator
                .rabbitmq_is_alive_checker()
                .await
        })
    }
}
