use std::{
    string::FromUtf8Error,
    sync::{Arc, LazyLock},
};

use jsonwebtoken::DecodingKey;
use thiserror::Error;
use tokio::sync::RwLock;

use crate::infrastructure::external_api::auth::auth_requests::{
    AuthCommunicator, AuthCommunicatorError,
};

static KEY: LazyLock<RwLock<Option<Arc<DecodingKey>>>> = LazyLock::new(|| RwLock::new(None));

#[derive(Debug, Clone)]
pub struct KeyManager {
    pub auth_communicator: Arc<AuthCommunicator>,
}

#[derive(Debug, Error)]
pub enum KeyManagerError {
    #[error("AuthCommunicator Error: {0}")]
    AuthCommunicator(#[from] AuthCommunicatorError),

    #[error("Jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("From utf-8 error: {0}")]
    FromUTF8(#[from] FromUtf8Error),

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Unexpected error")]
    Unexpected,
}

impl KeyManager {
    pub fn new(auth_communicator: Arc<AuthCommunicator>) -> Self {
        tracing::info!("KeyManager initialized");
        Self { auth_communicator }
    }

    pub async fn sync(&self) -> Result<(), KeyManagerError> {
        let auth_communicator = self.auth_communicator.clone();

        let _ = auth_communicator
            .consume_public_pem(Arc::new(move |pem: String| async move {
                let mut key_conf = KEY.write().await;

                *key_conf = Some(Arc::new(DecodingKey::from_rsa_pem(pem.as_bytes())?));

                Ok::<(), KeyManagerError>(())
            }))
            .await?;

        Ok(())
    }

    pub async fn fetch_latest(&self) -> Result<Arc<DecodingKey>, KeyManagerError> {
        let pem = self.auth_communicator.get_public_pem().await?;

        let mut key_conf = KEY.write().await;

        let decoding_key = Arc::new(DecodingKey::from_rsa_pem(pem.as_bytes())?);

        *key_conf = Some(decoding_key.clone());

        Ok(decoding_key)
    }

    pub async fn get_latest(&self) -> Option<Arc<DecodingKey>> {
        let key_guard = KEY.read().await;

        let key = key_guard.clone();

        key
    }
}
