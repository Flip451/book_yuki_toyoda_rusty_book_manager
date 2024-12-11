use crate::handler;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use registry::AppRegistry;

pub fn build_book_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/checkouts", get(handler::checkout::show_checked_out_list))
        .route("/", post(handler::book::register_book))
        .route("/", get(handler::book::show_book_list))
        .route("/:book_id", get(handler::book::show_book))
        .route("/:book_id", put(handler::book::update_book))
        .route("/:book_id", delete(handler::book::delete_book))
        .route(
            "/:book_id/checkouts",
            post(handler::checkout::checkout_book),
        )
        .route(
            "/:book_id/checkouts/:checkout_id/returned",
            put(handler::checkout::return_book),
        )
        .route(
            "/:book_id/checkout-history",
            get(handler::checkout::checkout_history),
        );
    Router::new().nest("/books", routers)
}
