use argon2::{
    Argon2,
    password_hash::{Error as HashingError, PasswordHasher, SaltString, rand_core::OsRng},
};
use thiserror::Error;
use tracing::debug;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to hash password because: {0}")]
    Hashing(HashingError),

    #[error("The hashed password is missing the hash: {0}")]
    MissingHash(String),
}

use brew_types::auth::common::Password;

pub fn hash_password(password: Password) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(Error::Hashing)?
        .to_string();

    debug!("Hashed password: {:?}", password_hash);

    Ok(password_hash)
}
