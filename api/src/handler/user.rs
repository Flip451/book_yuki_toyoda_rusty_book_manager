use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use garde::Validate;
use kernel::{
    model::user::{event::DeleteUser, UserIdError},
    repository::{checkout::CheckoutRepositoryError, user::UserRepositoryError},
};
use registry::AppRegistry;
use uuid::Uuid;

use crate::{
    extractor::AuthorizedUser,
    model::{
        checkout::CheckoutsResponse,
        user::{
            CreateUserRequest, UpdateUserPasswordRequest, UpdateUserPasswordRequestWithUserId,
            UpdateUserRoleRequest, UpdateUserRoleRequestWithUserId, UserModelError, UserResponse,
            UsersResponse,
        },
    },
};

// 管理者がユーザーを登録する
#[axum::debug_handler]
pub(crate) async fn register_user(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, UserHandlerError> {
    if !user.is_admin() {
        return Err(UserHandlerError::Forbidden);
    }

    req.validate()?;

    let registered_user = registry.user_repository().create(req.try_into()?).await?;

    Ok(Json(registered_user.into()))
}

// ユーザー一覧を取得する
pub(crate) async fn list_users(
    _user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> Result<Json<UsersResponse>, UserHandlerError> {
    Ok(Json(registry.user_repository().find_all().await?.into()))
}

// 管理者がユーザーを削除する
pub(crate) async fn delete_user(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode, UserHandlerError> {
    if !user.is_admin() {
        return Err(UserHandlerError::Forbidden);
    }

    let delete_user = DeleteUser {
        user_id: user_id.try_into()?,
    };

    registry.user_repository().delete(delete_user).await?;

    Ok(StatusCode::NO_CONTENT)
}

// 管理者がユーザーの権限を変更する
pub(crate) async fn change_user_role(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> Result<StatusCode, UserHandlerError> {
    if !user.is_admin() {
        return Err(UserHandlerError::Forbidden);
    }

    req.validate()?;

    let update_user_role = UpdateUserRoleRequestWithUserId::new(user_id.try_into()?, req);

    registry
        .user_repository()
        .update_role(update_user_role.into())
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

// ユーザーが自身の情報を取得する
pub(crate) async fn get_current_user(
    user: AuthorizedUser,
) -> Result<Json<UserResponse>, UserHandlerError> {
    Ok(Json(user.user.into()))
}

// ユーザーが自身のパスワードを変更する
pub(crate) async fn change_password(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<UpdateUserPasswordRequest>,
) -> Result<StatusCode, UserHandlerError> {
    req.validate()?;

    let update_user_password =
        UpdateUserPasswordRequestWithUserId::new(user.user_id().clone(), req);

    registry
        .user_repository()
        .update_password(update_user_password.try_into()?)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn get_checkouts(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> Result<Json<CheckoutsResponse>, UserHandlerError> {
    let checkouts = registry
        .checkout_repository()
        .find_unreturned_by_user_id(user.user_id())
        .await?;
    Ok(Json(checkouts.into()))
}

#[derive(Debug, thiserror::Error)]
pub enum UserHandlerError {
    #[error("forbidden")]
    Forbidden,

    #[error("invalid user id: {0}")]
    InvalidUserId(#[from] UserIdError),

    #[error("validation error: {0}")]
    ValidationError(#[from] garde::Report),

    #[error("model error: {0}")]
    ModelError(#[from] UserModelError),

    #[error("repository error: {0}")]
    UserRepositoryError(#[from] UserRepositoryError),

    #[error("checkout repository error: {0}")]
    CheckoutRepositoryError(#[from] CheckoutRepositoryError),
}

impl IntoResponse for UserHandlerError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            UserHandlerError::Forbidden => StatusCode::FORBIDDEN,
            UserHandlerError::InvalidUserId(_) => StatusCode::BAD_REQUEST,
            UserHandlerError::ValidationError(_) => StatusCode::BAD_REQUEST,
            UserHandlerError::ModelError(_) => StatusCode::BAD_REQUEST,
            UserHandlerError::UserRepositoryError(e) => {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "unexpected error happened"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
            UserHandlerError::CheckoutRepositoryError(e) => {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "unexpected error happened"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        status_code.into_response()
    }
}
