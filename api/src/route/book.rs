use crate::handler;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use registry::AppRegistry;

pub fn build_book_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", post(handler::book::register_book))
        .route("/", get(handler::book::show_book_list))
        .route("/:book_id", get(handler::book::show_book))
        .route("/:book_id", put(handler::book::update_book))
        .route("/:book_id", delete(handler::book::delete_book));
    Router::new().nest("/books", routers)
}
