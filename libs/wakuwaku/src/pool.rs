use crossbeam_queue::ArrayQueue;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

type ConnectionFactoryFut<T, E = anyhow::Error> =
    Pin<Box<dyn Future<Output = Result<T, E>> + Send>>;
type ConnectionFactory<T, E> = Pin<Box<dyn Fn() -> ConnectionFactoryFut<T, E> + Send + Sync>>;

struct PoolInner<T, FactoryError = anyhow::Error> {
    idle: ArrayQueue<T>,
    sem: Arc<Semaphore>,
    factory: ConnectionFactory<T, FactoryError>,
}

pub struct Pool<T, FactoryError = anyhow::Error> {
    inner: Arc<PoolInner<T, FactoryError>>,
}

impl<T, FE> Clone for Pool<T, FE> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

pub struct Pooled<T, FactoryError = anyhow::Error> {
    inner: Arc<PoolInner<T, FactoryError>>,
    permit: Option<OwnedSemaphorePermit>,
    conn: Option<T>,
}

impl<T, FE> Pooled<T, FE> {
    pub fn get_ref(&self) -> Option<&T> {
        self.conn.as_ref()
    }
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.conn.as_mut()
    }

    /// Mark the connection is disconnected.
    ///
    /// It will drop the connection. The connection capacity will also be released.
    pub fn disconnect(&mut self) {
        self.conn.take();
        self.permit.take();
    }
}

impl<T, FE> Drop for Pooled<T, FE> {
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            let _ = self.inner.idle.push(conn);
        }
        let _ = self.permit.take();
    }
}

impl<T, FE> Pool<T, FE> {
    pub fn new<F>(factory: Pin<Box<F>>, capacity: usize) -> Self
    where
        F: Fn() -> ConnectionFactoryFut<T, FE> + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(PoolInner {
                idle: ArrayQueue::new(capacity),
                sem: Arc::new(Semaphore::new(capacity)),
                factory,
            }),
        }
    }
    pub fn idle_len(&self) -> usize {
        self.inner.idle.len()
    }
    pub async fn factory_create(&self) -> Result<T, FE> {
        (self.inner.factory)().await
    }
    pub async fn get(&self) -> PoolingResult<T, FE> {
        let sem = self.inner.sem.clone();
        let Ok(permit) = sem.acquire_owned().await else {
            return PoolingResult::SemanticsError;
        };
        let connection = self.inner.idle.pop();

        if let Some(conn) = connection {
            PoolingResult::Ok(Pooled {
                inner: self.inner.clone(),
                permit: Some(permit),
                conn: Some(conn),
            })
        } else {
            let new = self.factory_create().await;
            match new {
                Ok(conn) => PoolingResult::Ok(Pooled {
                    inner: self.inner.clone(),
                    permit: Some(permit),
                    conn: Some(conn),
                }),
                Err(err) => PoolingResult::FactoryErr(err),
            }
        }
    }
}

pub enum PoolingResult<T, FE> {
    Ok(Pooled<T, FE>),
    SemanticsError,
    FactoryErr(FE),
}

impl<T, FE: Into<crate::Error>> From<PoolingResult<T, FE>> for Result<Pooled<T, FE>, crate::Error> {
    fn from(result: PoolingResult<T, FE>) -> Self {
        match result {
            PoolingResult::Ok(succ) => Ok(succ),
            PoolingResult::FactoryErr(err) => Err(err.into()),
            PoolingResult::SemanticsError => Err(crate::Error::BusinessPanic(anyhow::anyhow!(
                "Semaphore error"
            ))),
        }
    }
}
