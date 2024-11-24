use kernel::model::{
    auth::{event::CreateToken, AccessToken},
    user::UserId,
    value_object::ValueObject,
};

use crate::redis::model::{RedisKey, RedisValue, RedisValueError};

pub struct UserRow {
    pub user_id: uuid::Uuid,
    pub password_hash: String,
}

pub struct AuthorizationKey(AccessToken);
pub struct AuthorizedUserId(UserId);

pub fn from(event: CreateToken) -> (AuthorizationKey, AuthorizedUserId) {
    (
        AuthorizationKey::from(event.access_token),
        AuthorizedUserId::from(event.user_id),
    )
}

impl From<AuthorizationKey> for AccessToken {
    fn from(authorization_key: AuthorizationKey) -> Self {
        authorization_key.0
    }
}

impl From<AccessToken> for AuthorizationKey {
    fn from(access_token: AccessToken) -> Self {
        Self(access_token)
    }
}

impl From<&AccessToken> for AuthorizationKey {
    fn from(access_token: &AccessToken) -> Self {
        Self(access_token.clone())
    }
}

impl From<UserId> for AuthorizedUserId {
    fn from(user_id: UserId) -> Self {
        Self(user_id)
    }
}

impl RedisKey for AuthorizationKey {
    type Value = AuthorizedUserId;

    fn inner(&self) -> String {
        self.0.clone().into_inner()
    }
}

impl TryFrom<String> for AuthorizedUserId {
    type Error = RedisValueError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ok(Self(
            UserId::try_from(
                s.parse::<uuid::Uuid>()
                    .map_err(|e| RedisValueError::ParsingError(Box::new(e)))?,
            )
            .map_err(|e| RedisValueError::ParsingError(Box::new(e)))?,
        ))
    }
}

impl RedisValue for AuthorizedUserId {
    fn inner(&self) -> String {
        self.0.inner_ref().to_string()
    }
}

impl AuthorizedUserId {
    pub fn into_inner(self) -> UserId {
        self.0
    }
}
