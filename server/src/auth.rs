use std::{collections::HashSet, env};

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::{consts::CLAIMS, Error};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub sub: String,
    pub company: String,
    pub exp: u64,
}

#[derive(Clone)]
pub struct JwtDetails {
    pub decoding_key: DecodingKey,
    pub validation: Validation,
}

pub fn init_jwt() -> JwtDetails {
    let decoding_key =
        DecodingKey::from_ed_pem(JWT_PUBLIC_KEY.as_bytes()).expect("failed to create decoding key");

    let mut validation = Validation::new(Algorithm::EdDSA);

    JwtDetails {
        decoding_key,
        validation,
    }
}

pub fn verify_token(token: &str, jwt_details: &JwtDetails) -> Result<(), Error> {
    let JwtDetails {
        decoding_key,
        validation,
    } = jwt_details;

    let token_message = decode::<Claims>(&token, decoding_key, validation).map_err(Error::Jwt)?;

    if token_message.claims != *CLAIMS {
        return Err(Error::AuthTokenInvalid);
    }

    Ok(())
}
