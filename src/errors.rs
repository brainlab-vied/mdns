use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Dns(#[from] dns_parser::Error),
    #[error("{0}")]
    TimeoutError(#[from] async_std::future::TimeoutError),
}
