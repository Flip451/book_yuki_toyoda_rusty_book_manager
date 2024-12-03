use async_trait::async_trait;
use thiserror::Error;

use crate::model::user::{
    event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole},
    User, UserId,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_current_user(&self, user_id: &UserId) -> UserRepositoryResult<Option<User>>;
    async fn find_all(&self) -> UserRepositoryResult<Vec<User>>;
    async fn create(&self, event: CreateUser) -> UserRepositoryResult<User>;
    async fn update_password(&self, event: UpdateUserPassword) -> UserRepositoryResult<()>;
    async fn update_role(&self, event: UpdateUserRole) -> UserRepositoryResult<()>;
    async fn delete(&self, event: DeleteUser) -> UserRepositoryResult<()>;
}

#[derive(Debug, Error)]
pub enum UserRepositoryError {
    #[error("saved entity is invalid")]
    InvalidSavedEntity(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("unexpected error occurred")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub type UserRepositoryResult<T> = Result<T, UserRepositoryError>;