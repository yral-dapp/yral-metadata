use std::env;

use candid::Principal;
use ntex::web::{
    self,
    error::ErrorUnauthorized,
    types::{Json, Path, State},
};
use redis::AsyncCommands;
use types::{
    ApiResult, BulkUsers, GetUserMetadataRes, SetUserMetadataReq, SetUserMetadataRes, UserMetadata,
};

use crate::{state::AppState, Error, Result};

const METADATA_FIELD: &str = "metadata";

#[web::post("/metadata/{user_principal}")]
async fn set_user_metadata(
    state: State<AppState>,
    user_principal: Path<Principal>,
    req: Json<SetUserMetadataReq>,
) -> Result<Json<ApiResult<SetUserMetadataRes>>> {
    let signature = req.0.signature;
    let metadata = req.0.metadata;
    signature.verify_identity(*user_principal.as_ref(), metadata.clone().into())?;

    let user = user_principal.to_text();
    let mut conn = state.redis.get().await?;
    let meta_raw = serde_json::to_vec(&metadata).expect("failed to serialize user metadata?!");
    let _replaced: bool = conn.hset(user, METADATA_FIELD, &meta_raw).await?;

    Ok(Json(Ok(())))
}

#[web::get("/metadata/{user_principal}")]
async fn get_user_metadata(
    state: State<AppState>,
    path: Path<Principal>,
) -> Result<Json<ApiResult<GetUserMetadataRes>>> {
    let user = path.to_text();

    let mut conn = state.redis.get().await?;
    let meta_raw: Option<Box<[u8]>> = conn.hget(&user, METADATA_FIELD).await?;
    let Some(meta_raw) = meta_raw else {
        return Ok(Json(Ok(None)));
    };
    let meta: UserMetadata = serde_json::from_slice(&meta_raw).map_err(Error::Deser)?;

    Ok(Json(Ok(Some(meta))))
}

#[web::delete("/metadata/bulk")]
async fn delete_metadata_bulk(
    state: State<AppState>,
    req: Json<BulkUsers>,
    http_req: web::HttpRequest,
) -> Result<Json<ApiResult<()>>> {
    // authorize the request
    let req_auth_token = http_req
        .headers()
        .get("Authorization")
        .ok_or(Error::AuthToken("No Authorization header".to_string()))?;
    let auth_token = env::var("OFF_CHAIN_AGENT_TOKEN")
        .map_err(|_| Error::AuthToken("OFF_CHAIN_AGENT_TOKEN not set".to_string()))?;
    if *req_auth_token != format!("Bearer {}", auth_token) {
        return Err(Error::AuthToken("Invalid Authorization header".to_string()));
    }

    let keys = req.users.iter().map(|p| p.to_text()).collect::<Vec<_>>();

    let mut conn = state.redis.get().await?;
    match conn.del::<Vec<String>, bool>(keys).await {
        Ok(bool) => (),
        Err(e) => return Err(Error::Redis(e).into()),
    };

    Ok(Json(Ok(())))
}
