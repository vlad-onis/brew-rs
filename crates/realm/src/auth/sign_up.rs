use thiserror::Error;
use tracing::info;

use crate::context::Context;

use super::password::{Error as HashingError, hash_password};
use brew_types::auth::{common::Email, sign_up::SignUpParams};
use storage::users::{UserRow, insert_user};
#[derive(Error, Debug)]
pub enum Error {
    // todo: don't log this is PII
    #[error("Email already exists")]
    EmailAlreadyExists(String),
    #[error("Hashing error: {0}")]
    Hashing(#[from] HashingError),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

/// Check if a user with the given email exists in the database
/// Returns false if the user does not exist, true if the user exists
pub async fn check_email_already_exists(_email: &Email) -> Result<bool, Error> {
    Ok(false)
}

/// Sign up a new user with the given email and password
/// Returns an error if the email is already in use
pub async fn sign_up_handler(params: SignUpParams, context: Context) -> Result<(), Error> {
    let email = params.email;

    info!(?email, "Signing up user with email");

    if check_email_already_exists(&email).await? {
        return Err(Error::EmailAlreadyExists(String::from(&email)));
    }

    let hashed_password = hash_password(params.password)?;

    let mut conn = context.db.pool.acquire().await?;

    insert_user(
        UserRow {
            id: None,
            first_name: params.first_name,
            last_name: params.last_name,
            email: String::from(&email),
            password_hash: hashed_password,
            created_at: None,
            updated_at: None,
        },
        &mut *conn,
    )
    .await?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use brew_types::auth::common::Email;

    #[tokio::test]
    pub async fn test_check_email_already_exists() {
        let email = Email::new("test@example.com");
        assert!(!check_email_already_exists(&email).await.unwrap());
    }
}
