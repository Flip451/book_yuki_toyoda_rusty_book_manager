use std::sync::Arc;

use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        auth::{event::CreateToken, AccessToken},
        user::{Password, UserEmail, UserId},
        value_object::ValueObject,
    },
    repository::auth::{AuthRepository, AuthRepositoryError, AuthRepositoryResult},
};

use crate::{
    database::{
        model::auth::{AuthorizationKey, UserRow},
        ConnectionPool,
    },
    redis::RedisClient,
};

#[derive(new)]
pub struct AuthRepositoryImpl {
    db: ConnectionPool,
    kvs: Arc<RedisClient>,
    ttl: u64,
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn fetch_user_id_from_token(
        &self,
        access_token: &AccessToken,
    ) -> AuthRepositoryResult<Option<UserId>> {
        let key: AuthorizationKey = access_token.into();
        self.kvs
            .get(&key)
            .await
            .map(|x| x.map(|y| y.into_inner()))
            .map_err(|e| AuthRepositoryError::Unexpected(Box::new(e)))
    }

    async fn verify_user(
        &self,
        email: &UserEmail,
        password: &Password,
    ) -> AuthRepositoryResult<UserId> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"SELECT user_id, password_hash FROM users WHERE email = $1"#,
            email.inner_ref().to_string()
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(|e| AuthRepositoryError::Unexpected(Box::new(e)))?
        .ok_or(AuthRepositoryError::InvalidPassword)?;

        let valid = bcrypt::verify(password.inner_ref(), &user_row.password_hash)
            .map_err(|e| AuthRepositoryError::Unexpected(Box::new(e)))?;

        if !valid {
            return Err(AuthRepositoryError::InvalidPassword);
        }

        Ok(user_row.user_id.try_into()?)
    }

    async fn create_token(&self, event: CreateToken) -> AuthRepositoryResult<AccessToken> {
        let (authorization_key, authorized_user_id) = crate::database::model::auth::from(event);

        self.kvs
            .set_ex(&authorization_key, &authorized_user_id, self.ttl)
            .await
            .map_err(|e| AuthRepositoryError::Unexpected(Box::new(e)))?;
        Ok(authorization_key.into())
    }

    async fn delete_token(&self, access_token: &AccessToken) -> AuthRepositoryResult<()> {
        let key: AuthorizationKey = access_token.into();
        self.kvs
            .delete(&key)
            .await
            .map_err(|e| AuthRepositoryError::Unexpected(Box::new(e)))
    }
}
