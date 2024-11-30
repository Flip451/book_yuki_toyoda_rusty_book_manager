use axum::{routing::post, Router};
use registry::AppRegistry;

use crate::handler;

pub fn build_auth_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/login", post(handler::auth::login))
        .route("/logout", post(handler::auth::logout));
    Router::new().nest("/auth", routers)
}
