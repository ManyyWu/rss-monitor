use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("system error")]
    System,
    #[error("IO error {0}")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;