use std::sync::Arc;

use adapter::{
    database::ConnectionPool,
    redis::RedisClient,
    repository::{
        auth::AuthRepositoryImpl, book::BookRepositoryImpl, health::HealthCheckRepositoryImpl,
        user::UserRepositoryImpl,
    },
};
use kernel::repository::{
    auth::AuthRepository, book::BookRepository, health::HealthCheckRepository, user::UserRepository,
};
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistry {
    auth_repository: Arc<dyn AuthRepository>,
    book_repository: Arc<dyn BookRepository>,
    health_check_repository: Arc<dyn HealthCheckRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl AppRegistry {
    pub fn new(
        pool: ConnectionPool,
        redis_client: Arc<RedisClient>,
        app_config: AppConfig,
    ) -> Self {
        // 依存解決
        let auth_repository = Arc::new(AuthRepositoryImpl::new(
            pool.clone(),
            redis_client,
            app_config.auth.ttl,
        ));
        let book_repository = Arc::new(BookRepositoryImpl::new(pool.clone()));
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));

        Self {
            auth_repository,
            book_repository,
            health_check_repository,
            user_repository,
        }
    }

    // 依存解決したインスタンスを返すメソッド
    pub fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        self.auth_repository.clone()
    }

    pub fn book_repository(&self) -> Arc<dyn BookRepository> {
        self.book_repository.clone()
    }
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    pub fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }
}
