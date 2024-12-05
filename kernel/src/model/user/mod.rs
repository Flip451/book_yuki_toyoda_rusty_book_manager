pub mod event;

use std::fmt::Display;

use derive_getters::Dissolve;
use derive_getters::Getters;
use thiserror::Error;

use crate::enum_value_object_with_simple_error;
use crate::impl_entity;
use crate::tuple_value_object_requiring_error_definition;
use crate::tuple_value_object_with_simple_error;

tuple_value_object_with_simple_error!(UserId, uuid::Uuid, UserIdError);
tuple_value_object_with_simple_error!(UserName, String, UserNameError);
tuple_value_object_requiring_error_definition!(
    UserEmail,
    email_address::EmailAddress,
    UserEmailError
);
tuple_value_object_with_simple_error!(Password, String, PasswordError);

enum_value_object_with_simple_error!(
    #[derive(Default)]
    UserRole {
        Admin,
        #[default]
        User,
    },
    UserRoleError
);

#[derive(Debug, thiserror::Error)]
pub enum UserEmailError {
    #[error("failed to parse email address: {0}")]
    ParseError(#[from] email_address::Error),
}

impl std::str::FromStr for UserEmail {
    type Err = UserEmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(email_address::EmailAddress::from_str(s)?))
    }
}

impl Display for UserEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Getters, Debug, Clone, derive_new::new, Dissolve)]
pub struct User {
    user_id: UserId,
    user_name: UserName,
    role: UserRole,
    email: UserEmail,
}

impl_entity!(User, user_id, UserId);

#[derive(Debug, Error)]
pub enum UserError {
    #[error("invalid user id: {0}")]
    InvalidUserId(#[from] UserIdError),

    #[error("invalid user name: {0}")]
    InvalidUserName(#[from] UserNameError),

    #[error("invalid user role: {0}")]
    InvalidUserRole(#[from] UserRoleError),

    #[error("invalid user email: {0}")]
    InvalidUserEmail(#[from] UserEmailError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookOwner(User);
impl BookOwner {
    pub fn user_id(&self) -> &UserId {
        &self.0.user_id
    }

    pub fn user_name(&self) -> &UserName {
        &self.0.user_name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckoutUser(User);
impl CheckoutUser {
    pub fn user_id(&self) -> &UserId {
        &self.0.user_id
    }

    pub fn user_name(&self) -> &UserName {
        &self.0.user_name
    }
}
