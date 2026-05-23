use kanau::message::{MessageDe, MessageSer};
use redis::AsyncCommands;
#[cfg(feature = "tracing")]
use tracing::instrument;

/// Type alias for redis multiplexed connection.
pub type RedisConnection = redis::aio::MultiplexedConnection;

/// Redis key wrapper used by [`KeyValue`] trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedisKey(pub Box<[u8]>);

impl From<String> for RedisKey {
    fn from(v: String) -> Self {
        Self(v.into_bytes().into_boxed_slice())
    }
}

impl From<&str> for RedisKey {
    fn from(v: &str) -> Self {
        Self(v.as_bytes().to_vec().into_boxed_slice())
    }
}

impl From<Vec<u8>> for RedisKey {
    fn from(v: Vec<u8>) -> Self {
        Self(v.into_boxed_slice())
    }
}

impl From<&[u8]> for RedisKey {
    fn from(v: &[u8]) -> Self {
        Self(v.to_vec().into_boxed_slice())
    }
}

impl From<[u8; 16]> for RedisKey {
    fn from(v: [u8; 16]) -> Self {
        Self(v.to_vec().into_boxed_slice())
    }
}

#[cfg(feature = "uuid")]
impl From<uuid::Uuid> for RedisKey {
    fn from(v: uuid::Uuid) -> Self {
        Self(v.as_bytes().to_vec().into_boxed_slice())
    }
}

impl redis::ToSingleRedisArg for RedisKey {}

impl redis::ToRedisArgs for RedisKey {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        self.0.as_ref().write_redis_args(out)
    }
}

/// Abstraction for key-value pairs stored in redis.
#[allow(unused)]
pub trait KeyValue: Sized + Send + Sync {
    /// Key type.
    type Key: redis::ToSingleRedisArg + Send + Sync + Sized;
    /// Value type.
    type Value: Send + Sync + Sized;

    /// Get key from the pair.
    fn key(&self) -> Self::Key;
    /// Get value by cloning.
    fn value(&self) -> Self::Value;
    /// Consume pair and return value.
    fn into_value(self) -> Self::Value;
    /// Create new pair from key and value.
    fn new(key: Self::Key, value: Self::Value) -> Self;

    /// Delete value by key.
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(value_type = std::any::type_name::<Self>())))]
    fn delete(
        conn: &mut RedisConnection,
        key: Self::Key,
    ) -> impl Future<Output = Result<(), crate::error::Error>> + Send {
        async {
            let _: () = conn.del(key).await?;
            Ok(())
        }
    }
}

/// Helper trait for reading value from redis.
pub trait KeyValueRead: KeyValue
where
    Self::Value: MessageDe,
{
    /// Read value by key.
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(value_type = std::any::type_name::<Self>())))]
    fn read(
        conn: &mut RedisConnection,
        key: Self::Key,
    ) -> impl Future<Output = Result<Option<Self::Value>, crate::error::Error>> + Send {
        async {
            let data: Option<Vec<u8>> = conn.get(key).await?;
            if let Some(bytes) = data {
                let val = <Self::Value as MessageDe>::from_bytes(&bytes)
                    .map_err(|e| crate::error::Error::DeserializeError(e.into()))?;
                Ok(Some(val))
            } else {
                Ok(None)
            }
        }
    }
}

#[allow(unused)]
/// Helper trait for writing value to redis.
pub trait KeyValueWrite: KeyValue + Send
where
    Self::Value: MessageSer,
    Self::Key: Send,
    Self::Value: Send,
{
    /// Write current pair into redis.
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(value_type = std::any::type_name::<Self>())))]
    fn write(
        &self,
        conn: &mut RedisConnection,
    ) -> impl Future<Output = Result<(), crate::error::Error>> + Send {
        async { Self::write_kv(conn, self.key(), self.value()).await }
    }

    /// Write provided key and value into redis.
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(value_type = std::any::type_name::<Self>())))]
    fn write_kv(
        conn: &mut RedisConnection,
        key: Self::Key,
        value: Self::Value,
    ) -> impl Future<Output = Result<(), crate::error::Error>> + Send {
        async {
            let bytes = MessageSer::to_bytes(value)
                .map_err(|e| crate::error::Error::SerializeError(e.into()))?;
            let _: () = conn.set(key, bytes.as_ref()).await?;
            Ok(())
        }
    }

    /// Write current pair into redis with TTL.
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(value_type = std::any::type_name::<Self>())))]
    fn write_with_ttl(
        &self,
        conn: &mut RedisConnection,
        ttl: std::time::Duration,
    ) -> impl Future<Output = Result<(), crate::error::Error>> + Send {
        async move { Self::write_kv_with_ttl(conn, self.key(), self.value(), ttl).await }
    }

    /// Write provided key and value into redis with TTL.
    #[cfg_attr(feature = "tracing", instrument(skip_all, fields(value_type = std::any::type_name::<Self>())))]
    fn write_kv_with_ttl(
        conn: &mut RedisConnection,
        key: Self::Key,
        value: Self::Value,
        ttl: std::time::Duration,
    ) -> impl Future<Output = Result<(), crate::error::Error>> + Send {
        async move {
            let bytes = MessageSer::to_bytes(value)
                .map_err(|e| crate::error::Error::SerializeError(e.into()))?;
            let _: () = conn.set_ex(key, bytes.as_ref(), ttl.as_secs()).await?;
            Ok(())
        }
    }
}
