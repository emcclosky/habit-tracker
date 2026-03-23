use axum::{
    Router,
    routing::{get, post},
};

use crate::handler;
use crate::storage::SharedState;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/habits", get(handler::get_habits).post(handler::add_habit))
        .route("/habits/{name}/completions", post(handler::complete_habit))
}
