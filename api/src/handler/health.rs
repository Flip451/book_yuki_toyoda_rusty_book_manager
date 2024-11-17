use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    let result = registry.health_check_repository().check_db().await;
    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
