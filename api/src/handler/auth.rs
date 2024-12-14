use std::str::FromStr;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use kernel::{
    model::{
        auth::event::CreateToken,
        user::{Password, PasswordError, UserEmail, UserEmailError},
        value_object::ValueObject,
    },
    repository::auth::AuthRepositoryError,
};
use registry::AppRegistry;
use thiserror::Error;

use crate::{
    extractor::AuthorizedUser,
    model::auth::{AccessTokenResponse, LoginRequest},
};

pub(crate) async fn login(
    State(registry): State<AppRegistry>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AccessTokenResponse>, AuthHandlerError> {
    let email = UserEmail::from_str(&req.email).map_err(AuthHandlerError::from)?;
    let password = Password::try_from(req.password).map_err(AuthHandlerError::from)?;

    let user_id = registry
        .auth_repository()
        .verify_user(&email, &password)
        .await?;

    let create_token = CreateToken::new(user_id.clone());
    let access_token = registry
        .auth_repository()
        .create_token(create_token)
        .await?;

    Ok(Json(AccessTokenResponse {
        access_token: access_token.into_inner(),
        user_id: user_id.into_inner(),
    }))
}

pub(crate) async fn logout(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> Result<StatusCode, AuthHandlerError> {
    registry
        .auth_repository()
        .delete_token(&user.access_token)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Error)]
pub enum AuthHandlerError {
    #[error("auth repository error: {0}")]
    AuthRepositoryError(#[from] AuthRepositoryError),

    #[error("invalid email: {0}")]
    InvalidEmail(#[from] UserEmailError),

    #[error("invalid password: {0}")]
    InvalidPassword(#[from] PasswordError),
}

impl IntoResponse for AuthHandlerError {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            AuthHandlerError::InvalidEmail(_email_error) => StatusCode::BAD_REQUEST,
            AuthHandlerError::InvalidPassword(_password_error) => StatusCode::BAD_REQUEST,
            AuthHandlerError::AuthRepositoryError(AuthRepositoryError::InvalidPassword) => {
                StatusCode::UNAUTHORIZED
            }
            AuthHandlerError::AuthRepositoryError(_) => {
                tracing::error!(
                    error.cause_chain = ?self,
                    error.message = %self,
                    "unexpected error happened"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        status_code.into_response()
    }
}
