pub mod error;
pub mod habit;
pub mod handler;
pub mod route;
pub mod service;
pub mod storage;

use storage::SharedState;

use axum::Router;
use tower_http::trace::TraceLayer;

pub fn app(shared_state: SharedState) -> Router {
    route::router()
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state)
}
