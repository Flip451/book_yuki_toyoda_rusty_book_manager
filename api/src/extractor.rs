use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use kernel::{
    model::{
        auth::{AccessToken, AccessTokenError},
        user::{User, UserId, UserRole},
    },
    repository::{auth::AuthRepositoryError, user::UserRepositoryError},
};
use registry::AppRegistry;

pub struct AuthorizedUser {
    pub access_token: AccessToken,
    pub user: User,
}

impl AuthorizedUser {
    pub fn user_id(&self) -> &UserId {
        self.user.user_id()
    }

    pub fn is_admin(&self) -> bool {
        self.user.role() == &UserRole::Admin
    }
}

#[async_trait]
impl FromRequestParts<AppRegistry> for AuthorizedUser {
    type Rejection = AuthorizedUserError;

    async fn from_request_parts(
        parts: &mut Parts,
        registry: &AppRegistry,
    ) -> Result<Self, Self::Rejection> {
        // HTTP ヘッダーからアクセストークンを取得
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthorizedUserError::Unauthenticated)?;

        let access_token: AccessToken = bearer.token().to_string().try_into()?;

        // アクセストークンに紐づくユーザーIDを取得
        let user_id = registry
            .auth_repository()
            .fetch_user_id_from_token(&access_token)
            .await?
            .ok_or(AuthorizedUserError::Unauthenticated)?;

        // ユーザーIDに紐づくユーザーを取得
        let user = registry
            .user_repository()
            .find_current_user(&user_id)
            .await?
            .ok_or(AuthorizedUserError::Unauthenticated)?;

        Ok(AuthorizedUser { access_token, user })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuthorizedUserError {
    #[error("unauthorized")]
    Unauthorized,

    #[error("unauthenticated")]
    Unauthenticated,

    #[error("invalid access token")]
    InvalidAccessToken(#[from] AccessTokenError),

    #[error("auth repository error")]
    AuthRepositoryError(#[from] AuthRepositoryError),

    #[error("user repository error")]
    UserRepositoryError(#[from] UserRepositoryError),
}

impl IntoResponse for AuthorizedUserError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::Unauthorized => StatusCode::FORBIDDEN,
            Self::Unauthenticated => StatusCode::UNAUTHORIZED,
            Self::InvalidAccessToken(_) => StatusCode::UNAUTHORIZED,
            e @ (Self::AuthRepositoryError(_) | Self::UserRepositoryError(_)) => {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "unexpected error happened"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        status.into_response()
    }
}
