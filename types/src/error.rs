use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize, Error, Debug)]
#[non_exhaustive]
pub enum ApiError {
    #[error("internal error: cloudflare")]
    Cloudflare,
    #[error("invalid signature provided")]
    InvalidSignature,
    #[error("internal error: redis")]
    Redis,
    #[error("internal error: deser")]
    Deser,
    #[error("unknown: {0}")]
    Unknown(String),
}
