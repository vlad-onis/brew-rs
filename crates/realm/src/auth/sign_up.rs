use thiserror::Error;

use brew_types::auth::{common::Email, sign_up::SignUpParams};

#[derive(Error, Debug)]
pub enum Error {
    // todo: don't log this is PII
    #[error("Email already exists")]
    EmailAlreadyExists(String),
}

/// Check if a user with the given email exists in the database
/// Returns false if the user does not exist, true if the user exists
pub async fn check_email_already_exists(_email: &Email) -> Result<bool, Error> {
    Ok(false)
}

/// Sign up a new user with the given email and password
/// Returns an error if the email is already in use
pub async fn sign_up_handler(params: SignUpParams) -> Result<(), Error> {
    let email = params.email;
    if check_email_already_exists(&email).await? {
        return Err(Error::EmailAlreadyExists(String::from(&email)));
    } else {
        return Ok(());
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::common::types::Email;

    #[tokio::test]
    pub async fn test_check_email_already_exists() {
        let email = Email::new("test@example.com");
        assert!(!check_email_already_exists(&email).await.unwrap());
    }
}
