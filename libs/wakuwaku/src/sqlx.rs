#[cfg(feature = "tracing-otel")]
use tracing::info;

#[derive(Debug, Clone)]
pub struct DatabaseProcessor {
    executor: sqlx::PgPool,
}

impl DatabaseProcessor {
    pub fn new(executor: sqlx::PgPool) -> Self {
        Self { executor }
    }
    pub fn db(&self) -> &sqlx::PgPool {
        #[cfg(feature = "tracing-otel")]
        info!(monotonic_counter.sql = 1);
        &self.executor
    }
    pub fn from_pool(pool: sqlx::PgPool) -> Self {
        Self::new(pool)
    }
}
