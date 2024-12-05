use axum::{
    routing::{delete, get, post, put},
    Router,
};
use registry::AppRegistry;

use crate::handler;

pub fn build_user_routers() -> Router<AppRegistry> {
    let routers = Router::new()
        .route("/", post(handler::user::register_user))
        .route("/", get(handler::user::list_users))
        .route("/:user_id", delete(handler::user::delete_user))
        .route("/:user_id/role", put(handler::user::change_user_role))
        .route("/me", get(handler::user::get_current_user))
        .route("/me/password", put(handler::user::change_password));
    Router::new().nest("/users", routers)
}
