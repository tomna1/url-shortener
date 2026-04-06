use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use clap::Parser;
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

mod error;
pub use error::Error;

mod url_store;
use url_store::UrlStore;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(long)]
    http_bind_host: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let args = Args::parse();

    let url_store = Arc::new(Mutex::new(UrlStore::new()));

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/shorten", post(create_url))
        .with_state(url_store);

    let listener = tokio::net::TcpListener::bind(&args.http_bind_host)
        .await
        .map_err(|e| Error::TcpListenerBind(e))?;

    info!("Starting web server listening on {}", args.http_bind_host);
    axum::serve(listener, app)
        .await
        .map_err(|e| Error::WebServerStart(e))?;
    Ok(())
}

#[derive(Deserialize)]
struct CreateRequest {
    url: String,
}

#[derive(Serialize)]
struct CreateRequestResponse {
    short_url: String,
}

async fn create_url(
    State(url_store): State<Arc<Mutex<UrlStore>>>,
    Json(payload): Json<CreateRequest>,
) -> Json<CreateRequestResponse> {
    info!("Tryinh to create url");
    let mut store = url_store.lock().expect("Failed to get lock for url_store");
    let short = store
        .store_url(&payload.url)
        .expect("Failed to generate short url");

    Json(CreateRequestResponse { short_url: short })
}

async fn health_handler() -> &'static str {
    "Healthy"
}
