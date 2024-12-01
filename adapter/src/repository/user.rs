use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        user::{
            event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole},
            User, UserId,
        },
        value_object::ValueObject,
    },
    repository::user::{UserRepository, UserRepositoryError, UserRepositoryResult},
};

use crate::database::{model::user::UserRow, ConnectionPool};

#[derive(new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_current_user(&self, user_id: &UserId) -> UserRepositoryResult<Option<User>> {
        let user_row = sqlx::query_as!(
            UserRow,
            r#"
                SELECT
                    u.user_id as user_id,
                    u.name as user_name,
                    u.email as user_email,
                    r.name as user_role_name,
                    u.created_at as created_at,
                    u.updated_at as updated_at
                FROM users u
                INNER JOIN roles r ON u.role_id = r.role_id
                WHERE user_id = $1
            "#,
            user_id.inner_ref()
        )
        .fetch_optional(self.db.inner_ref())
        .await
        .map_err(|e| UserRepositoryError::Unexpected(e.into()))?;

        user_row
            .map(User::try_from)
            .transpose()
            .map_err(|e| UserRepositoryError::InvalidSavedEntity(e.into()))
    }

    async fn find_all(&self) -> UserRepositoryResult<Vec<User>> {
        todo!()
    }

    async fn create(&self, _event: CreateUser) -> UserRepositoryResult<User> {
        todo!()
    }

    async fn update_password(&self, _event: UpdateUserPassword) -> UserRepositoryResult<()> {
        todo!()
    }

    async fn update_role(&self, _event: UpdateUserRole) -> UserRepositoryResult<()> {
        todo!()
    }

    async fn delete(&self, _event: DeleteUser) -> UserRepositoryResult<()> {
        todo!()
    }
}
