use crate::impl_entity;
use crate::tuple_value_object_requiring_error_definition;
use crate::tuple_value_object_with_simple_error;

tuple_value_object_with_simple_error!(UserId, uuid::Uuid, UserIdError);
tuple_value_object_requiring_error_definition!(Email, email_address::EmailAddress, EmailError);
tuple_value_object_with_simple_error!(Password, String, PasswordError);

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("failed to parse email address: {0}")]
    ParseError(#[from] email_address::Error),
}

impl std::str::FromStr for Email {
    type Err = EmailError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(email_address::EmailAddress::from_str(s)?))
    }
}

pub struct User {
    id: UserId,
    email: Email,
}

impl_entity!(User, id, UserId);
