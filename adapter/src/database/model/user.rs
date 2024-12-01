use kernel::model::user::{User, UserEmail, UserEmailError, UserIdError, UserNameError, UserRole};
use sqlx::types::chrono::{DateTime, Utc};
use strum::EnumString;
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

#[derive(Debug, EnumString)]
pub enum UserRoleName {
    Admin,
    User,
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

        let user_role = match user_role {
            UserRoleName::Admin => UserRole::Admin,
            UserRoleName::User => UserRole::User,
        };

        Ok(User::new(
            user_id.try_into()?,
            user_name.try_into()?,
            user_role,
            user_email.parse::<UserEmail>()?,
        ))
    }
}
