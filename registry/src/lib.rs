use std::sync::Arc;

use adapter::{
    database::ConnectionPool,
    redis::RedisClient,
    repository::{
        auth::AuthRepositoryImpl, book::BookRepositoryImpl, checkout::CheckoutRepositoryImpl,
        health::HealthCheckRepositoryImpl, user::UserRepositoryImpl,
    },
};
use kernel::repository::{
    auth::AuthRepository, book::BookRepository, checkout::CheckoutRepository,
    health::HealthCheckRepository, user::UserRepository,
};
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistryImpl {
    auth_repository: Arc<dyn AuthRepository>,
    book_repository: Arc<dyn BookRepository>,
    checkout_repository: Arc<dyn CheckoutRepository>,
    health_check_repository: Arc<dyn HealthCheckRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl AppRegistryImpl {
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
        let checkout_repository = Arc::new(CheckoutRepositoryImpl::new(pool.clone()));
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let user_repository = Arc::new(UserRepositoryImpl::new(pool.clone()));

        Self {
            auth_repository,
            book_repository,
            checkout_repository,
            health_check_repository,
            user_repository,
        }
    }
}

#[mockall::automock]
pub trait AppRegistryExt {
    fn auth_repository(&self) -> Arc<dyn AuthRepository>;
    fn book_repository(&self) -> Arc<dyn BookRepository>;
    fn checkout_repository(&self) -> Arc<dyn CheckoutRepository>;
    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository>;
    fn user_repository(&self) -> Arc<dyn UserRepository>;
}

impl AppRegistryExt for AppRegistryImpl {
    // 依存解決したインスタンスを返すメソッド
    fn auth_repository(&self) -> Arc<dyn AuthRepository> {
        self.auth_repository.clone()
    }

    fn book_repository(&self) -> Arc<dyn BookRepository> {
        self.book_repository.clone()
    }

    fn checkout_repository(&self) -> Arc<dyn CheckoutRepository> {
        self.checkout_repository.clone()
    }

    fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    fn user_repository(&self) -> Arc<dyn UserRepository> {
        self.user_repository.clone()
    }
}

pub type AppRegistry = Arc<dyn AppRegistryExt + Send + Sync + 'static>;
