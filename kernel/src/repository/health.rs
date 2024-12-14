use anyhow::Result;
use async_trait::async_trait;
use thiserror::Error;

#[mockall::automock]
#[async_trait]
pub trait HealthCheckRepository: Send + Sync {
    async fn check_db(&self) -> Result<(), HealthCheckError>;
}

#[derive(Debug, Error)]
pub enum HealthCheckError {
    #[error("database connection error")]
    DatabaseConnectionError,
}
