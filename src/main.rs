use axum::{
    Json, Router,
    routing::{get, post},
};
use clap::Parser;
use log::info;
use serde::{Deserialize, Serialize};

mod error;
pub use error::Error;

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

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/shorten", post(create_url));

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

async fn create_url(Json(payload): Json<CreateRequest>) -> Json<CreateRequestResponse> {
    Json(CreateRequestResponse {
        short_url: payload.url,
    })
}

async fn health_handler() -> &'static str {
    "Healthy"
}
