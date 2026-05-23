pub use amqprs::channel::ExchangeType as AmqpExchangeType;

#[cfg(feature = "tracing-otel")]
use tracing::info;

use crate::error::Error;
use crate::pool::Pooled;
use amqprs::channel::{
    BasicAckArguments, BasicConsumeArguments, BasicNackArguments, Channel, ConfirmSelectArguments,
    ExchangeDeclareArguments, QueueBindArguments, QueueDeclareArguments,
};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use kanau::message::{MessageDe, MessageSer};
use kanau::processor::Processor;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;

pub type AmqpPool = crate::pool::Pool<Channel, amqprs::error::Error>;

impl AmqpPool {
    pub async fn connect(connection: amqprs::connection::Connection) -> Self {
        let factory = move || {
            let connection = connection.clone();
            Box::pin(async move { connection.open_channel(None).await })
                as Pin<Box<dyn Future<Output = Result<Channel, amqprs::error::Error>> + Send>>
        };
        crate::pool::Pool::<Channel, amqprs::error::Error>::new(Box::pin(factory), 4096)
    }
}

/// Trait for routing message to rabbitmq
pub trait AmqpRouting {
    /// Exchange name
    const EXCHANGE: &'static str;

    /// Exchange type
    const EXCHANGE_TYPE: AmqpExchangeType;

    /// Routing key
    const ROUTING_KEY: &'static str;

    // Allow async fn in trait because we don't want the user to override this function
    #[allow(async_fn_in_trait)]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, err, ret))]
    async fn ensure_exchange(pool: &AmqpPool) -> Result<(), Error> {
        let channel: Result<Pooled<Channel, _>, Error> = pool.get().await.into();
        let channel = channel?;
        let channel = channel
            .get_ref()
            .ok_or(Error::Io(anyhow::anyhow!("Channel is unexpectedly closed")))?;
        channel
            .exchange_declare(
                ExchangeDeclareArguments::of_type(Self::EXCHANGE, Self::EXCHANGE_TYPE)
                    .durable(true)
                    .finish(),
            )
            .await?;
        Ok(())
    }
}

/// Trait for sending message to rabbitmq
pub trait AmqpMessageSend: MessageSer + Send + Sized + AmqpRouting {
    // Allow async fn in trait because we don't want the user to override this function
    #[allow(async_fn_in_trait)]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, err, ret))]
    /// Send message to rabbitmq
    async fn send(self, pool: &AmqpPool) -> Result<(), crate::error::Error> {
        let bytes = self.to_bytes().map_err(|e| e.into())?;
        let channel: Result<Pooled<Channel, _>, crate::error::Error> = pool.get().await.into();
        let channel = channel?;
        let channel = channel
            .get_ref()
            .ok_or(Error::Io(anyhow::anyhow!("Channel is unexpectedly closed")))?;
        channel
            .confirm_select(ConfirmSelectArguments::new(false))
            .await?;
        channel
            .basic_publish(
                BasicProperties::default(),
                bytes.into_vec(),
                amqprs::channel::BasicPublishArguments::new(Self::EXCHANGE, Self::ROUTING_KEY)
                    .mandatory(true)
                    .finish(),
            )
            .await?;

        #[cfg(feature = "tracing-otel")]
        info!(monotonic_counter.mq_event_push = 1);
        Ok(())
    }
}

/// Trait for consuming message from rabbitmq
pub trait AmqpMessageProcessor<Message: AmqpMessageSend + MessageDe>:
    Processor<Message, Output = (), Error = crate::error::Error>
{
    const QUEUE: &'static str;

    // Allow async fn in trait because we don't want the user to override this function
    #[allow(async_fn_in_trait)]
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, err))]
    /// Ensure the topology of the queue and get the channel with the queue bound
    async fn ensure_queue(pool: &AmqpPool) -> Result<Channel, crate::error::Error> {
        // ensure exchange first
        Message::ensure_exchange(pool).await?;

        // Declare a durable, client-named queue
        let channel = pool.factory_create().await?;
        let queue_arg = QueueDeclareArguments::durable_client_named(Self::QUEUE);
        channel.queue_declare(queue_arg).await?;

        // Bind queue -> exchange with routing key
        let queue_bind_arg =
            QueueBindArguments::new(Self::QUEUE, Message::EXCHANGE, Message::ROUTING_KEY);
        channel.queue_bind(queue_bind_arg).await?;
        Ok(channel)
    }
}

/// Consumer for rabbitmq
pub struct AmqpMessageConsumer<
    Message: AmqpMessageSend + MessageDe,
    Inner: AmqpMessageProcessor<Message>,
> {
    inner: Arc<Inner>,
    _marker: PhantomData<Message>,
}

impl<Message: AmqpMessageSend + MessageDe, Inner: AmqpMessageProcessor<Message>>
    AmqpMessageConsumer<Message, Inner>
{
    /// Create a new consumer
    pub fn new(inner: Arc<Inner>) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    /// Process message
    pub async fn on_message(&self, _prop: BasicProperties, content: Vec<u8>) -> Result<(), Error> {
        #[cfg(feature = "tracing-otel")]
        info!(monotonic_counter.mq_event_receive = 1);
        let decoded_message = Message::from_bytes(&content).map_err(|e| e.into())?;
        self.inner.process(decoded_message).await
    }
}

impl<M, I> AsyncConsumer for AmqpMessageConsumer<M, I>
where
    M: AmqpMessageSend + MessageDe + Send + Sync,
    I: AmqpMessageProcessor<M> + Send + Sync,
    M::DeError: Send,
{
    fn consume<'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        channel: &'life1 Channel,
        deliver: Deliver,
        basic_properties: BasicProperties,
        content: Vec<u8>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        Self: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
    {
        Box::pin(async move {
            match self.on_message(basic_properties, content).await {
                Ok(_) => {
                    ack(
                        channel,
                        BasicAckArguments::new(deliver.delivery_tag(), false),
                        5,
                    )
                    .await;
                }
                #[cfg(feature = "sqlx")]
                Err(Error::DatabaseError(e)) => {
                    nack(
                        channel,
                        BasicNackArguments::new(deliver.delivery_tag(), false, true),
                        5,
                    )
                    .await;
                    #[cfg(feature = "tracing")]
                    tracing::error!("Database: {}", e);
                    #[cfg(not(feature = "tracing"))]
                    let _ = e;
                }
                Err(Error::SerializeError(_)) | Err(Error::DeserializeError(_)) => {
                    ack(
                        channel,
                        BasicAckArguments::new(deliver.delivery_tag(), false),
                        5,
                    )
                    .await;
                }
                #[cfg(feature = "redis")]
                Err(Error::RedisError(e)) => {
                    nack(
                        channel,
                        BasicNackArguments::new(deliver.delivery_tag(), false, true),
                        5,
                    )
                    .await;
                    #[cfg(feature = "tracing")]
                    tracing::error!("Redis: {}", e);
                    #[cfg(not(feature = "tracing"))]
                    let _ = e;
                }
                Err(Error::InvalidInput) | Err(Error::NotFound) | Err(Error::PermissionsDenied) => {
                    ack(
                        channel,
                        BasicAckArguments::new(deliver.delivery_tag(), false),
                        5,
                    )
                    .await;
                    #[cfg(feature = "tracing")]
                    tracing::error!("Invalid input in event");
                }
                Err(Error::AmqpError(e)) => {
                    #[cfg(feature = "tracing")]
                    tracing::error!("RabbitMQ: {}", e);
                    #[cfg(not(feature = "tracing"))]
                    let _ = e;
                }
                Err(Error::BusinessPanic(e)) => {
                    ack(
                        channel,
                        BasicAckArguments::new(deliver.delivery_tag(), false),
                        5,
                    )
                    .await;
                    #[cfg(feature = "tracing")]
                    tracing::error!("Business panic: {}", e);
                    #[cfg(not(feature = "tracing"))]
                    let _ = e;
                }
                Err(Error::Io(e)) => {
                    nack(
                        channel,
                        BasicNackArguments::new(deliver.delivery_tag(), false, true),
                        5,
                    )
                    .await;
                    #[cfg(feature = "tracing")]
                    tracing::error!("IO error: {}", e);
                    #[cfg(not(feature = "tracing"))]
                    let _ = e;
                }
            }
        })
    }
}

/// Ack message with retry
pub async fn ack(channel: &Channel, arg: BasicAckArguments, max_retries: u32) {
    let mut retries = 0;
    while retries < max_retries {
        match channel.basic_ack(arg.clone()).await {
            Ok(_) => return,
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to ack message: {e}");
                #[cfg(not(feature = "tracing"))]
                let _ = e;
                retries += 1;
            }
        }
    }

    if retries == max_retries {
        #[cfg(feature = "tracing")]
        tracing::error!("Failed to ack message after {} retries", max_retries);
    }
}

pub async fn nack(channel: &Channel, arg: BasicNackArguments, max_retries: u32) {
    let mut retries = 0;
    while retries < max_retries {
        match channel.basic_nack(arg.clone()).await {
            Ok(_) => return,
            Err(e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("Failed to nack message: {e}");
                #[cfg(not(feature = "tracing"))]
                let _ = e;
                retries += 1;
            }
        }
    }
}

/// bind consumer for a message type
pub async fn setup_consumer<M, H>(
    channel: &Channel,
    hook: Arc<H>,
) -> Result<(), amqprs::error::Error>
where
    M: AmqpMessageSend + MessageDe + Send + Sync + 'static,
    M::DeError: Send,
    H: AmqpMessageProcessor<M> + Send + Sync + 'static,
{
    let queue = H::QUEUE;
    channel
        .queue_bind(QueueBindArguments::new(queue, M::EXCHANGE, M::ROUTING_KEY).finish())
        .await?;
    channel
        .basic_consume(
            AmqpMessageConsumer::<M, H>::new(hook),
            BasicConsumeArguments::new(queue, "")
                .manual_ack(true)
                .finish(),
        )
        .await?;
    Ok(())
}
