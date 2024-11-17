use crate::handler;
use axum::{routing::get, Router};
use registry::AppRegistry;

pub fn build_health_check_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", get(handler::health::health_check))
        .route("/db", get(handler::health::health_check_db));
    Router::new().nest("/health", routers)
}
