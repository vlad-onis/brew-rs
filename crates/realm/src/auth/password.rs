use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{Error as HashingError, PasswordHasher, SaltString, rand_core::OsRng},
};
use thiserror::Error;
use tracing::debug;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Hashing failed: {0}")]
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

pub fn verify_password(
    input_password: Password,
    existing_password_hash: String,
) -> Result<(), Error> {
    let existing_hash_in_argon2_format =
        PasswordHash::new(&existing_password_hash).map_err(Error::Hashing)?;

    Argon2::default()
        .verify_password(input_password.as_bytes(), &existing_hash_in_argon2_format)
        .map_err(Error::Hashing)
}
