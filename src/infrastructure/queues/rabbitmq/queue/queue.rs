use std::sync::Arc;

use lapin::{
    BasicProperties, Channel, Confirmation, Queue,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::{FieldTable, ShortString},
};

#[derive(Debug, Clone)]
pub struct RabbitMQueue {
    channel: Arc<Channel>,
    pub queue: Arc<Queue>,
    name: String,
}

impl RabbitMQueue {
    pub async fn new(
        channel: Arc<Channel>,
        name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let queue = Arc::new(
            channel
                .queue_declare(
                    name.into(),
                    QueueDeclareOptions::durable(),
                    FieldTable::default(),
                )
                .await?,
        );

        Ok(Self {
            channel,
            queue,
            name: name.to_string(),
        })
    }

    pub async fn publish<Exchange: Into<ShortString>, Payload: AsRef<[u8]>>(
        &self,
        exchange: Exchange,
        payload: Payload,
    ) -> Result<Confirmation, Box<dyn std::error::Error>> {
        let confirmation = self
            .channel
            .basic_publish(
                exchange.into(),
                self.name.as_str().into(),
                BasicPublishOptions::default(),
                payload.as_ref(),
                BasicProperties::default(),
            )
            .await?
            .await?;

        Ok(confirmation)
    }
}
