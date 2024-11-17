use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use kernel::repository::health::{HealthCheckError, HealthCheckRepository};

use crate::database::ConnectionPool;

#[derive(new)]
pub struct HealthCheckRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn check_db(&self) -> Result<(), HealthCheckError> {
        let _ = sqlx::query("SELECT 1")
            .fetch_one(self.db.inner_ref())
            .await
            .map_err(|_| HealthCheckError::DatabaseConnectionError)?;
        Ok(())
    }
}
