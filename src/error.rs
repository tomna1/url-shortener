#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to generate random String: {0:?}")]
    RandomStringGenerate(randomizer::Error),
    #[error("Tcp listener failed to bind: {0}")]
    TcpListenerBind(std::io::Error),
    #[error("Failed to start web server: {0}")]
    WebServerStart(std::io::Error),
}
