use std::env;

use candid::Principal;
use futures::prelude::*;
use ntex::web::{
    self,
    error::ErrorUnauthorized,
    types::{Json, Path, State},
};
use redis::{AsyncCommands, RedisError};
use types::{
    ApiResult, BulkUsers, GetUserMetadataRes, SetUserMetadataReq, SetUserMetadataRes, UserMetadata,
};

use crate::{auth::verify_token, state::AppState, Error, Result};

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
    // verify token
    let token = http_req
        .headers()
        .get("Authorization")
        .ok_or(Error::AuthToken("missing Authorization header".to_string()))?
        .to_str()
        .map_err(|_| Error::AuthToken("invalid Authorization header".to_string()))?;
    let token = token.trim_start_matches("Bearer ");
    match verify_token(token) {
        Ok(true) => (),
        Ok(false) => return Err(Error::AuthToken("invalid token".to_string())),
        Err(e) => return Err(Error::AuthToken(e.to_string())),
    }

    let keys = req.users.iter().map(|p| p.to_text()).collect::<Vec<_>>();

    let mut conn = state.redis.get().await?;

    let futures_iter = keys.iter().map(|key| async {
        let res = conn
            .clone()
            .hdel::<_, _, bool>(key.to_string(), METADATA_FIELD)
            .await;
        match res {
            Ok(_) => Ok(()),
            Err(e) => Err((key.to_string(), e.to_string())),
        }
    });
    let stream = futures::stream::iter(futures_iter)
        .boxed()
        .buffer_unordered(100);
    let results = stream.collect::<Vec<Result<_, (String, String)>>>().await;
    let errors = results
        .into_iter()
        .filter_map(|res| match res {
            Ok(_) => None,
            Err((key, e)) => Some((key, e)),
        })
        .collect::<Vec<_>>();
    if !errors.is_empty() {
        log::error!("Failed to delete metadata for some users: {:?}", errors);
    }

    Ok(Json(Ok(())))
}
