use kernel::model::user::{User, UserEmail, UserEmailError, UserIdError, UserNameError, UserRole};
use sqlx::types::chrono::{DateTime, Utc};
use strum::{Display, EnumString};
use thiserror::Error;
use uuid::Uuid;

pub struct UserRow {
    pub user_id: Uuid,
    pub user_name: String,
    pub user_role_name: String,
    pub user_email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, EnumString, Display)]
pub enum UserRoleName {
    Admin,
    User,
}

impl From<UserRoleName> for UserRole {
    fn from(value: UserRoleName) -> Self {
        match value {
            UserRoleName::Admin => UserRole::Admin,
            UserRoleName::User => UserRole::User,
        }
    }
}

impl From<UserRole> for UserRoleName {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Admin => UserRoleName::Admin,
            UserRole::User => UserRoleName::User,
        }
    }
}

#[derive(Debug, Error)]
pub enum UserRowError {
    #[error("saved user role is invalid: {0}")]
    InvalidUserRole(#[from] strum::ParseError),

    #[error("saved user id is invalid: {0}")]
    InvalidUserId(#[from] UserIdError),

    #[error("saved user name is invalid: {0}")]
    InvalidUserName(#[from] UserNameError),

    #[error("saved user email is invalid: {0}")]
    InvalidUserEmail(#[from] UserEmailError),
}

impl TryFrom<UserRow> for User {
    type Error = UserRowError;

    fn try_from(
        UserRow {
            user_id,
            user_name,
            user_role_name,
            user_email,
            ..
        }: UserRow,
    ) -> Result<Self, Self::Error> {
        let user_role = user_role_name.parse::<UserRoleName>()?;

        Ok(User::new(
            user_id.try_into()?,
            user_name.try_into()?,
            user_role.into(),
            user_email.parse::<UserEmail>()?,
        ))
    }
}
