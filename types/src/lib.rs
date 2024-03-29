pub mod error;
use candid::Principal;
use error::ApiError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserMetadata {
    pub user_canister_id: Principal,
    pub user_name: String,
}

pub type ApiResult<T> = Result<T, ApiError>;

pub type SetUserMetadataRes = ();
