use std::sync::Arc;
use tokio::sync::Mutex;

use futures_lite::stream::StreamExt;
use lapin::{Channel, Consumer, message::Delivery, options::BasicAckOptions, types::FieldTable};

#[derive(Debug, Clone)]
pub struct RabbitMQConsumer {
    pub consumer: Arc<Mutex<Consumer>>,
}

impl RabbitMQConsumer {
    pub async fn new(
        channel: Arc<Channel>,
        tag: &str,
        queue_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let consumer = Arc::new(Mutex::new(
            channel
                .basic_consume(
                    queue_name.into(),
                    tag.into(),
                    lapin::options::BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await?,
        ));

        Ok(Self { consumer })
    }

    pub async fn consume<F, Fut>(&self, handler: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Delivery) -> Fut + Send + 'static,
        Fut: Future<Output = Result<(), Box<dyn std::error::Error>>> + Send,
    {
        let consumer = self.consumer.clone();

        tokio::spawn(async move {
            loop {
                let mut locked_consumer = consumer.lock().await;

                let delivery_opt = locked_consumer.next().await;

                drop(locked_consumer);

                match delivery_opt {
                    Some(Ok(delivery)) => {
                        tracing::info!("Message recieved from queue");
                        let delivery = delivery;
                        delivery.ack(BasicAckOptions::default()).await.expect("ack");
                        let _ = handler(delivery).await;
                    }
                    Some(Err(error)) => {
                        tracing::error!("Failed to consume: {}", error)
                    }
                    None => {
                        tracing::warn!("Stream closed");
                        break;
                    }
                }
            }
        });

        Ok(())
    }
}
