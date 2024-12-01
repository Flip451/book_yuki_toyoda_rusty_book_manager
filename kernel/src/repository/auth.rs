use async_trait::async_trait;
use thiserror::Error;

use crate::model::{
    auth::{event::CreateToken, AccessToken, AccessTokenError},
    user::{Password, UserEmail, UserId, UserIdError},
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn fetch_user_id_from_token(
        &self,
        access_token: &AccessToken,
    ) -> AuthRepositoryResult<Option<UserId>>;

    async fn verify_user(
        &self,
        email: &UserEmail,
        password: &Password,
    ) -> AuthRepositoryResult<UserId>;

    async fn create_token(&self, event: CreateToken) -> AuthRepositoryResult<AccessToken>;

    async fn delete_token(&self, access_token: &AccessToken) -> AuthRepositoryResult<()>;
}

#[derive(Debug, Error)]
pub enum AuthRepositoryError {
    #[error("invalid user_id has been saved: {0}")]
    InvalidUserId(#[from] UserIdError),

    #[error("invalid access_token has been saved: {0}")]
    InvalidAccessToken(#[from] AccessTokenError),

    #[error("invalid password")]
    InvalidPassword,

    #[error("unexpected error occurred: {0}")]
    Unexpected(#[source] Box<dyn std::error::Error + Send + Sync>),
}

pub type AuthRepositoryResult<T> = Result<T, AuthRepositoryError>;
