use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use api::storage::Storage;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let storage = Storage::new(PathBuf::from("habits.json"));
    let shared_state = Arc::new(RwLock::new(storage));

    let app = api::app(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("failed to bind to port");

    println!(
        "listening on {}",
        listener.local_addr().expect("failed to get local address")
    );
    axum::serve(listener, app).await.expect("server error");
}
