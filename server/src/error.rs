use ntex::{
    http::{header, StatusCode},
    web,
};
use thiserror::Error;
use types::{error::ApiError, ApiResult};

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("failed to load config {0}")]
    Config(#[from] config::ConfigError),
    #[error("{0}")]
    Cloudflare(#[from] gob_cloudflare::Error),
    #[error("{0}")]
    Identity(#[from] yral_identity::Error),
}

impl From<&Error> for ApiResult<()> {
    fn from(value: &Error) -> Self {
        let err = match value {
            Error::IO(_) | Error::Config(_) => {
                log::warn!("internal error {value}");
                ApiError::Unknown("internal error, reported".into())
            }
            Error::Identity(_) => ApiError::InvalidSignature,
            Error::Cloudflare(e) => {
                log::warn!("cloudflare error {e}");
                ApiError::Cloudflare
            }
        };
        ApiResult::Err(err)
    }
}

impl web::error::WebResponseError for Error {
    fn error_response(&self, _: &web::HttpRequest) -> web::HttpResponse {
        let api_error = ApiResult::from(self);
        web::HttpResponse::build(self.status_code())
            .set_header(header::CONTENT_TYPE, "application/json")
            .json(&api_error)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Error::IO(_) | Error::Config(_) | Error::Cloudflare(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Error::Identity(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
