use std::{str::FromStr, sync::Arc};

use api::route::{auth, v1};
use axum::{http::request::Builder, Router};
use kernel::{
    model::{
        auth::AccessToken,
        user::{User, UserEmail, UserId, UserName, UserRole},
    },
    repository::{auth::MockAuthRepository, user::MockUserRepository},
};
use registry::MockAppRegistryExt;
use rstest::fixture;
use uuid::Uuid;

pub fn v1(endpoint: &str) -> String {
    format!("/api/v1{}", endpoint)
}

pub fn make_router(registry: MockAppRegistryExt) -> Router {
    Router::new()
        .merge(v1::routes())
        .merge(auth::build_auth_routers())
        .with_state(Arc::new(registry))
}

#[fixture]
pub fn fixture_registry() -> MockAppRegistryExt {
    MockAppRegistryExt::new()
}

#[fixture]
pub fn fixture_auth(mut fixture_registry: MockAppRegistryExt) -> MockAppRegistryExt {
    fixture_registry.expect_auth_repository().returning(|| {
        let mut mock_auth_repository = MockAuthRepository::new();
        mock_auth_repository
            .expect_fetch_user_id_from_token()
            .returning(|_| Ok(Some(UserId::new(Uuid::new_v4()))));
        mock_auth_repository
            .expect_verify_user()
            .returning(|_, _| Ok(UserId::new(Uuid::new_v4())));
        mock_auth_repository
            .expect_create_token()
            .returning(|_| Ok(AccessToken::new("dummy".to_string())));
        Arc::new(mock_auth_repository)
    });
    fixture_registry
}

#[fixture]
pub fn fixture(mut fixture_auth: MockAppRegistryExt) -> MockAppRegistryExt {
    fixture_auth.expect_user_repository().returning(|| {
        let mut mock_user_repository = MockUserRepository::new();
        mock_user_repository
            .expect_find_current_user()
            .returning(|id| {
                Ok(Some(User::new(
                    id.clone(),
                    UserName::new("dummy-user".to_string()),
                    UserRole::User,
                    UserEmail::from_str("dummy@example.com").unwrap(),
                )))
            });
        Arc::new(mock_user_repository)
    });
    fixture_auth
}

pub trait TestRequestExt {
    fn bearer(self) -> Builder;
    #[allow(dead_code)]

    fn application_json(self) -> Builder;
}

impl TestRequestExt for Builder {
    fn bearer(self) -> Builder {
        self.header("Authorization", "Bearer dummy")
    }

    fn application_json(self) -> Builder {
        self.header("Content-Type", "application/json")
    }
}

// to_bytesなどを使って関数やトレイトに切り出してしまってもよいのだが、
// ライフタイムの解決がうまくいかないケースがある。そのため、マクロとして
// 生成させた方が楽なため、マクロを使用している。
#[macro_export]
macro_rules! deserialize_json {
    ($res:expr, $target:ty) => {{
        use tokio_stream::StreamExt;

        let mut bytes = Vec::new();
        let body = $res.into_body();
        let mut stream = body.into_data_stream();
        while let Ok(Some(chunk)) = stream.try_next().await {
            bytes.extend_from_slice(&chunk[..]);
        }
        let body: $target = serde_json::from_slice(&bytes)?;
        body
    }};
}
