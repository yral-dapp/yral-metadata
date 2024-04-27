use std::{collections::HashSet, env};

use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::consts::CLAIMS;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    pub sub: String,
    pub company: String,
}

pub fn verify_token(token: &str) -> Result<bool, jsonwebtoken::errors::Error> {
    let public_key = env::var("JWT_PUBLIC_KEY").expect("JWT_PUBLIC_KEY not set");

    let decoding_key = DecodingKey::from_ed_pem(public_key.as_bytes())?;
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.required_spec_claims = HashSet::new();
    validation.validate_exp = false;

    let token_message = decode::<Claims>(&token, &decoding_key, &validation);

    match token_message {
        Ok(token_data) => Ok(token_data.claims == *CLAIMS),
        Err(e) => Err(e),
    }
}
