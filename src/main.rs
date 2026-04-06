use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/shorten", post(create_url));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to tcp bind on 0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to server http server")
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
