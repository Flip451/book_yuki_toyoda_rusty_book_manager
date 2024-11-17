use crate::handler;
use axum::{
    routing::{get, post},
    Router,
};
use registry::AppRegistry;

pub fn build_book_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", post(handler::book::register_book))
        .route("/", get(handler::book::show_book_list))
        .route("/:book_id", get(handler::book::show_book));
    Router::new().nest("/books", routers)
}
