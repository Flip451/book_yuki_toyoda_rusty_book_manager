use derive_new::new;
use garde::Validate;
use kernel::model::{
    user::{
        event::{CreateUser, UpdateUserPassword, UpdateUserRole},
        PasswordError, User, UserEmailError, UserId, UserNameError, UserRole,
    },
    value_object::ValueObject,
};
use serde::{Deserialize, Serialize};
use strum::VariantNames;
use uuid::Uuid;

#[derive(Serialize, VariantNames, Deserialize)]
#[strum(serialize_all = "kebab-case")]
pub enum UserRoleName {
    Admin,
    User,
}

impl From<UserRole> for UserRoleName {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Admin => UserRoleName::Admin,
            UserRole::User => UserRoleName::User,
        }
    }
}

impl From<UserRoleName> for UserRole {
    fn from(value: UserRoleName) -> Self {
        match value {
            UserRoleName::Admin => UserRole::Admin,
            UserRoleName::User => UserRole::User,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookOwner {
    pub id: Uuid,
    pub name: String,
}

impl From<kernel::model::user::BookOwner> for BookOwner {
    fn from(value: kernel::model::user::BookOwner) -> Self {
        Self {
            id: value.user_id.into_inner(),
            name: value.user_name.into_inner(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersResponse {
    pub items: Vec<UserResponse>,
}

impl From<Vec<User>> for UsersResponse {
    fn from(value: Vec<User>) -> Self {
        Self {
            items: value.into_iter().map(UserResponse::from).collect(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub role: UserRoleName,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        let (user_id, user_name, role, email) = user.dissolve();

        Self {
            id: user_id.into_inner(),
            name: user_name.into_inner(),
            email: email.to_string(),
            role: role.into(),
        }
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserPasswordRequest {
    #[garde(length(min = 8))]
    pub current_password: String,
    #[garde(length(min = 8))]
    pub new_password: String,
}

#[derive(new)]
pub struct UpdateUserPasswordRequestWithUserId(UserId, UpdateUserPasswordRequest);

impl TryFrom<UpdateUserPasswordRequestWithUserId> for UpdateUserPassword {
    type Error = UserModelError;

    fn try_from(value: UpdateUserPasswordRequestWithUserId) -> Result<Self, Self::Error> {
        let UpdateUserPasswordRequestWithUserId(user_id, request) = value;
        let UpdateUserPasswordRequest {
            current_password,
            new_password,
        } = request;

        Ok(UpdateUserPassword {
            user_id,
            current_password: current_password.try_into()?,
            new_password: new_password.try_into()?,
        })
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    #[garde(length(min = 1))]
    pub name: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8))]
    pub password: String,
}

impl TryFrom<CreateUserRequest> for CreateUser {
    type Error = UserModelError;

    fn try_from(value: CreateUserRequest) -> Result<Self, Self::Error> {
        let CreateUserRequest {
            name,
            email,
            password,
        } = value;

        Ok(CreateUser {
            name: name.try_into()?,
            email: email.parse()?,
            password: password.try_into()?,
        })
    }
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRoleRequest {
    #[garde(skip)]
    pub role: UserRoleName,
}

#[derive(new)]
pub struct UpdateUserRoleRequestWithUserId(UserId, UpdateUserRoleRequest);

impl From<UpdateUserRoleRequestWithUserId> for UpdateUserRole {
    fn from(value: UpdateUserRoleRequestWithUserId) -> Self {
        let UpdateUserRoleRequestWithUserId(user_id, request) = value;
        let UpdateUserRoleRequest { role } = request;

        UpdateUserRole {
            user_id,
            role: role.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserModelError {
    #[error("Invalid name: {0}")]
    InvalidName(#[from] UserNameError),

    #[error("Invalid current password: {0}")]
    InvalidCurrentPassword(#[from] PasswordError),

    #[error("Invalid email: {0}")]
    InvalidEmail(#[from] UserEmailError),
}
