use futures_lite::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};
use reqwest::Client;
use sha2::digest::consts::True;
use std::{
    env::VarError,
    string::FromUtf8Error,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::{sync::Mutex, time::sleep};

use crate::infrastructure::queues::rabbitmq::{consumer, rabbitmq::RabbitMQChannel};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AuthCommunicator {
    rabbit_channel: Arc<Mutex<RabbitMQChannel>>,
    client: Client,
    is_http_alive: Arc<AtomicBool>,
    is_rabbitmq_alive: Arc<AtomicBool>,
    auth_server_path: String,
}

#[derive(Debug, Error)]
pub enum AuthCommunicatorError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Var error: {0}")]
    Var(#[from] VarError),

    #[error("Error: {0}")]
    Standard(#[from] Box<dyn std::error::Error>),

    #[error("Recv error: {0}")]
    Recv(#[from] tokio::sync::oneshot::error::RecvError),

    #[error("From utf-8 error: {0}")]
    FromUTF8(#[from] FromUtf8Error),

    #[error("Service unavailable")]
    Unavailable,
}

impl AuthCommunicator {
    pub async fn new(
        rabbit_channel: Arc<Mutex<RabbitMQChannel>>,
        auth_server_path: &str,
    ) -> Result<Self, AuthCommunicatorError> {
        let client = Client::new();

        let is_http_alive: Arc<AtomicBool>;
        let is_rabbitmq_alive: Arc<AtomicBool>;

        match client.get(format!("{}/", &auth_server_path)).send().await {
            Ok(_) => {
                is_http_alive = Arc::new(AtomicBool::new(true));
            }
            Err(_) => {
                is_http_alive = Arc::new(AtomicBool::new(false));
                tracing::warn!("Auth http isn't alive")
            }
        }

        if rabbit_channel.lock().await.channel.status().connected() {
            is_rabbitmq_alive = Arc::new(AtomicBool::new(true));
        } else {
            is_rabbitmq_alive = Arc::new(AtomicBool::new(false));
            tracing::warn!("Auth rabbitMQ isn't alive")
        }

        tracing::info!("AuthCommunicator initialized");

        Ok(Self {
            rabbit_channel,
            client,
            is_http_alive,
            is_rabbitmq_alive,
            auth_server_path: auth_server_path.to_string(),
        })
    }

    pub async fn rabbitmq_is_alive_checker(&self) {
        loop {
            match self
                .rabbit_channel
                .lock()
                .await
                .channel
                .status()
                .connected()
            {
                true => {
                    if !self.is_rabbitmq_alive.load(Ordering::Relaxed) {
                        tracing::info!("Auth RabbitMQ is alive again")
                    }
                    self.is_rabbitmq_alive.store(true, Ordering::Relaxed);
                }
                false => {
                    if self.is_rabbitmq_alive.load(Ordering::Relaxed) {
                        tracing::warn!("Auth RabbitMQ dropped")
                    }
                    self.is_rabbitmq_alive.store(false, Ordering::Relaxed);
                }
            }
            self.is_rabbitmq_alive.store(
                self.rabbit_channel
                    .lock()
                    .await
                    .channel
                    .status()
                    .connected(),
                Ordering::Relaxed,
            );

            sleep(Duration::from_secs(30)).await;
        }
    }

    pub async fn http_is_alive_checker(&self) {
        loop {
            match self
                .client
                .get(format!("{}/", self.auth_server_path))
                .send()
                .await
            {
                Ok(_) => {
                    if !self.is_http_alive.load(Ordering::Relaxed) {
                        tracing::info!("Auth http is alive again")
                    }
                    self.is_http_alive.store(true, Ordering::Relaxed);
                }
                Err(error) => {
                    if self.is_http_alive.load(Ordering::Relaxed) {
                        tracing::warn!("Auth http dropped: {}", error)
                    }
                    self.is_http_alive.store(false, Ordering::Relaxed);
                }
            }
            sleep(Duration::from_secs(30)).await;
        }
    }

    pub async fn get_public_pem(&self) -> Result<String, AuthCommunicatorError> {
        if !self.is_http_alive.load(Ordering::Relaxed) {
            return Err(AuthCommunicatorError::Unavailable);
        }

        let resp = self
            .client
            .get(format!("{}/key/public", &self.auth_server_path))
            .send()
            .await?
            .text()
            .await?;

        Ok(resp)
    }

    pub async fn consume_public_pem<F, Fut, Err>(
        &self,
        on_consume: Arc<F>,
    ) -> Result<(), AuthCommunicatorError>
    where
        F: Fn(String) -> Fut + Sync + Send + 'static,
        Fut: Future<Output = Result<(), Err>> + Send,
    {
        while !self.is_rabbitmq_alive.load(Ordering::Relaxed) {
            let _ = sleep(Duration::from_secs(1));
        }

        let channel = self.rabbit_channel.lock().await;

        let consumer = channel.declare_consumer("public_pem", "pem").await?;

        tokio::spawn(async move {
            let _ = consumer
                .lock()
                .await
                .consume(move |delivery| {
                    let data = delivery.data.clone();
                    let on_consume = on_consume.clone();

                    async move {
                        let pem = String::from_utf8(data)?;

                        let _ = on_consume(pem).await;

                        Ok(())
                    }
                })
                .await;
        });

        Ok(())
    }
}
