use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to acquire lock for mutex")]
    LockMutex,
    #[error("Failed to generate random String: {0:?}")]
    RandomStringGenerate(randomizer::Error),
    #[error("Tcp listener failed to bind: {0}")]
    TcpListenerBind(std::io::Error),
    #[error("Failed to start web server: {0}")]
    WebServerStart(std::io::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Self::LockMutex => StatusCode::INTERNAL_SERVER_ERROR,
            Self::RandomStringGenerate(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::TcpListenerBind(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::WebServerStart(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
