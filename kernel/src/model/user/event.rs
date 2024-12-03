use super::{Password, UserEmail, UserId, UserName, UserRole};

#[derive(Debug)]
pub struct CreateUser {
    pub name: UserName,
    pub email: UserEmail,
    pub password: Password,
}

#[derive(Debug)]
pub struct UpdateUserRole {
    pub user_id: UserId,
    pub role: UserRole,
}

#[derive(Debug)]
pub struct UpdateUserPassword {
    pub user_id: UserId,
    pub current_password: Password,
    pub new_password: Password,
}

#[derive(Debug)]
pub struct DeleteUser {
    pub user_id: UserId,
}
