use thiserror::Error;
use types::error::ApiError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    Api(#[from] ApiError),
    #[error("failed to sign: {0}")]
    Identity(#[from] yral_identity::Error),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
