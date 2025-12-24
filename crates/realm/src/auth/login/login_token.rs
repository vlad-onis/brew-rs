use std::collections::BTreeMap;

use chrono::{Days, Utc};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::{Sha256, digest::InvalidLength};
use thiserror::Error;

const TOKEN_EXPIRATION_PERIOD: u64 = 15;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to create Hmac: {0}")]
    Hmac(InvalidLength),

    #[error("Failed to sign token: {0}")]
    SignToken(jwt::Error),

    #[error("Failed to verify the token: {0}")]
    VerifyToken(jwt::Error),

    #[error("Failed to generate expire date")]
    Expire,
}

/// generates a jwt token that include the user_id, inserted at time(iat) and expiration date (exp)
pub fn generate_token(user_id: i64) -> Result<String, Error> {
    // todo: get secret from env file
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"some-secret").map_err(Error::Hmac)?;

    let iat = Utc::now();
    let exp = Utc::now()
        .checked_add_days(Days::new(TOKEN_EXPIRATION_PERIOD))
        .ok_or(Error::Expire)?;

    let mut claims = BTreeMap::new();
    claims.insert("sub", user_id.to_string());
    claims.insert("iat", iat.to_string());
    claims.insert("exp", exp.to_string());

    let token = claims.sign_with_key(&key).map_err(Error::SignToken)?;

    Ok(token)
}
