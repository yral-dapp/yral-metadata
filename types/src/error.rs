use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[non_exhaustive]
pub enum ApiError {
    Cloudflare,
    Unknown(String)
}