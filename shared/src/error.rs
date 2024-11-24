use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    UnprocessableEntity(String),
    #[error("{0}")]
    EntityNotFound(String),
    #[error("{0}")]
    ValidationError(#[from] garde::Report),
    #[error("トランザクションを実行できませんでした。")]
    TransactionError(#[source] sqlx::Error),
    #[error("データベース処理実行中にエラーが発生しました。")]
    SpecificOperationError(#[source] sqlx::Error),
    #[error("No rows affected: {0}")]
    NoRowsAffectedError(String),
    #[error("{0}")]
    KeyValueStoreError(#[from] redis::RedisError),
    #[error("{0}")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("{0}")]
    ConvertToUuidError(#[from] uuid::Error),
    #[error("ログインに失敗しました。")]
    UnauthenticatedError,
    #[error("認可情報が間違っています。")]
    UnauthorizedError,
    #[error("許可されていない操作です。")]
    ForbiddenOperation,
    #[error("{0}")]
    ConversionEntityError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        use AppError::*;

        let status_code = match self {
            UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            EntityNotFound(_) => StatusCode::NOT_FOUND,
            ValidationError(_) | ConvertToUuidError(_) => StatusCode::BAD_REQUEST,
            UnauthenticatedError | ForbiddenOperation => StatusCode::FORBIDDEN,
            UnauthorizedError => StatusCode::UNAUTHORIZED,
            e @ (TransactionError(_)
            | SpecificOperationError(_)
            | NoRowsAffectedError(_)
            | KeyValueStoreError(_)
            | BcryptError(_)
            | ConversionEntityError(_)) => {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "Unexpected error happened"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        status_code.into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
