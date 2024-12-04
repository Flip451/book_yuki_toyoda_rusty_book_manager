use async_trait::async_trait;
use derive_new::new;
use kernel::{
    model::{
        user::{
            event::{CreateUser, DeleteUser, UpdateUserPassword, UpdateUserRole},
            Password, User, UserId, UserIdError, UserRole,
        },
        value_object::ValueObject,
    },
    repository::user::{UserRepository, UserRepositoryError, UserRepositoryResult},
};

use crate::database::{
    model::user::{UserRoleName, UserRow},
    ConnectionPool,
};

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
                WHERE user_id = $1;
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
        let users = sqlx::query_as!(
            UserRow,
            r#"
                SELECT
                    u.user_id as user_id,
                    u.name as user_name,
                    r.name as user_role_name,
                    u.email as user_email,
                    u.created_at as created_at,
                    u.updated_at as updated_at
                FROM users u
                INNER JOIN roles r ON u.role_id = r.role_id
                ORDER BY u.created_at DESC;
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await
        .map_err(|e| UserRepositoryError::Unexpected(e.into()))?;

        users
            .into_iter()
            .map(User::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| UserRepositoryError::InvalidSavedEntity(e.into()))
    }

    async fn create(&self, event: CreateUser) -> UserRepositoryResult<User> {
        let role: UserRoleName = UserRole::User.into();
        let hashed_password = hash_password(&event.password)
            .map_err(|e| UserRepositoryError::PasswordHash(e.into()))?;

        let user_id = sqlx::query_scalar!(
            r#"
                INSERT INTO users (name, email, password_hash, role_id)
                SELECT $1, $2, $3, r.role_id
                FROM roles r
                WHERE r.name = $4
                RETURNING user_id;
            "#,
            event.name.inner_ref(),
            event.email.to_string(),
            hashed_password,
            role.to_string()
        )
        .fetch_one(self.db.inner_ref())
        .await
        .map_err(|e| UserRepositoryError::Unexpected(e.into()))?;

        let user_id = user_id
            .try_into()
            .map_err(|e: UserIdError| UserRepositoryError::InvalidSavedEntity(e.into()))?;

        Ok(User::new(user_id, event.name, role.into(), event.email))
    }

    async fn update_password(&self, event: UpdateUserPassword) -> UserRepositoryResult<()> {
        let mut tx = self
            .db
            .begin()
            .await
            .map_err(|e| UserRepositoryError::Transaction(e.into()))?;

        let original_password_hash = sqlx::query!(
            r#"
                SELECT password_hash FROM users WHERE user_id = $1;
            "#,
            event.user_id.inner_ref()
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| UserRepositoryError::Unexpected(e.into()))?
        .password_hash;

        verify_password(&event.current_password, &original_password_hash)?;

        let new_password_hash = hash_password(&event.new_password)
            .map_err(|e| UserRepositoryError::PasswordHash(e.into()))?;

        sqlx::query!(
            r#"
                UPDATE users SET password_hash = $1 WHERE user_id = $2;
            "#,
            new_password_hash,
            event.user_id.inner_ref()
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| UserRepositoryError::Unexpected(e.into()))?;

        tx.commit()
            .await
            .map_err(|e| UserRepositoryError::Transaction(e.into()))
    }

    async fn update_role(&self, event: UpdateUserRole) -> UserRepositoryResult<()> {
        let role: UserRoleName = event.role.into();

        let res = sqlx::query!(
            r#"
                UPDATE users SET role_id = (SELECT role_id FROM roles WHERE name = $1) WHERE user_id = $2;
            "#,
            role.to_string(),
            event.user_id.inner_ref()
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| UserRepositoryError::Unexpected(e.into()))?;

        if res.rows_affected() < 1 {
            return Err(UserRepositoryError::NotFound);
        }

        Ok(())
    }

    async fn delete(&self, event: DeleteUser) -> UserRepositoryResult<()> {
        let res = sqlx::query!(
            r#"
                DELETE FROM users WHERE user_id = $1;
            "#,
            event.user_id.inner_ref()
        )
        .execute(self.db.inner_ref())
        .await
        .map_err(|e| UserRepositoryError::Unexpected(e.into()))?;

        if res.rows_affected() < 1 {
            return Err(UserRepositoryError::NotFound);
        }

        Ok(())
    }
}

fn hash_password(password: &Password) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password.inner_ref(), bcrypt::DEFAULT_COST)
}

fn verify_password(password: &Password, hash: &str) -> Result<(), UserRepositoryError> {
    let valid = bcrypt::verify(password.inner_ref(), hash)
        .map_err(|e| UserRepositoryError::PasswordHash(e.into()))?;

    if !valid {
        return Err(UserRepositoryError::InvalidPassword);
    }

    Ok(())
}
