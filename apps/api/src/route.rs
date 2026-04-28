use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::handler;
use crate::storage::SharedState;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/habits", get(handler::get_habits).post(handler::add_habit))
        .route("/habits/{name}", delete(handler::delete_habit))
        .route(
            "/habits/{name}/completions/{date}",
            post(handler::complete_habit).delete(handler::delete_completion),
        )
}
